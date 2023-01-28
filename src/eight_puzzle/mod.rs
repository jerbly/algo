use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::eight_puzzle::{board::Board, solver::Solver};

pub mod board;
pub mod solver;

pub fn run_solver(filename: String) -> anyhow::Result<()> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mut tiles: Vec<Vec<usize>> = vec![];
    for row in lines.flatten() {
        let nums: Vec<usize> = row
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        if nums.len() > 1 {
            tiles.push(nums);
        }
    }
    let solver = Solver::new(Board::new(tiles));
    if !solver.is_solvable() {
        println!("No solution possible");
    } else {
        println!("Minimum numnber of moves = {}\n", solver.moves());
        if let Some(boards) = solver.solution() {
            for bs in boards {
                println!("{bs}");
            }
        }
    }
    Ok(())
}
