use std::collections::VecDeque;

use super::digraph::Digraph;

#[derive(Debug)]
pub struct BreadthFirstDirectedPaths {
    marked: Box<[bool]>,
    edge_to: Box<[usize]>,
    dist_to: Box<[usize]>,
}

impl BreadthFirstDirectedPaths {
    pub fn new(g: &Digraph, s: usize) -> Self {
        let mut this = Self {
            marked: vec![false; g.v()].into_boxed_slice(),
            edge_to: vec![0; g.v()].into_boxed_slice(),
            dist_to: vec![usize::MAX; g.v()].into_boxed_slice(),
        };
        this.validate_vertex(s);
        this.bfs(g, s);
        this
    }

    pub fn new_multi(g: &Digraph, sources: &[usize]) -> Self {
        let mut this = Self {
            marked: vec![false; g.v()].into_boxed_slice(),
            edge_to: vec![0; g.v()].into_boxed_slice(),
            dist_to: vec![usize::MAX; g.v()].into_boxed_slice(),
        };
        this.validate_vertices(sources);
        this.bfs_multi(g, sources);
        this
    }

    // BFS from single source
    fn bfs(&mut self, g: &Digraph, s: usize) {
        let mut q: VecDeque<usize> = VecDeque::new();
        self.marked[s] = true;
        self.dist_to[s] = 0;
        q.push_back(s);
        while let Some(v) = q.pop_front() {
            for w in g.adj(v) {
                if !self.marked[*w] {
                    self.edge_to[*w] = v;
                    self.dist_to[*w] = self.dist_to[v] + 1;
                    self.marked[*w] = true;
                    q.push_back(*w);
                }
            }
        }
    }

    fn bfs_multi(&mut self, g: &Digraph, sources: &[usize]) {
        let mut q: VecDeque<usize> = VecDeque::new();
        for s in sources {
            self.marked[*s] = true;
            self.dist_to[*s] = 0;
            q.push_back(*s);
        }
        while let Some(v) = q.pop_front() {
            for w in g.adj(v) {
                if !self.marked[*w] {
                    self.edge_to[*w] = v;
                    self.dist_to[*w] = self.dist_to[v] + 1;
                    self.marked[*w] = true;
                    q.push_back(*w);
                }
            }
        }
    }

    /// Is there a directed path from the source `s` (or sources) to vertex `v`?
    pub fn has_path_to(&self, v: usize) -> bool {
        self.validate_vertex(v);
        self.marked[v]
    }

    /// Returns the number of edges in a shortest path from the source `s`
    /// (or sources) to vertex `v`
    pub fn distance_to(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.dist_to[v]
    }

    /// Returns a shortest path from `s` (or sources) to `v`, or
    /// `None` if no such path.
    pub fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        self.validate_vertex(v);

        if !self.has_path_to(v) {
            return None;
        }
        let mut path = vec![];
        let mut x = v;
        while self.dist_to[x] != 0 {
            path.push(x);
            x = self.edge_to[x];
        }
        path.push(x);
        Some(path)
    }

    fn validate_vertex(&self, v: usize) {
        assert!(v < self.marked.len());
    }

    fn validate_vertices(&self, vertices: &[usize]) {
        assert!(vertices.len() > 0);
        for v in vertices {
            self.validate_vertex(*v);
        }
    }
}
