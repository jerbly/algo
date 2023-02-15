use std::time::Instant;

use crate::util::{point2d::Point2D, rect_hv::RectHV, std_draw::Plot};

pub enum SplitHV {
    H,
    V,
}

pub struct KdTree {
    split: SplitHV,
    point: Option<Point2D>,
    left: Option<Box<KdTree>>,
    right: Option<Box<KdTree>>,
}

impl KdTree {
    // construct an empty tree of points
    pub fn new(split: SplitHV) -> Self {
        Self {
            split,
            point: None,
            left: None,
            right: None,
        }
    }

    // is the tree empty?
    pub fn is_empty(&self) -> bool {
        self.point.is_none()
    }

    // number of points in the tree
    pub fn size(&self) -> usize {
        self.all().len()
    }

    // add the point to the tree
    pub fn insert(&mut self, p: Point2D) {
        match &self.point {
            Some(point) => match self.split {
                SplitHV::H => {
                    if p.y() < point.y() {
                        // go left
                        if self.left.is_none() {
                            self.left = Some(Box::new(KdTree::new(SplitHV::V)));
                        }
                        self.left.as_mut().unwrap().insert(p);
                    } else {
                        // go right
                        if self.right.is_none() {
                            self.right = Some(Box::new(KdTree::new(SplitHV::V)));
                        }
                        self.right.as_mut().unwrap().insert(p);
                    }
                }
                SplitHV::V => {
                    if p.x() < point.x() {
                        // go left
                        if self.left.is_none() {
                            self.left = Some(Box::new(KdTree::new(SplitHV::H)));
                        }
                        self.left.as_mut().unwrap().insert(p);
                    } else {
                        // go right
                        if self.right.is_none() {
                            self.right = Some(Box::new(KdTree::new(SplitHV::H)));
                        }
                        self.right.as_mut().unwrap().insert(p);
                    }
                }
            },
            None => self.point = Some(p),
        }
    }

    // does the tree contain point p?
    pub fn contains(&self, p: Point2D) -> bool {
        match &self.point {
            Some(point) => {
                if *point == p {
                    true
                } else {
                    match self.split {
                        SplitHV::H => {
                            if p.y() < point.y() {
                                // go left
                                match &self.left {
                                    Some(t) => t.contains(p),
                                    None => false,
                                }
                            } else {
                                // go right
                                match &self.right {
                                    Some(t) => t.contains(p),
                                    None => false,
                                }
                            }
                        }
                        SplitHV::V => {
                            if p.x() < point.x() {
                                // go left
                                match &self.left {
                                    Some(t) => t.contains(p),
                                    None => false,
                                }
                            } else {
                                // go right
                                match &self.right {
                                    Some(t) => t.contains(p),
                                    None => false,
                                }
                            }
                        }
                    }
                }
            }
            None => false,
        }
    }

    // draw all points to standard draw
    pub fn draw(&self, plot: &mut Plot) -> anyhow::Result<()> {
        for p in self.all() {
            p.draw(plot)?;
        }
        Ok(())
    }

    // all points that are inside the rectangle (or on the boundary)
    pub fn range(&self, query_rect: RectHV) -> Vec<&Point2D> {
        /* Range search. To find all points contained in a given query rectangle,
        start at the root and recursively search for points in both subtrees using the following pruning rule:
        if the query rectangle does not intersect the rectangle corresponding to a node,
        there is no need to explore that node (or its subtrees).
        A subtree is searched only if it might contain a point contained in the query rectangle. */
        let mut points = vec![];
        let rect = RectHV::new(0.0, 0.0, 1.0, 1.0);
        self.collect_range_points(&query_rect, &rect, &mut points);
        points
    }

    fn collect_range_points<'a>(
        &'a self,
        query_rect: &RectHV,
        rect: &RectHV,
        points: &mut Vec<&'a Point2D>,
    ) {
        if let Some(p) = &self.point {
            if query_rect.contains(p) {
                points.push(p);
            }
            if query_rect.intersects(rect) {
                // now update the rects and contiue if they intersect
                if let Some(t) = &self.left {
                    match &self.split {
                        SplitHV::H => {
                            let new_rect =
                                RectHV::new(rect.xmin(), rect.ymin(), rect.xmax(), p.y());
                            t.collect_range_points(query_rect, &new_rect, points);
                        }
                        SplitHV::V => {
                            let new_rect =
                                RectHV::new(rect.xmin(), rect.ymin(), p.x(), rect.ymax());
                            t.collect_range_points(query_rect, &new_rect, points);
                        }
                    }
                }
                if let Some(t) = &self.right {
                    match &self.split {
                        SplitHV::H => {
                            let new_rect =
                                RectHV::new(rect.xmin(), p.y(), rect.xmax(), rect.ymax());
                            t.collect_range_points(query_rect, &new_rect, points);
                        }
                        SplitHV::V => {
                            let new_rect =
                                RectHV::new(p.x(), rect.ymin(), rect.xmax(), rect.ymax());
                            t.collect_range_points(query_rect, &new_rect, points);
                        }
                    }
                }
            }
        }
    }

    // a nearest neighbor in the set to point p; null if the set is empty
    pub fn nearest(&self, p: Point2D) -> Option<&Point2D> {
        /* Nearest-neighbor search. To find a closest point to a given query point,
        start at the root and recursively search in both subtrees using the following pruning rule:
        if the closest point discovered so far is closer than the distance between the query point and the rectangle
        corresponding to a node, there is no need to explore that node (or its subtrees).
        That is, search a node only only if it might contain a point that is closer than the best one found so far.
        The effectiveness of the pruning rule depends on quickly finding a nearby point.
        To do this, organize the recursive method so that when there are two possible subtrees to go down,
        you always choose the subtree that is on the same side of the splitting line as the query point as the
        first subtree to explore—the closest point found while exploring the first subtree may enable pruning of
        the second subtree. */
        if let Some(best_point) = &self.point {
            let start = Instant::now();
            let rect = RectHV::new(0.0, 0.0, 1.0, 1.0);
            let (bp, _) = self.inner_nearest(&p, &rect, best_point, f64::MAX);
            let duration = start.elapsed();
            println!("{:?}", duration);
            Some(bp)
        } else {
            None
        }
    }

    fn inner_nearest<'a>(
        &'a self,
        query_point: &Point2D,
        rect: &RectHV,
        best_point: &'a Point2D,
        best_distance: f64,
    ) -> (&Point2D, f64) {
        if let Some(p) = &self.point {
            let mut d = p.distance_to(query_point);
            let mut bp = if d < best_distance {
                p
            } else {
                d = best_distance;
                best_point
            };

            let mut went_left = false;
            let mut went_right = false;

            if self.left.is_some() && self.right.is_some() {
                // choose the subtree on the same side of the splitting line as the query point
                let go_left = match &self.split {
                    SplitHV::H => query_point.y() < p.y(),
                    SplitHV::V => query_point.x() < p.x(),
                };

                if go_left {
                    if let Some(t) = &self.left {
                        let new_rect = match &self.split {
                            SplitHV::H => RectHV::new(rect.xmin(), rect.ymin(), rect.xmax(), p.y()),
                            SplitHV::V => RectHV::new(rect.xmin(), rect.ymin(), p.x(), rect.ymax()),
                        };
                        if new_rect.distance_to(query_point) < d {
                            (bp, d) = t.inner_nearest(query_point, &new_rect, bp, d);
                        }
                    }
                    went_left = true;
                } else {
                    if let Some(t) = &self.right {
                        let new_rect = match &self.split {
                            SplitHV::H => RectHV::new(rect.xmin(), p.y(), rect.xmax(), rect.ymax()),
                            SplitHV::V => RectHV::new(p.x(), rect.ymin(), rect.xmax(), rect.ymax()),
                        };
                        if new_rect.distance_to(query_point) < d {
                            (bp, d) = t.inner_nearest(query_point, &new_rect, bp, d);
                        }
                    }
                    went_right = true;
                }
            }

            if !went_left {
                if let Some(t) = &self.left {
                    let new_rect = match &self.split {
                        SplitHV::H => RectHV::new(rect.xmin(), rect.ymin(), rect.xmax(), p.y()),
                        SplitHV::V => RectHV::new(rect.xmin(), rect.ymin(), p.x(), rect.ymax()),
                    };
                    if new_rect.distance_to(query_point) < d {
                        (bp, d) = t.inner_nearest(query_point, &new_rect, bp, d);
                    }
                }
            }

            if !went_right {
                if let Some(t) = &self.right {
                    let new_rect = match &self.split {
                        SplitHV::H => RectHV::new(rect.xmin(), p.y(), rect.xmax(), rect.ymax()),
                        SplitHV::V => RectHV::new(p.x(), rect.ymin(), rect.xmax(), rect.ymax()),
                    };
                    if new_rect.distance_to(query_point) < d {
                        (bp, d) = t.inner_nearest(query_point, &new_rect, bp, d);
                    }
                }
            }
            (bp, d)
        } else {
            (best_point, best_distance)
        }
    }

    pub fn all(&self) -> Vec<&Point2D> {
        let mut points = vec![];
        self.collect_points(&mut points);
        points
    }

    fn collect_points<'a>(&'a self, points: &mut Vec<&'a Point2D>) {
        if let Some(p) = &self.point {
            points.push(p);
            if let Some(t) = &self.left {
                t.collect_points(points);
            }
            if let Some(t) = &self.right {
                t.collect_points(points);
            }
        }
    }
}
