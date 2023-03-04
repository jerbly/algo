use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::util::{bfs_directed::BreadthFirstDirectedPaths, digraph::Digraph};

/// Shortest Ancestral Path
#[derive(Debug)]
pub struct Sap {
    g: Digraph,
}

impl Sap {
    /// constructor takes a digraph (not necessarily a DAG)
    pub fn new(g: Digraph) -> Self {
        Self { g }
    }

    /// length of shortest ancestral path between v and w; None if no such path
    pub fn length(&self, v: usize, w: usize) -> Option<usize> {
        self.multi_length(&[v], &[w])
    }

    /// a common ancestor of v and w that participates in a shortest ancestral path; None if no such path
    pub fn ancestor(&self, v: usize, w: usize) -> Option<usize> {
        self.multi_ancestor(&[v], &[w])
    }

    /// length of shortest ancestral path between any vertex in v and any vertex in w; None if no such path
    pub fn multi_length(&self, v: &[usize], w: &[usize]) -> Option<usize> {
        let bfs_a = BreadthFirstDirectedPaths::new_multi(&self.g, v);
        let bfs_b = BreadthFirstDirectedPaths::new_multi(&self.g, w);
        let (_, length) = self.shortest_values(bfs_a, bfs_b);
        length
    }

    /// a common ancestor that participates in shortest ancestral path; None if no such path
    pub fn multi_ancestor(&self, v: &[usize], w: &[usize]) -> Option<usize> {
        let bfs_a = BreadthFirstDirectedPaths::new_multi(&self.g, v);
        let bfs_b = BreadthFirstDirectedPaths::new_multi(&self.g, w);
        let (ancestor, _) = self.shortest_values(bfs_a, bfs_b);
        ancestor
    }

    // Do a BFS with dist_to from V
    // Do a BFS with dist_to from W
    // Add the two dist_to arrays and find the shortest > 0
    //   the index is the shortest common ancestor,
    //   the value is the shortest ancestral path,
    //   to print the path, combine the two path_to results
    fn shortest_values(
        &self,
        bfs_a: BreadthFirstDirectedPaths,
        bfs_b: BreadthFirstDirectedPaths,
    ) -> (Option<usize>, Option<usize>) {
        let mut common = 0usize;
        let mut length = usize::MAX;
        for v in 0..self.g.v() {
            let a = bfs_a.distance_to(v);
            let b = bfs_b.distance_to(v);
            if a > 0 && b > 0 && a < usize::MAX && b < usize::MAX {
                if a + b < length {
                    length = a + b;
                    common = v;
                }
            }
        }
        if length < usize::MAX {
            (Some(common), Some(length))
        } else {
            (None, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test25() -> anyhow::Result<()> {
        let file = File::open("projects/wordnet/digraph25.txt")?;
        let lines = io::BufReader::new(file).lines();
        let g = Digraph::from_lines(lines)?;
        let sap = Sap::new(g);
        let length = sap.multi_length(&[13, 23, 24], &[6, 16, 17]);
        let ancestor = sap.multi_ancestor(&[13, 23, 24], &[6, 16, 17]);
        assert_eq!(length, Some(4));
        assert_eq!(ancestor, Some(3));

        Ok(())
    }

    #[test]
    fn test1() -> anyhow::Result<()> {
        let file = File::open("projects/wordnet/digraph1.txt")?;
        let lines = io::BufReader::new(file).lines();
        let g = Digraph::from_lines(lines)?;
        let sap = Sap::new(g);
        let length = sap.length(3, 11);
        let ancestor = sap.ancestor(3, 11);
        assert_eq!(length, Some(4));
        assert_eq!(ancestor, Some(1));

        let length = sap.length(9, 12);
        let ancestor = sap.ancestor(9, 12);
        assert_eq!(length, Some(3));
        assert_eq!(ancestor, Some(5));

        let length = sap.length(7, 2);
        let ancestor = sap.ancestor(7, 2);
        assert_eq!(length, Some(4));
        assert_eq!(ancestor, Some(0));

        let length = sap.length(1, 6);
        let ancestor = sap.ancestor(1, 6);
        assert_eq!(length, None);
        assert_eq!(ancestor, None);

        Ok(())
    }
}
