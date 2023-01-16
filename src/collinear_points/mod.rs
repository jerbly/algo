use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

use crate::util::std_draw::Plot;

use self::{
    line_segment::LineSegment,
    point::{Comparator, Point, SlopeOrderComparator},
};
use itertools::Itertools;

pub mod line_segment;
pub mod point;

struct BruteCollinearPoints(Vec<LineSegment>);

impl BruteCollinearPoints {
    fn new(points: Vec<Point>) -> Self {
        // finds all line segments containing 4 points
        let mut ls: Vec<LineSegment> = vec![];
        // get all combinations of 4 points and find the ones with equal slopes
        let coms = points.iter().combinations(4);
        for mut v in coms {
            if v[0].slope_to(v[1]) == v[0].slope_to(v[2])
                && v[0].slope_to(v[1]) == v[0].slope_to(v[3])
            {
                v.sort();
                ls.push(LineSegment::new(v[0].clone(), v[3].clone()));
            }
        }

        Self(ls)
    }

    fn number_of_segments(&self) -> usize {
        // the number of line segments
        self.0.len()
    }

    fn segments(&self) -> &Vec<LineSegment> {
        // the line segments
        &self.0
    }
}

struct FastCollinearPoints(Vec<LineSegment>);

impl FastCollinearPoints {
    fn new(points: &Vec<Point>) -> Self {
        // finds all line segments containing 4 points
        let mut ls: HashSet<LineSegment> = HashSet::new();
        let mut points_copy = points.clone();
        // Think of p as the origin.
        // For each other point q, determine the slope it makes with p.
        // Sort the points according to the slopes they makes with p.
        for point in points {
            // println!("=====================================================================");
            let comparator = SlopeOrderComparator::new(point);
            points_copy.sort_by(|v, w| comparator.compare(v, w));
            // println!("Point={point} List={:?}", points_copy);
            // Check if any 3 (or more) adjacent points in the sorted order have equal slopes with respect to p.
            // If so, these points, together with p, are collinear.
            let mut slope: f32 = f32::NAN;
            let mut adjacent = 1usize;
            for (i, p) in points_copy.iter().enumerate() {
                if point == p {
                    continue;
                }
                let s = point.slope_to(p);
                // println!("Comparing {slope} with {s} for {p}");
                if s == slope {
                    adjacent += 1;
                    // println!("Inc adjacent: {adjacent}");
                } else {
                    slope = s;
                    if adjacent >= 3 {
                        let mut v: Vec<&Point> = points_copy[i - adjacent..i].iter().collect();
                        v.push(point);
                        v.sort();
                        // println!("Adjacents={:?}", v);
                        ls.insert(LineSegment::new(v[0].clone(), v[v.len() - 1].clone()));
                        // adjacent = 1;
                        // break;
                    }
                    adjacent = 1;
                }
            }
            if adjacent >= 3 {
                let mut v: Vec<&Point> =
                    points_copy[points_copy.len() - adjacent..].iter().collect();
                v.push(point);
                v.sort();
                // println!("Adjacents={:?}", v);
                ls.insert(LineSegment::new(v[0].clone(), v[v.len() - 1].clone()));
            }

            // println!();
        }
        // Applying this method for each of the n points in turn yields an efficient algorithm to the problem.
        // The algorithm solves the problem because points that have equal slopes with respect to p are collinear,
        // and sorting brings such points together. The algorithm is fast because the bottleneck operation is sorting.

        Self(ls.iter().cloned().collect())
    }

    fn number_of_segments(&self) -> usize {
        // the number of line segments
        self.0.len()
    }

    fn segments(&self) -> &Vec<LineSegment> {
        // the line segments
        &self.0
    }
}

pub fn run_collinear_points(filename: String) -> anyhow::Result<()> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mut points: Vec<Point> = vec![];
    for row in lines.flatten() {
        let nums: Vec<i32> = row
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        if nums.len() == 2 {
            points.push(Point::new(nums[0], nums[1]));
        }
    }
    let b = FastCollinearPoints::new(&points);

    let mut plot = Plot::new()?;
    plot.set_x_scale(0.0, 32768.0);
    plot.set_y_scale(0.0, 32768.0);

    for p in points {
        p.draw(&mut plot)?;
    }

    for ls in b.segments() {
        println!("{}", ls);
        ls.draw(&mut plot)?;
    }

    plot.present()?;

    anyhow::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brute() {
        let points = vec![
            //Point::new(19000, 10000),
            Point::new(18000, 10000),
            Point::new(32000, 10000),
            Point::new(21000, 10000),
            Point::new(1234, 5678),
            Point::new(14000, 10000),
        ];
        let b = BruteCollinearPoints::new(points);
        assert_eq!(b.number_of_segments(), 1);
        let a = vec![LineSegment::new(
            Point::new(14000, 10000),
            Point::new(32000, 10000),
        )];
        itertools::assert_equal(&a, b.segments());
        for ls in b.segments() {
            println!("{}", ls);
        }
    }

    #[test]
    fn test_fast6() {
        let points = vec![
            Point::new(19000, 10000),
            Point::new(18000, 10000),
            Point::new(32000, 10000),
            Point::new(21000, 10000),
            Point::new(1234, 5678),
            Point::new(14000, 10000),
        ];
        let b = FastCollinearPoints::new(&points);
        assert_eq!(b.number_of_segments(), 1);
        let a = vec![LineSegment::new(
            Point::new(14000, 10000),
            Point::new(32000, 10000),
        )];
        itertools::assert_equal(&a, b.segments());
        for ls in b.segments() {
            println!("{}", ls);
        }
    }

    #[test]
    fn test_fast8() {
        let points = vec![
            Point::new(10000, 0),
            Point::new(0, 10000),
            Point::new(3000, 7000),
            Point::new(7000, 3000),
            Point::new(20000, 21000),
            Point::new(3000, 4000),
            Point::new(14000, 15000),
            Point::new(6000, 7000),
        ];
        let b = FastCollinearPoints::new(&points);
        assert_eq!(b.number_of_segments(), 2);
        let a = vec![
            LineSegment::new(Point::new(10000, 0), Point::new(0, 10000)),
            LineSegment::new(Point::new(3000, 4000), Point::new(20000, 21000)),
        ];
        itertools::assert_equal(&a, b.segments());
        for ls in b.segments() {
            println!("{}", ls);
        }
    }
}
