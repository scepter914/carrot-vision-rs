extern crate image;

use image::{GrayImage, Luma};

pub fn convert_gray_to_binary_within_threshold(
    image: &GrayImage,
    low_threshold: u8,
    high_threshold: u8,
) -> GrayImage {
    let width = image.width();
    let height = image.height();
    let mut binary_image = image::GrayImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            let value = binarize_pixel(pixel, high_threshold, low_threshold);
            binary_image.put_pixel(i, j, value);
        }
    }
    return binary_image;
}

fn binarize_pixel(pixel: &Luma<u8>, low_threshold: u8, high_threshold: u8) -> image::Luma<u8> {
    let value: [u8; 1];
    if low_threshold <= pixel[0] && pixel[0] <= high_threshold {
        value = [255; 1];
    } else {
        value = [0; 1];
    }
    return image::Luma(value);
}
