use super::wn::WordNet;

pub struct Outcast {
    wordnet: WordNet,
}
impl Outcast {
    pub fn new(wordnet: WordNet) -> Self {
        Self { wordnet }
    }

    pub fn outcast(&self, nouns: &[String]) -> Option<String> {
        if nouns.iter().any(|word| !self.wordnet.is_noun(word)) {
            return None;
        }
        let mut max_distance = 0usize;
        let mut outcast_noun = &String::new();
        for noun in nouns {
            let mut sum_distance = 0usize;
            for other in nouns {
                if noun != other {
                    if let Some(d) = self.wordnet.distance(noun, other) {
                        sum_distance += d;
                    }
                }
            }
            if sum_distance > max_distance {
                max_distance = sum_distance;
                outcast_noun = noun;
            }
        }
        Some(outcast_noun.clone())
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
        let out = Outcast::new(wn);

        let o = out.outcast(&[
            "horse".to_string(),
            "zebra".to_string(),
            "cat".to_string(),
            "bear".to_string(),
            "table".to_string(),
        ]);
        assert_eq!(o, Some("table".to_string()));

        let o = out.outcast(&[
            "water".to_string(),
            "soda".to_string(),
            "bed".to_string(),
            "orange_juice".to_string(),
            "milk".to_string(),
            "apple_juice".to_string(),
            "tea".to_string(),
            "coffee".to_string(),
        ]);
        assert_eq!(o, Some("bed".to_string()));

        let o = out.outcast(&[
            "apple".to_string(),
            "pear".to_string(),
            "peach".to_string(),
            "banana".to_string(),
            "lime".to_string(),
            "lemon".to_string(),
            "blueberry".to_string(),
            "strawberry".to_string(),
            "mango".to_string(),
            "watermelon".to_string(),
            "potato".to_string(),
        ]);
        assert_eq!(o, Some("potato".to_string()));

        Ok(())
    }
}
