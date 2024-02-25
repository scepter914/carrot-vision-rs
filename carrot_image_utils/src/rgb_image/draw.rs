use image::RgbImage;
use imageproc::drawing;
use imageproc::point::Point;

use crate::geometry::point;

/// Get debug image drawing a point
pub fn draw_point(image: &RgbImage, point: Point<f32>, rgb: image::Rgb<u8>) -> RgbImage {
    let point_size: i32 = image.width() as i32 / 80;
    drawing::draw_filled_circle(image, point::to_i32_tuple(point), point_size, rgb)
}
