use float_ord::FloatOrd;

use super::std_draw::Plot;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point2D {
    x: FloatOrd<f64>,
    y: FloatOrd<f64>,
}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.y == other.y {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Point2D {
    // construct the point (x, y)
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: FloatOrd(x),
            y: FloatOrd(y),
        }
    }

    // x-coordinate
    pub fn x(&self) -> f64 {
        self.x.0
    }

    // y-coordinate
    pub fn y(&self) -> f64 {
        self.y.0
    }

    // Euclidean distance between two points
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        self.distance_squared_to(other).sqrt()
    }

    // square of Euclidean distance between two points
    pub fn distance_squared_to(&self, other: &Point2D) -> f64 {
        let dx = self.x.0 - other.x.0;
        let dy = self.y.0 - other.y.0;
        dx * dx + dy * dy
    }

    // draw to standard draw
    pub fn draw(&self, plot: &mut Plot) -> anyhow::Result<()> {
        plot.point(self.x.0, self.y.0)?;
        Ok(())
    }
    pub fn draw_match(&self, plot: &mut Plot) -> anyhow::Result<()> {
        plot.point_match(self.x.0, self.y.0)?;
        Ok(())
    }
}
