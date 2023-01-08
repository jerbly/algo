#[derive(Debug)]
pub struct UnionFind {
    index: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            index: (0..size).collect(),
            size: vec![1; size],
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);
        if i == j {
            return;
        }
        if self.size[i] < self.size[j] {
            self.index[i] = j;
            self.size[j] += self.size[i];
        } else {
            self.index[j] = i;
            self.size[i] += self.size[j];
        }
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    fn root(&mut self, i: usize) -> usize {
        let mut idx = i;
        while idx != self.index[idx] {
            // make every other node in path point to its grandparent (thereby having path length)
            self.index[idx] = self.index[self.index[idx]];
            idx = self.index[idx];
        }
        idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut uf = UnionFind::new(10);
        uf.union(4, 3);
        uf.union(3, 8);
        uf.union(6, 5);
        uf.union(9, 4);
        uf.union(2, 1);
        assert!(uf.connected(8, 9));
        assert!(!uf.connected(5, 0));
        uf.union(5, 0);
        uf.union(7, 2);
        uf.union(6, 1);
        println!("{:?}", uf);
    }
}
