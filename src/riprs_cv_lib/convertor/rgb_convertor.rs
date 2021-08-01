extern crate image;

use image::{GrayImage, Luma, Rgb, RgbImage};

pub struct RGBThreshold {
    pub high_threshold: Rgb<u8>,
    pub low_threshold: Rgb<u8>,
}

impl RGBThreshold {}

// image rgb to binary
pub fn image_rgb_to_binary_by_rgb_threshold(
    image: &RgbImage,
    rgb_threshold: &RGBThreshold,
) -> GrayImage {
    let width = image.width();
    let height = image.height();
    let mut binarized_image = image::GrayImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            let value = pixel_rgb_to_binary_by_rgb_threshold(pixel, rgb_threshold);
            binarized_image.put_pixel(i, j, image::Luma(value));
        }
    }
    return binarized_image;
}

fn pixel_rgb_to_binary_by_rgb_threshold(pixel: &Rgb<u8>, rgb_threshold: &RGBThreshold) -> [u8; 1] {
    let value: [u8; 1];
    if rgb_threshold.low_threshold[0] <= pixel[0]
        && pixel[0] <= rgb_threshold.high_threshold[0]
        && rgb_threshold.low_threshold[1] <= pixel[1]
        && pixel[1] <= rgb_threshold.high_threshold[1]
        && rgb_threshold.low_threshold[2] <= pixel[2]
        && pixel[2] <= rgb_threshold.high_threshold[2]
    {
        value = [255; 1];
    } else {
        value = [0; 1];
    }
    return value;
}

// image each channel (rgb) to binary
pub fn image_r_to_binary(image: &RgbImage, rgb_threshold: &RGBThreshold) -> GrayImage {
    return image_channel_to_binary(image, rgb_threshold, 0);
}

pub fn image_g_to_binary(image: &RgbImage, rgb_threshold: &RGBThreshold) -> GrayImage {
    return image_channel_to_binary(image, rgb_threshold, 1);
}

pub fn image_b_to_binary(image: &RgbImage, rgb_threshold: &RGBThreshold) -> GrayImage {
    return image_channel_to_binary(image, rgb_threshold, 2);
}

fn image_channel_to_binary(
    image: &RgbImage,
    rgb_threshold: &RGBThreshold,
    channel: usize,
) -> GrayImage {
    let width = image.width();
    let height = image.height();
    let mut binary_image = image::GrayImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            let value = pixel_channel_to_binary(pixel, rgb_threshold, channel);
            binary_image.put_pixel(i, j, image::Luma(value));
        }
    }
    return binary_image;
}

fn pixel_channel_to_binary(
    pixel: &Rgb<u8>,
    rgb_threshold: &RGBThreshold,
    channel: usize,
) -> [u8; 1] {
    let value: [u8; 1];
    if rgb_threshold.low_threshold[channel] <= pixel[channel]
        && pixel[channel] <= rgb_threshold.high_threshold[channel]
    {
        value = [255; 1];
    } else {
        value = [0; 1];
    }
    return value;
}

// image each channel (rgb) to gray
pub fn image_r_to_gray(image: &RgbImage) -> GrayImage {
    return image_channel_to_gray(image, 0);
}

pub fn image_g_to_gray(image: &RgbImage) -> GrayImage {
    return image_channel_to_gray(image, 1);
}

pub fn image_b_to_gray(image: &RgbImage) -> GrayImage {
    return image_channel_to_gray(image, 2);
}

fn image_channel_to_gray(image: &RgbImage, channel: usize) -> GrayImage {
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
