use std::{cmp::Ordering, collections::BinaryHeap};

use super::board::Board;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    board: Board,
    priority: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on priority.
        // In case of a tie we compare boards - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.board.cmp(&other.board))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solver {
    solution: Vec<Board>,
}

impl Solver {
    // find a solution to the initial board (using the A* algorithm)
    fn new(initial: Board) -> Self {
        let mut solver = Solver { solution: vec![] };
        let priority = initial.manhattan() + solver.solution.len();
        let n = Node {
            board: initial,
            priority,
        };
        let mut heap = BinaryHeap::new();
        heap.push(n);

        while let Some(node) = heap.pop() {
            solver.solution.push(node.board.clone());
            if node.board.is_goal() {
                break;
            }
            for board in node.board.neighbors() {
                let priority = board.manhattan() + solver.solution.len();
                heap.push(Node { board, priority });
            }
        }

        solver
    }

    // is the initial board solvable? (see below)
    fn is_solvable(&self) -> bool {
        self.moves() > -1
    }

    // min number of moves to solve initial board; -1 if unsolvable
    fn moves(&self) -> isize {
        if self.solution.is_empty() {
            -1
        } else {
            self.solution.len().try_into().unwrap()
        }
    }

    // sequence of boards in a shortest solution; null if unsolvable
    fn solution(&self) -> Option<&Vec<Board>> {
        Some(&self.solution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let b = Board::new(vec![vec![0, 1, 3], vec![4, 2, 5], vec![7, 8, 6]]);
        let s = Solver::new(b);
        if let Some(boards) = s.solution() {
            for bs in boards {
                println!("{bs}");
            }
        }
    }
}
