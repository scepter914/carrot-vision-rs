use image::GrayImage;
use imageproc::point::Point;

pub fn get_center_of_gravity(image: &GrayImage) -> Point<f32> {
    let width = image.width();
    let height = image.height();
    let mut cg = Point::new(0.0, 0.0);
    let mut white_number: u32 = 0;
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            if pixel[0] == 255 {
                white_number += 1;
                cg.x += i as f32;
                cg.y += j as f32;
            }
        }
    }
    cg.x /= number as f32;
    cg.y /= number as f32;
    cg
}
