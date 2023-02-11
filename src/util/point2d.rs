use super::std_draw::Plot;

#[derive(Debug, PartialEq)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.y.partial_cmp(&other.y) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.x.partial_cmp(&other.x)
    }
}

impl Point2D {
    // construct the point (x, y)
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // x-coordinate
    pub fn x(&self) -> f64 {
        self.x
    }

    // y-coordinate
    pub fn y(&self) -> f64 {
        self.y
    }

    // Euclidean distance between two points
    pub fn distance_to(&self, other: Point2D) -> f64 {
        self.distance_squared_to(other).sqrt()
    }

    // square of Euclidean distance between two points
    pub fn distance_squared_to(&self, other: Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    // draw to standard draw
    pub fn draw(&self, plot: &mut Plot) -> anyhow::Result<()> {
        plot.point(self.x, self.y)?;
        Ok(())
    }
}
