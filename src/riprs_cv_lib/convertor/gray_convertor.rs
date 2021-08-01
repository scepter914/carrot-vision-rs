extern crate image;

use image::{GrayImage, Rgb, RgbImage};

pub fn image_gray_to_rgb(image: &image::GrayImage) -> image::RgbImage {
    let width = image.width();
    let height = image.height();
    let mut rgb_image = image::RgbImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j)[0];
            rgb_image.put_pixel(i, j, image::Rgb([pixel, pixel, pixel]))
        }
    }
    return rgb_image;
}
