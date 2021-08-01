extern crate image;
extern crate imageproc;

use image::{GrayImage, RgbImage};
use imageproc::point::Point;

pub struct Moment {
    moment_10: Point<f32>,
    moment_01: Point<f32>,
    moment_20: Point<f32>,
    moment_11: Point<f32>,
    moment_02: Point<f32>,
}

impl Moment {
    pub fn new(binary_image: &GrayImage) -> Moment {
        Moment {
            moment_10: Point::new(0.0, 0.0),
            moment_01: Point::new(0.0, 0.0),
            moment_20: Point::new(0.0, 0.0),
            moment_11: Point::new(0.0, 0.0),
            moment_02: Point::new(0.0, 0.0),
        }
    }
}
