/*************************************************************************
 *
 *  An immutable data type for Line segments in the plane.
 *  For use on Coursera, Algorithms Part I programming assignment.
 *
 *************************************************************************/

use std::fmt::Display;

use crate::util::std_draw::Plot;

use super::point::Point;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LineSegment {
    p: Point, // one endpoint of this line segment
    q: Point, // the other endpoint of this line segment
}

impl Display for LineSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.p, self.q)
    }
}

impl LineSegment {
    /**
     * Initializes a new line segment.
     *
     * @param  p one endpoint
     * @param  q the other endpoint
     */
    pub fn new(p: Point, q: Point) -> Self {
        LineSegment { p, q }
    }

    /**
     * Draws this line segment to standard draw.
     */
    pub fn draw(&self, plot: &mut Plot) -> anyhow::Result<()> {
        self.p.draw_to(plot, &self.q)?;
        Ok(())
    }
}
