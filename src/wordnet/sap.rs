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
        todo!()
    }

    /// a common ancestor of v and w that participates in a shortest ancestral path; None if no such path
    pub fn ancestor(&self, v: usize, w: usize) -> Option<usize> {
        todo!()
    }

    /// length of shortest ancestral path between any vertex in v and any vertex in w; None if no such path
    pub fn multi_length(&self, v: Vec<usize>, w: Vec<usize>) -> Option<usize> {
        todo!()
    }

    /// a common ancestor that participates in shortest ancestral path; None if no such path
    pub fn multi_ancestor(&self, v: Vec<usize>, w: Vec<usize>) -> Option<usize> {
        todo!()
    }
}

// BFS impl should use HashMap<Vertex, Distance>
// Do a BFS with dist_to from V
// Do a BFS with dist_to from W
// Add the two dist_to arrays and find the shortest > 0
//   the index is the shortest common ancestor,
//   the value is the shortest ancestral path,
//   to print the path, combine the two path_to results

#[test]
fn test() -> anyhow::Result<()> {
    let file = File::open("/Users/jerbly/Documents/code/rust/algo/projects/wordnet/digraph25.txt")?;
    let lines = io::BufReader::new(file).lines();
    let g = Digraph::from_lines(lines)?;
    let bfs_a: BreadthFirstDirectedPaths = BreadthFirstDirectedPaths::new_multi(&g, &[13, 23, 24]);
    println!("{:?}", bfs_a);
    let bfs_b: BreadthFirstDirectedPaths = BreadthFirstDirectedPaths::new_multi(&g, &[6, 16, 17]);
    println!("{:?}", bfs_b);

    let mut common = 0usize;
    let mut length = usize::MAX;
    for v in 0..g.v() {
        let a = bfs_a.distance_to(v);
        let b = bfs_b.distance_to(v);
        if a > 0 && b > 0 && a < usize::MAX && b < usize::MAX {
            if a+b < length {
                length = a+b;
                common = v;
            }
        }
    }
    println!("common {common}, length {length}");

    //    println!("{:?}", bfs.path_to(3));
    Ok(())
}
