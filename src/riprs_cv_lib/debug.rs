extern crate image;
extern crate imageproc;

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
        println!(
            "Process {}.{:03} msec",
            end.as_micros() / 1000,
            end.as_micros() % 1000,
        );
    }
}

// RGB image
pub fn print_image_info(image: &RgbImage) -> () {
    println!("dimensions {:?}", image.dimensions());
    //println!("{:?}", self.image.color());
}

pub fn print_pixel_from_point(image: &RgbImage, point: Point<f32>) -> () {
    print_pixel(image, point.x as u32, point.y as u32);
}

pub fn print_pixel(image: &RgbImage, x: u32, y: u32) -> () {
    let pixel: &image::Rgb<u8> = image.get_pixel(x, y);
    println!("RGB : {}, {}, {}", pixel[0], pixel[1], pixel[2]);
}

// Point
pub fn print_point_info(point: &Point<f32>, name: &str) -> () {
    println!("{} : x {}, y {}", name, point.x, point.y);
}
