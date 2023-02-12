use std::collections::BTreeSet;

use crate::util::{point2d::Point2D, rect_hv::RectHV, std_draw::Plot};

struct PointSET {
    set: BTreeSet<Point2D>,
}

impl PointSET {
    // construct an empty set of points
    pub fn new() -> Self {
        Self {
            set: BTreeSet::new(),
        }
    }

    // is the set empty?
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    // number of points in the set
    pub fn size(&self) -> usize {
        self.set.len()
    }

    // add the point to the set (if it is not already in the set)
    pub fn insert(&mut self, p: Point2D) {
        self.set.insert(p);
    }

    // does the set contain point p?
    pub fn contains(&self, p: Point2D) -> bool {
        self.set.contains(&p)
    }

    // draw all points to standard draw
    pub fn draw(&self, plot: &mut Plot) -> anyhow::Result<()> {
        for p in self.set.iter() {
            p.draw(plot)?;
        }
        Ok(())
    }

    // all points that are inside the rectangle (or on the boundary)
    pub fn range(&self, rect: RectHV) -> Vec<Point2D> {
        let mut v = vec![];
        for p in self.set.iter() {
            if rect.contains(p) {
                v.push(p.clone());
            }
        }
        v
    }

    // a nearest neighbor in the set to point p; null if the set is empty
    pub fn nearest(&self, p: Point2D) -> Option<&Point2D> {
        let mut near: Option<&Point2D> = None;
        let mut dist: f64 = f64::MAX;
        for other in self.set.iter() {
            let d = p.distance_to(other);
            if d < dist {
                dist = d;
                near = Some(other);
            }
        }
        near
    }
}
