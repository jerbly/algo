use std::{cmp::Ordering, collections::BinaryHeap};

use super::board::Board;

#[derive(Debug, Clone)]
struct BoardNode {
    board: Board,
    parent: Option<usize>, // index of the parent in the node_store
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    index: usize, // position of this node in the node_store
    priority: usize,
    move_num: usize,
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
            .then_with(|| self.index.cmp(&other.index))
            .then_with(|| self.move_num.cmp(&other.move_num))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solver {
    board_node_store: Vec<BoardNode>,
    solution: Vec<Board>,
}

impl Solver {
    // find a solution to the initial board (using the A* algorithm)
    fn new(initial: Board) -> Self {
        let mut solver = Solver {
            board_node_store: vec![],
            solution: vec![],
        };

        let priority = initial.manhattan();
        solver.board_node_store.push(BoardNode {
            board: initial,
            parent: None,
        });

        let mut heap = BinaryHeap::new();
        heap.push(Node {
            index: 0,
            priority,
            move_num: 0,
        });

        let mut end_node = Node {
            index: 0,
            priority: 0,
            move_num: 0,
        };
        while let Some(node) = heap.pop() {
            let board_node = &solver.board_node_store[node.index];
            let board_node_parent = board_node.parent;

            if board_node.board.is_goal() {
                end_node = node;
                break;
            }
            let move_num = node.move_num + 1;
            for neighbour in board_node.board.neighbors() {
                // critical optimization: don't add a neighbour matching the parent of the search node
                // This is essentially moving the same tile back where it came from!
                if solver.is_parent(board_node_parent, &neighbour) {
                    continue;
                }

                let index = solver.board_node_store.len();
                let priority = neighbour.manhattan() + move_num;

                solver.board_node_store.push(BoardNode {
                    board: neighbour,
                    parent: Some(node.index),
                });

                heap.push(Node {
                    index,
                    priority,
                    move_num,
                });
            }
        }
        solver.build_solution(end_node.index);
        solver
    }

    fn build_solution(&mut self, end_index: usize) {
        // walk back from node using parent index and clone into the solution vec
        let mut bn = &self.board_node_store[end_index];
        loop {
            self.solution.push(bn.board.clone());
            if let Some(parent) = bn.parent {
                bn = &self.board_node_store[parent];
            } else {
                break;
            }
        }
        self.solution.reverse();
    }

    fn is_parent(&self, parent: Option<usize>, neighbour: &Board) -> bool {
        match parent {
            Some(p) => {
                let bn = &self.board_node_store[p];
                bn.board == *neighbour
            }
            None => false,
        }
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
            self.solution.len() as isize - 1
        }
    }

    // sequence of boards in a shortest solution; null if unsolvable
    fn solution(&self) -> Option<&Vec<Board>> {
        if self.is_solvable() {
            Some(&self.solution)
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
        //        let b = Board::new(vec![vec![0, 1, 3], vec![4, 2, 5], vec![7, 8, 6]]);
        let b = Board::new(vec![vec![8, 1, 3], vec![4, 0, 2], vec![7, 6, 5]]);
        let s = Solver::new(b);
        println!("Minimum numnber of moves = {}\n", s.moves());
        if let Some(boards) = s.solution() {
            for bs in boards {
                println!("{bs}");
            }
        }
    }
}
