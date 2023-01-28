use std::{
    collections::HashSet,
    fmt::{self, Display},
};

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub struct Board {
    tiles: Vec<Vec<usize>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in &self.tiles {
            for tile in row {
                s += &format!("{:3}", tile);
            }
            s += "\n";
        }
        write!(f, "{}\n{}", self.dimension(), s)
    }
}

impl Board {
    // create a board from an n-by-n array of tiles,
    // where tiles[row][col] = tile at (row, col)
    pub fn new(tiles: Vec<Vec<usize>>) -> Self {
        assert!(tiles.len() > 1);
        assert!(tiles.len() < 128);
        let mut seen: HashSet<usize> = HashSet::new();
        for row in &tiles {
            assert!(row.len() == tiles.len());
            for tile in row {
                assert!(tile < &(tiles.len() * tiles.len()));
                assert!(seen.insert(*tile));
            }
        }
        Board { tiles }
    }

    // board dimension n
    fn dimension(&self) -> usize {
        self.tiles.len()
    }

    // number of tiles out of place
    pub fn hamming(&self) -> usize {
        let mut i = 1;
        let max = self.tiles.len() * self.tiles.len();
        let mut ham = 0;
        for row in &self.tiles {
            for tile in row {
                if *tile != 0 && *tile != i {
                    ham += 1;
                }
                i += 1;
                i %= max;
            }
        }
        ham
    }

    // sum of Manhattan distances between tiles and goal
    pub fn manhattan(&self) -> usize {
        let mut i = 1;
        let max = self.tiles.len() * self.tiles.len();
        let mut man = 0;
        for (r, row) in self.tiles.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if *tile != 0 && *tile != i {
                    man += r.abs_diff((*tile - 1) / self.dimension())
                        + c.abs_diff((*tile - 1) % self.dimension());
                }
                i += 1;
                i %= max;
            }
        }
        man
    }

    // is this board the goal board?
    pub fn is_goal(&self) -> bool {
        self.hamming() == 0
    }

    fn find_zero(&self) -> (usize, usize) {
        for (r, row) in self.tiles.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if *tile == 0 {
                    return (r, c);
                }
            }
        }
        (0, 0)
    }

    // all neighboring boards
    pub fn neighbors(&self) -> std::vec::IntoIter<Board> {
        // find the 0
        let (r, c) = self.find_zero();
        let mut boards: Vec<Board> = Vec::new();
        // left
        if c > 0 {
            let mut b = self.clone();
            b.tiles[r][c] = b.tiles[r][c - 1];
            b.tiles[r][c - 1] = 0;
            boards.push(b);
        }
        // up
        if r > 0 {
            let mut b = self.clone();
            b.tiles[r][c] = b.tiles[r - 1][c];
            b.tiles[r - 1][c] = 0;
            boards.push(b);
        }
        // down
        if r < self.dimension() - 1 {
            let mut b = self.clone();
            b.tiles[r][c] = b.tiles[r + 1][c];
            b.tiles[r + 1][c] = 0;
            boards.push(b);
        }
        // right
        if c < self.dimension() - 1 {
            let mut b = self.clone();
            b.tiles[r][c] = b.tiles[r][c + 1];
            b.tiles[r][c + 1] = 0;
            boards.push(b);
        }

        boards.into_iter()
    }

    // a board that is obtained by exchanging any pair of tiles (zero is not a tile)
    // used to determine whether a puzzle is solvable: exactly one of a board and its twin are solvable.
    pub fn twin(&self) -> Self {
        let mut twin_board = self.clone();
        let mut from_pos = None;
        let mut to_pos = None;
        for (r, row) in twin_board.tiles.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if *tile != 0 {
                    match from_pos {
                        Some(_) => {
                            to_pos = Some((r, c));
                            break;
                        }
                        None => from_pos = Some((r, c)),
                    }
                }
            }
            if to_pos.is_some() {
                break;
            }
        }
        //swap
        if let Some((fr, fc)) = from_pos {
            if let Some((tr, tc)) = to_pos {
                let ftile = twin_board.tiles[fr][fc];
                let ttile = twin_board.tiles[tr][tc];
                twin_board.tiles[fr][fc] = ttile;
                twin_board.tiles[tr][tc] = ftile;
            }
        }

        twin_board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let b = Board::new(vec![vec![8, 1, 3], vec![4, 0, 2], vec![7, 6, 5]]);
        println!("{b}");
        assert_eq!(b.hamming(), 5);
        assert_eq!(b.manhattan(), 10);
        let b2 = Board::new(vec![vec![8, 1, 3], vec![4, 0, 2], vec![7, 6, 5]]);
        assert_eq!(b, b2);
        let b3 = Board::new(vec![vec![1, 0, 3], vec![4, 2, 5], vec![7, 8, 6]]);
        for b in b3.neighbors() {
            println!("{b}");
        }
    }

    #[test]
    fn test2() {
        let b = Board::new(vec![vec![1, 0, 3], vec![4, 8, 2], vec![7, 6, 5]]);
        println!("{b}");
        let twin = b.twin();
        println!("{twin}");
    }
}
