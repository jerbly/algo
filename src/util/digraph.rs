use anyhow::anyhow;
use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

#[derive(Debug, Clone)]
pub struct Digraph {
    vsize: usize,         // number of vertices in this digraph
    esize: usize,         // number of edges in this digraph
    adj: Vec<Vec<usize>>, // adj[v] = adjacency list for vertex v
    indegree: Vec<usize>, // indegree[v] = indegree of vertex v
}

impl Digraph {
    pub fn new(vsize: usize) -> Self {
        assert!(vsize > 0);

        Self {
            vsize,
            esize: 0,
            adj: {
                let mut v = Vec::new();
                for _ in 0..vsize {
                    v.push(Vec::new())
                }
                v
            },
            indegree: vec![0; vsize],
        }
    }

    /**
     * Initializes a digraph from the specified input stream.
     * The format is the number of vertices <em>V</em>,
     * followed by the number of edges <em>E</em>,
     * followed by <em>E</em> pairs of vertices, with each entry separated by whitespace.
     */
    pub fn from_lines(lines: Lines<BufReader<File>>) -> anyhow::Result<Self> {
        let mut flat = lines.flatten();
        let mut dg = Digraph::new(match flat.next() {
            Some(s) => s.parse::<usize>()?,
            None => 0,
        });
        // dg.esize = match flat.next() {
        //     Some(s) => s.parse::<usize>()?,
        //     None => return Err(anyhow!("failed to parse number of edges")),
        // };
        for line in flat {
            let nums: Vec<usize> = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            if nums.len() > 1 {
                dg.add_edge(nums[0], nums[1]);
            }
        }
        Ok(dg)
    }

    pub fn v(&self) -> usize {
        self.vsize
    }
    pub fn e(&self) -> usize {
        self.esize
    }

    pub fn validate_vertex(&self, v: usize) {
        assert!(v < self.vsize);
    }

    // Adds the directed edge v→w to this digraph.
    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.validate_vertex(v);
        self.validate_vertex(w);
        self.adj[v].push(w);
        self.indegree[w] = self.indegree[w] + 1;
        self.esize += 1;
    }

    /// Returns the vertices adjacent from vertex `v` in this digraph.
    pub fn adj(&self, v: usize) -> &Vec<usize> {
        self.validate_vertex(v);
        &self.adj[v]
    }

    /// Returns the number of directed edges incident from vertex `v`.
    /// This is known as the *outdegree* of vertex `v`.
    pub fn outdegree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    /// Returns the number of directed edges incident to vertex `v`.
    /// This is known as the *indegree* of vertex `v`.
    pub fn indegree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.indegree[v]
    }

    /// Returns the reverse of the digraph.
    pub fn reverse(&self) -> Self {
        let mut reverse = Self::new(self.v());
        for v in 0..self.v() {
            for w in self.adj(v) {
                reverse.add_edge(*w, v);
            }
        }
        reverse
    }
}

impl Display for Digraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = vec![];
        s.push(format!("{} vertices, {} edges\n", self.v(), self.e()));
        for v in 0..self.v() {
            s.push(format!("{v}: "));
            for w in self.adj(v) {
                s.push(format!("{w} "));
            }
            s.push("\n".to_owned());
        }

        write!(f, "{}", s.join(""))
    }
}

#[test]
fn test() -> anyhow::Result<()> {
    let file = File::open("/Users/jerbly/Documents/code/rust/algo/projects/wordnet/digraph25.txt")?;
    let lines = io::BufReader::new(file).lines();
    let g = Digraph::from_lines(lines)?;
    println!("{g}");
    let gr = g.reverse();
    println!("{gr}");
    Ok(())
}
