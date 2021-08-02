extern crate image;

use image::{GrayImage, Luma, Rgb, RgbImage};

pub fn convert_to_rgb_image(image: &image::GrayImage) -> image::RgbImage {
    let width = image.width();
    let height = image.height();
    let mut rgb_image = image::RgbImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            rgb_image.put_pixel(i, j, convert_to_rgb_pixel(&pixel))
        }
    }
    return rgb_image;
}

pub fn convert_to_rgb_pixel(pixel: &image::Luma<u8>) -> image::Rgb<u8> {
    return image::Rgb([pixel[0], pixel[0], pixel[0]]);
}
