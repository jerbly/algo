/******************************************************************************
 *
 *  An immutable data type for points in the plane.
 *  For use on Coursera, Algorithms Part I programming assignment.
 *
 ******************************************************************************/

use std::{cmp::Ordering, fmt::Display};

use crate::util::std_draw::Plot;

pub trait Comparator<T> {
    fn compare(&self, v: &T, w: &T) -> Ordering;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/**
 * Compares two points by y-coordinate, breaking ties by x-coordinate.
 * Formally, the invoking point (x0, y0) is less than the argument point
 * (x1, y1) if and only if either y0 < y1 or if y0 = y1 and x0 < x1.
 *
 * @param  that the other point
 * @return the value <tt>0</tt> if this point is equal to the argument
 *         point (x0 = x1 and y0 = y1);
 *         a negative integer if this point is less than the argument
 *         point; and a positive integer if this point is greater than the
 *         argument point
 */
impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.y == other.y {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn draw(&self, plot: &mut Plot) -> anyhow::Result<()> {
        plot.point(self.x as f64, self.y as f64)?;
        Ok(())
    }

    pub fn draw_to(&self, plot: &mut Plot, other: &Point) -> anyhow::Result<()> {
        plot.line(self.x as f64, self.y as f64, other.x as f64, other.y as f64)?;
        Ok(())
    }

    /**
     * Returns the slope between this point and the specified point.
     * Formally, if the two points are (x0, y0) and (x1, y1), then the slope
     * is (y1 - y0) / (x1 - x0). For completeness, the slope is defined to be
     * +0.0 if the line segment connecting the two points is horizontal;
     * Double.POSITIVE_INFINITY if the line segment is vertical;
     * and Double.NEGATIVE_INFINITY if (x0, y0) and (x1, y1) are equal.
     *
     * @param  that the other point
     * @return the slope between this point and the specified point
     */
    pub fn slope_to(&self, other: &Point) -> f32 {
        if self == other {
            return f32::NEG_INFINITY;
        }
        let Point { x: x0, y: y0 } = self;
        let Point { x: x1, y: y1 } = other;
        if x0 == x1 {
            return f32::INFINITY;
        }
        if y0 == y1 {
            return 0.0;
        }
        (y1 - y0) as f32 / (x1 - x0) as f32
    }

    /**
     * Compares two points by the slope they make with this point.
     * The slope is defined as in the slopeTo() method.
     *
     * @return the Comparator that defines this ordering on points
     */
    pub fn slope_order(&self) -> SlopeOrderComparator {
        SlopeOrderComparator(self)
    }
}

pub struct SlopeOrderComparator<'a>(&'a Point);

impl<'a> SlopeOrderComparator<'a> {
    pub fn new(p: &'a Point) -> Self {
        SlopeOrderComparator(p)
    }
}

impl Comparator<Point> for SlopeOrderComparator<'_> {
    fn compare(&self, v: &Point, w: &Point) -> Ordering {
        self.0.slope_to(v).total_cmp(&self.0.slope_to(w))
    }
}
