use plotters::{backend::RGBPixel, coord::Shift, prelude::*};

const DEFAULT_SIZE: usize = 512;

pub struct Plot<'a> {
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
    width: usize,
    height: usize,
    backend: DrawingArea<BitMapBackend<'a, RGBPixel>, Shift>,
}

impl Plot<'_> {
    pub fn new() -> anyhow::Result<Self> {
        let backend = BitMapBackend::new("./plot.png", (DEFAULT_SIZE as u32, DEFAULT_SIZE as u32))
            .into_drawing_area();
        backend.fill(&WHITE)?;
        Ok(Plot {
            xmin: 0f64,
            ymin: 0f64,
            xmax: DEFAULT_SIZE as f64,
            ymax: DEFAULT_SIZE as f64,
            width: DEFAULT_SIZE,
            height: DEFAULT_SIZE,
            backend,
        })
    }

    pub fn set_x_scale(&mut self, min: f64, max: f64) {
        self.xmin = min;
        self.xmax = max;
    }

    pub fn set_y_scale(&mut self, min: f64, max: f64) {
        self.ymin = min;
        self.ymax = max;
    }

    pub fn scale_x(&mut self, x: f64) -> f64 {
        self.width as f64 * (x - self.xmin) / (self.xmax - self.xmin)
    }
    pub fn scale_y(&mut self, y: f64) -> f64 {
        self.height as f64 * (self.ymax - y) / (self.ymax - self.ymin)
    }

    /**
     * Draws a point centered at (<em>x</em>, <em>y</em>).
     */
    pub fn point(&mut self, x: f64, y: f64) -> anyhow::Result<()> {
        let xs = self.scale_x(x);
        let ys = self.scale_y(y);

        self.backend
            .draw(&Pixel::new((xs as i32, ys as i32), BLUE))?;
        Ok(())
    }

    pub fn point_match(&mut self, x: f64, y: f64) -> anyhow::Result<()> {
        let xs = self.scale_x(x);
        let ys = self.scale_y(y);

        self.backend
            .draw(&Pixel::new((xs as i32, ys as i32), RED))?;
        Ok(())
    }

    pub fn line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) -> anyhow::Result<()> {
        let xs0 = self.scale_x(x0) as i32;
        let ys0 = self.scale_y(y0) as i32;
        let xs1 = self.scale_x(x1) as i32;
        let ys1 = self.scale_y(y1) as i32;

        self.backend
            .draw(&PathElement::new(vec![(xs0, ys0), (xs1, ys1)], BLUE))?;
        Ok(())
    }

    pub fn present(&self) -> anyhow::Result<()> {
        self.backend.present()?;
        Ok(())
    }
}
