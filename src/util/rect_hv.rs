use super::{point2d::Point2D, std_draw::Plot};

#[derive(Debug, PartialEq)]
pub struct RectHV {
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
}

impl RectHV {
    // construct the rectangle [xmin, xmax] x [ymin, ymax]
    pub fn new(xmin: f64, ymin: f64, xmax: f64, ymax: f64) -> Self {
        Self {
            xmin,
            ymin,
            xmax,
            ymax,
        }
        // TODO throw an IllegalArgumentException if (xmin > xmax) or (ymin > ymax)
    }

    pub fn xmin(&self) -> f64 {
        self.xmin
    }

    pub fn ymin(&self) -> f64 {
        self.ymin
    }

    pub fn xmax(&self) -> f64 {
        self.xmax
    }

    pub fn ymax(&self) -> f64 {
        self.ymax
    }

    // does this rectangle contain the point p (either inside or on boundary)?
    pub fn contains(&self, p: &Point2D) -> bool {
        (p.x() >= self.xmin) && (p.x() <= self.xmax) && (p.y() >= self.ymin) && (p.y() <= self.ymax)
    }

    // does this rectangle intersect that rectangle (at one or more points)?
    pub fn intersects(&self, other: RectHV) -> bool {
        self.xmax >= other.xmin
            && self.ymax >= other.ymin
            && other.xmax >= self.xmin
            && other.ymax >= self.ymin
    }

    // Euclidean distance from point p to closest point in rectangle
    pub fn distance_to(&self, p: Point2D) -> f64 {
        self.distance_squared_to(p).sqrt()
    }

    // square of Euclidean distance from point p to closest point in rectangle
    pub fn distance_squared_to(&self, p: Point2D) -> f64 {
        let mut dx = 0.0;
        let mut dy = 0.0;
        if p.x() < self.xmin {
            dx = p.x() - self.xmin;
        } else if p.x() > self.xmax {
            dx = p.x() - self.xmax;
        }
        if p.y() < self.ymin {
            dy = p.y() - self.ymin;
        } else if p.y() > self.ymax {
            dy = p.y() - self.ymax;
        }
        dx * dx + dy * dy
    }

    // draw to standard draw
    pub fn draw(&self, plot: &mut Plot) -> anyhow::Result<()> {
        plot.line(self.xmin, self.ymin, self.xmax, self.ymin)?;
        plot.line(self.xmax, self.ymin, self.xmax, self.ymax)?;
        plot.line(self.xmax, self.ymax, self.xmin, self.ymax)?;
        plot.line(self.xmin, self.ymax, self.xmin, self.ymin)?;
        Ok(())
    }
}
