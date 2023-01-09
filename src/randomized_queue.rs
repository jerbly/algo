use rand::{thread_rng, Rng};

struct RandomizedQueue<T>(Vec<T>);

impl<T> Iterator for RandomizedQueue<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.dequeue()
    }
}

impl<T> RandomizedQueue<T> {
    // construct an empty randomized queue
    pub fn new() -> Self {
        RandomizedQueue(Vec::new())
    }

    // is the randomized queue empty?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // return the number of items on the randomized queue
    pub fn size(&self) -> usize {
        self.0.len()
    }

    // add the item
    pub fn enqueue(&mut self, item: T) {
        self.0.push(item)
    }

    // remove and return a random item
    pub fn dequeue(&mut self) -> Option<T> {
        if self.size() > 0 {
            let index = thread_rng().gen_range(0..self.0.len());
            Some(self.0.swap_remove(index))
        } else {
            None
        }
    }

    // return a random item (but do not remove it)
    pub fn sample(&self) -> Option<&T> {
        if self.size() > 0 {
            let index = thread_rng().gen_range(0..self.0.len());
            Some(&self.0[index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut r: RandomizedQueue<String> = RandomizedQueue::new();
        for x in ["A", "B", "C", "D", "E", "F", "G", "H", "I"] {
            r.enqueue(x.to_string());
        }
        assert_eq!(r.size(), 9);

        let s = r.sample();
        assert_ne!(s, None);
        assert_eq!(r.size(), 9);
        println!("{:?}", s);

        for s in &mut r {
            println!("{s}");
        }
        let empty = r.is_empty();
        assert!(empty);
    }
}
