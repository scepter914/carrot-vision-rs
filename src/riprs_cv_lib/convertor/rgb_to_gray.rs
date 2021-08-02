extern crate image;

use image::{GrayImage, Luma, Rgb, RgbImage};

// image each channel (rgb) to gray
pub fn convert_r_to_gray_image(image: &RgbImage) -> GrayImage {
    return convert_channel_to_gray_image(image, 0);
}

pub fn convert_g_to_gray_image(image: &RgbImage) -> GrayImage {
    return convert_channel_to_gray_image(image, 1);
}

pub fn convert_b_to_gray_image(image: &RgbImage) -> GrayImage {
    return convert_channel_to_gray_image(image, 2);
}

fn convert_channel_to_gray_image(image: &RgbImage, channel: usize) -> GrayImage {
    let width = image.width();
    let height = image.height();
    let mut binary_image = image::GrayImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            binary_image.put_pixel(i, j, image::Luma([pixel[channel]; 1]));
        }
    }
    return binary_image;
}
