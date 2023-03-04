use std::collections::{hash_map::Keys, HashMap, HashSet};
use std::{
    fs::File,
    io::{self, BufRead},
};

use itertools::Itertools;

use crate::util::digraph::Digraph;

use super::sap::Sap;

pub struct WordNet {
    // Map from noun to the set of synsets containing this noun
    nouns: HashMap<String, HashSet<usize>>,
    synsets: Vec<String>,
    sap: Sap,
}

impl WordNet {
    // constructor takes the name of the two input files
    pub fn new(synsets_path: &str, hypernyms_path: &str) -> anyhow::Result<Self> {
        let file = File::open(synsets_path)?;
        let lines = io::BufReader::new(file).lines();
        let mut nouns: HashMap<String, HashSet<usize>> = HashMap::new();
        let mut vertices = 0usize;
        let mut synsets = vec![];
        for (i, row) in lines.flatten().enumerate() {
            let ss = row.split(',').collect::<Vec<&str>>()[1];
            synsets.push(ss.to_string());
            let ns: Vec<&str> = ss.split_ascii_whitespace().collect();

            for n in ns {
                nouns
                    .entry(n.to_string())
                    .and_modify(|set| {
                        set.insert(i);
                    })
                    .or_insert({
                        let mut set = HashSet::new();
                        set.insert(i);
                        set
                    });
            }
            vertices += 1;
        }

        let file = File::open(hypernyms_path)?;
        let lines = io::BufReader::new(file).lines();
        let mut graph = Digraph::new(vertices);
        for row in lines.flatten() {
            let nums: Vec<usize> = row
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            if nums.len() > 1 {
                for w in nums[1..].iter() {
                    graph.add_edge(nums[0], *w);
                }
            }
        }
        Ok(Self {
            nouns,
            synsets,
            sap: Sap::new(graph),
        })
    }

    // returns all WordNet nouns
    pub fn nouns(&self) -> Keys<'_, String, HashSet<usize>> {
        self.nouns.keys()
    }

    // is the word a WordNet noun?
    pub fn is_noun(&self, word: &String) -> bool {
        self.nouns.contains_key(word)
    }

    // distance between nounA and nounB (defined below)
    pub fn distance(&self, noun_a: &String, noun_b: &String) -> Option<usize> {
        if let Some(v_set) = self.nouns.get(noun_a) {
            if let Some(w_set) = self.nouns.get(noun_b) {
                self.sap.multi_length(
                    &v_set.iter().copied().collect_vec(),
                    &w_set.iter().copied().collect_vec(),
                )
            } else {
                None
            }
        } else {
            None
        }
    }

    // a synset (second field of synsets.txt) that is the common ancestor of nounA and nounB
    // in a shortest ancestral path (defined below)
    pub fn sap(&self, noun_a: String, noun_b: String) -> Option<&String> {
        if let Some(v_set) = self.nouns.get(&noun_a) {
            if let Some(w_set) = self.nouns.get(&noun_b) {
                if let Some(ancestor) = self.sap.multi_ancestor(
                    &v_set.iter().copied().collect_vec(),
                    &w_set.iter().copied().collect_vec(),
                ) {
                    Some(&self.synsets[ancestor])
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let synsets = "projects/wordnet/synsets.txt";
        let hypernyms = "projects/wordnet/hypernyms.txt";
        let wn = WordNet::new(synsets, hypernyms)?;

        let distance = wn.distance(&"worm".to_string(), &"bird".to_string());
        let sap = wn.sap("worm".to_string(), "bird".to_string());
        assert_eq!(distance, Some(5));
        assert_eq!(
            sap,
            Some(&"animal animate_being beast brute creature fauna".to_string())
        );

        let distance = wn.distance(&"white_marlin".to_string(), &"mileage".to_string());
        assert_eq!(distance, Some(23));

        let distance = wn.distance(&"Black_Plague".to_string(), &"black_marlin".to_string());
        assert_eq!(distance, Some(33));

        let distance = wn.distance(
            &"American_water_spaniel".to_string(),
            &"histology".to_string(),
        );
        assert_eq!(distance, Some(27));

        let distance = wn.distance(&"Brown_Swiss".to_string(), &"barrel_roll".to_string());
        assert_eq!(distance, Some(29));

        Ok(())
    }
}
