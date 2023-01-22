use std::{
    collections::HashSet,
    fmt::{self, Display},
};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Board {
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
    fn new(tiles: Vec<Vec<usize>>) -> Self {
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
    fn hamming(&self) -> usize {
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
    fn manhattan(&self) -> usize {
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
    fn is_goal() -> bool {
        todo!();
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
    fn neighbors(&self) -> std::vec::IntoIter<Board> {
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

    // a board that is obtained by exchanging any pair of tiles
    fn twin() -> Self {
        todo!();
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
        for a in 1..9 {
            println!("{a} {},{}", (a - 1) / 3, (a - 1) % 3);
        }
    }
}
