use crate::{error::AlgoError, union_find::UnionFind};
use rand::{thread_rng, Rng};
use statrs::statistics::Statistics;
use std::collections::HashSet;

/*

With dimension 3, initialize by unioning top with top row, bottom with bottom row.
Percolates if top connects with bottom.

   0
 / | \
 1 2 3
 4 5 6
 7 8 9
 \ | /
   10

*/
struct BlockedSites {
    set: HashSet<usize>,
    vec: Vec<usize>,
}

impl BlockedSites {
    fn new(n: usize) -> Self {
        Self {
            set: HashSet::from_iter(1..=n * n),
            vec: (1..=n * n).collect(),
        }
    }

    fn remove_random(&mut self) -> usize {
        let index = thread_rng().gen_range(0..self.vec.len());
        let elem = self.vec.swap_remove(index);
        self.set.remove(&elem);
        elem
    }
}

struct Percolation {
    dimension: usize,
    uf: UnionFind,
    top: usize,
    bottom: usize,
    blocked_sites: BlockedSites,
}

impl Percolation {
    // creates n-by-n grid, with all sites initially blocked
    fn new(n: usize) -> Result<Self, AlgoError> {
        if n == 0 {
            return Err(AlgoError::IllegalArgument);
        }
        let mut p = Percolation {
            dimension: n,
            uf: UnionFind::new(2 + n * n),
            top: 0,
            bottom: 1 + n * n,
            blocked_sites: BlockedSites::new(n),
        };
        // initialize top and bottom rows
        for i in 1..=n {
            p.uf.union(p.top, i);
            p.uf.union(p.bottom, n * (n - 1) + i);
        }
        Ok(p)
    }

    // opens the site (row, col) if it is not open already
    fn open(&mut self, row: usize, col: usize) -> Result<(), AlgoError> {
        let id = self.to_id(row, col)?;
        //        if self.blocked_sites.remove(&id) {
        // makes unions to adjacent open sites
        if row > 1 && self.is_open(row - 1, col)? {
            self.uf.union(id, self.to_id(row - 1, col)?);
        }
        if row < self.dimension && self.is_open(row + 1, col)? {
            self.uf.union(id, self.to_id(row + 1, col)?);
        }
        if col > 1 && self.is_open(row, col - 1)? {
            self.uf.union(id, self.to_id(row, col - 1)?);
        }
        if col < self.dimension && self.is_open(row, col + 1)? {
            self.uf.union(id, self.to_id(row, col + 1)?);
        }
        //        }
        Ok(())
    }

    // is the site (row, col) open?
    fn is_open(&self, row: usize, col: usize) -> Result<bool, AlgoError> {
        Ok(!self.blocked_sites.set.contains(&self.to_id(row, col)?))
    }

    // is the site (row, col) full (does this site connect to the top)
    fn is_full(&mut self, row: usize, col: usize) -> Result<bool, AlgoError> {
        if !self.is_open(row, col)? {
            Ok(false)
        } else {
            Ok(self.uf.connected(self.top, self.to_id(row, col)?))
        }
    }

    // returns the number of open sites
    fn number_of_open_sites(&self) -> usize {
        self.dimension * self.dimension - self.blocked_sites.set.len()
    }

    // does the system percolate?
    fn percolates(&mut self) -> bool {
        self.uf.connected(self.top, self.bottom)
    }

    fn to_id(&self, row: usize, col: usize) -> Result<usize, AlgoError> {
        let id = (row - 1) * self.dimension + col;
        if id > 1 + self.dimension * self.dimension {
            Err(AlgoError::IllegalArgument)
        } else {
            Ok(id)
        }
    }

    fn open_random(&mut self) -> Result<(), AlgoError> {
        if !self.blocked_sites.set.is_empty() {
            let x = self.blocked_sites.remove_random();
            let row = ((x - 1) / self.dimension) + 1;
            let col = x - (((x - 1) / self.dimension) * self.dimension);
            //println!("{x},{row},{col}");
            self.open(row, col)
        } else {
            Err(AlgoError::NoBlockedSites)
        }
    }

    fn print(&mut self) -> Result<(), AlgoError> {
        for row in 1..=self.dimension {
            for col in 1..=self.dimension {
                if !self.is_open(row, col)? {
                    print!("#");
                } else if self.is_full(row, col)? {
                    print!("o");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        Ok(())
    }
}

struct PercolationStats {
    results: Vec<f64>,
}

impl PercolationStats {
    // perform independent trials on an n-by-n grid
    fn new(n: usize, trials: usize) -> Result<Self, AlgoError> {
        let mut ps = PercolationStats {
            results: Vec::new(),
        };
        for _ in 0..trials {
            let mut p = Percolation::new(n)?;
            while !p.percolates() {
                p.open_random()?
            }
            ps.results
                .push(p.number_of_open_sites() as f64 / (n * n) as f64);
        }
        Ok(ps)
    }

    // sample mean of percolation threshold
    fn mean(&self) -> f64 {
        self.results[..].mean()
    }

    // sample standard deviation of percolation threshold
    fn stddev(&self) -> f64 {
        self.results[..].std_dev()
    }

    // low endpoint of 95% confidence interval
    fn confidence_lo(&self) -> f64 {
        let z95 = 1.96;
        self.mean() - (z95 * (self.stddev() / (self.results.len() as f64).sqrt()))
    }

    // high endpoint of 95% confidence interval
    fn confidence_hi(&self) -> f64 {
        let z95 = 1.96;
        self.mean() + (z95 * (self.stddev() / (self.results.len() as f64).sqrt()))
    }
}

pub fn monte_carlo() -> Result<(), AlgoError> {
    // Monte Carlo simulation. To estimate the percolation threshold, consider the following computational experiment:
    // Initialize all sites to be blocked.
    // Repeat the following until the system percolates:
    // - Choose a site uniformly at random among all blocked sites.
    // - Open the site.
    // The fraction of sites that are opened when the system percolates provides an estimate of the percolation threshold.

    let mut p = Percolation::new(20)?;

    while !p.percolates() {
        p.open_random()?;
    }

    p.print()?;
    println!("{}", p.number_of_open_sites() as f32 / (20.0 * 20.0));

    let ps = PercolationStats::new(200, 100)?;
    println!("mean                    = {}", ps.mean());
    println!("stddev                  = {}", ps.stddev());
    println!(
        "95% confidence interval = [{}, {}]",
        ps.confidence_lo(),
        ps.confidence_hi()
    );

    Ok(())
}
