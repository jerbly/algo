pub mod sap;

use std::collections::{hash_map::Keys, HashMap, HashSet};
use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::util::digraph::Digraph;

struct WordNet {
    // Map from noun to the set of synsets containing this noun
    nouns: HashMap<String, HashSet<usize>>,
    graph: Digraph,
}

impl WordNet {
    // constructor takes the name of the two input files
    pub fn new(synsets: &str, hypernyms: &str) -> anyhow::Result<Self> {
        let file = File::open(synsets)?;
        let lines = io::BufReader::new(file).lines();
        let mut nouns: HashMap<String, HashSet<usize>> = HashMap::new();
        let mut vertices = 0usize;
        for (i, row) in lines.flatten().enumerate() {
            let ns: Vec<&str> = row.split(',').collect::<Vec<&str>>()[1]
                .split_ascii_whitespace()
                .collect();

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

        let file = File::open(hypernyms)?;
        let lines = io::BufReader::new(file).lines();
        let mut dg = Digraph::new(vertices);
        for row in lines.flatten() {
            let nums: Vec<usize> = row
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            if nums.len() > 1 {
                for w in nums[1..].iter() {
                    dg.add_edge(nums[0], *w);
                }
            }
        }
        println!("{}", dg);
        Ok(Self { nouns, graph: dg })
    }

    // returns all WordNet nouns
    pub fn nouns(&self) -> Keys<'_, String, HashSet<usize>> {
        self.nouns.keys()
    }

    // is the word a WordNet noun?
    pub fn is_noun(&self, word: String) -> bool {
        self.nouns.contains_key(&word)
    }

    // distance between nounA and nounB (defined below)
    pub fn distance(&self, noun_a: String, noun_b: String) -> usize {
        todo!();
    }

    // a synset (second field of synsets.txt) that is the common ancestor of nounA and nounB
    // in a shortest ancestral path (defined below)
    pub fn sap(&self, noun_a: String, noun_b: String) -> String {
        todo!();
    }
}

#[test]
fn test() -> anyhow::Result<()> {
    let synsets = "/Users/jerbly/Documents/code/rust/algo/projects/wordnet/synsets100-subgraph.txt";
    let hypernyms =
        "/Users/jerbly/Documents/code/rust/algo/projects/wordnet/hypernyms100-subgraph.txt";
    let wn = WordNet::new(synsets, hypernyms)?;
    for k in wn.nouns() {
        println!("{:?}", k);
    }
    Ok(())
}
