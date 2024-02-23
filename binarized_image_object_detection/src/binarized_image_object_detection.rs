use std::f32::consts::PI;

use image::GrayImage;
use imageproc::point::Point;

pub struct Circle {
    pub position: Point<f32>,
    pub radius: f32,
}

pub fn get_center_of_gravity(image: &GrayImage) -> Circle {
    let width = image.width();
    let height = image.height();
    let mut center_of_gravity = Point::new(0.0, 0.0);
    let mut mask_number: u32 = 0;
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            if pixel[0] == 255 {
                mask_number += 1;
                center_of_gravity.x += i as f32;
                center_of_gravity.y += j as f32;
            }
        }
    }
    center_of_gravity.x /= mask_number as f32;
    center_of_gravity.y /= mask_number as f32;

    Circle {
        position: center_of_gravity,
        radius: f32::sqrt(mask_number as f32 / std::f32::consts::PI / 2.0),
    }
}
