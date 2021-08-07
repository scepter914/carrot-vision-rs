extern crate image;
extern crate imageproc;

use image::{GrayImage, RgbImage};
use imageproc::drawing::draw_filled_circle;
use imageproc::point::Point;

pub fn get_cg_from_binary(image: &GrayImage) -> Point<f32> {
    let width = image.width();
    let height = image.height();
    let mut cg = Point::new(0.0, 0.0);
    let mut number: u32 = 0;
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            if pixel[0] == 255 {
                number += 1;
                cg.x += i as f32;
                cg.y += j as f32;
            }
        }
    }
    cg.x = cg.x / number as f32;
    cg.y = cg.y / number as f32;
    return cg;
}

pub fn get_cg_debug_image(image: &RgbImage, point: &Point<f32>) -> RgbImage {
    let point_size = image.width() / 80;
    let cg_image = draw_filled_circle(
        image,
        (point.x as i32, point.y as i32),
        point_size as i32,
        image::Rgb([0, 0, 255]),
    );
    return cg_image;
}
