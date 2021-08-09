extern crate image;
extern crate imageproc;

use log::{debug, error, info, trace, warn};
use std::time::{Duration, Instant};

use image::RgbImage;
use imageproc::point::Point;

pub struct Benchmark {
    start_time: Instant,
}

impl Benchmark {
    pub fn set_start_time() -> Benchmark {
        let now_time = Instant::now();
        Benchmark {
            start_time: now_time,
        }
    }

    pub fn print_bench_time(&self) -> () {
        let end = self.start_time.elapsed();
        info!(
            "Process {}.{:03} msec",
            end.as_micros() / 1000,
            end.as_micros() % 1000,
        );
    }
}

// RGB image
pub fn print_image_info(image: &RgbImage) -> () {
    info!("dimensions {:?}", image.dimensions());
    //info!("{:?}", self.image.color());
}

pub fn print_pixel_from_point(debug_message: &str, image: &RgbImage, point: Point<f32>) -> () {
    print_pixel(debug_message, image, point.x as u32, point.y as u32);
}

pub fn print_pixel(debug_message: &str, image: &RgbImage, x: u32, y: u32) -> () {
    let pixel: &image::Rgb<u8> = image.get_pixel(x, y);
    debug!(
        "{}, RGB : {}, {}, {}",
        debug_message, pixel[0], pixel[1], pixel[2]
    );
}

// Point
pub fn print_point_info(debug_message: &str, point: &Point<f32>) -> () {
    debug!("{}, x {}, y {}", debug_message, point.x, point.y);
}
