use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::util::{point2d::Point2D, rect_hv::RectHV, std_draw::Plot};

use self::{
    kdtree::{KdTree, SplitHV},
    point_set::PointSET,
};

pub mod kdtree;
pub mod point_set;

pub fn run_point_set(filename: String) -> anyhow::Result<()> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mut points = PointSET::new();
    for row in lines.flatten() {
        let nums: Vec<f64> = row
            .split_ascii_whitespace()
            .map(|n| n.parse::<f64>().unwrap())
            .collect();
        if nums.len() == 2 {
            points.insert(Point2D::new(nums[0], nums[1]));
        }
    }

    let mut plot = Plot::new()?;
    plot.set_x_scale(0.0, 1.0);
    plot.set_y_scale(0.0, 1.0);

    points.draw(&mut plot)?;

    // make a rect and find the points in it
    let rect = RectHV::new(0.25, 0.25, 0.75, 0.75);
    rect.draw(&mut plot)?;

    for p in points.range(rect) {
        p.draw_match(&mut plot)?;
    }

    // find nearest point to 0.5,0.5
    let p = points.nearest(Point2D::new(0.5, 0.5));
    if let Some(near) = p {
        plot.line(0.5, 0.5, near.x(), near.y())?;
    }

    plot.present()?;

    anyhow::Ok(())
}

pub fn run_kdtree(filename: String) -> anyhow::Result<()> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mut points = KdTree::new(SplitHV::V);
    for row in lines.flatten() {
        let nums: Vec<f64> = row
            .split_ascii_whitespace()
            .map(|n| n.parse::<f64>().unwrap())
            .collect();
        if nums.len() == 2 {
            points.insert(Point2D::new(nums[0], nums[1]));
        }
    }

    let mut plot = Plot::new()?;
    plot.set_x_scale(0.0, 1.0);
    plot.set_y_scale(0.0, 1.0);

    points.draw(&mut plot)?;

    // make a rect and find the points in it
    let rect = RectHV::new(0.25, 0.25, 0.75, 0.75);
    rect.draw(&mut plot)?;

    for p in points.range(rect) {
        p.draw_match(&mut plot)?;
    }

    // find nearest point to 0.5,0.5
    let p = points.nearest(Point2D::new(0.5, 0.5));
    if let Some(near) = p {
        plot.line(0.5, 0.5, near.x(), near.y())?;
    }

    plot.present()?;

    anyhow::Ok(())
}
