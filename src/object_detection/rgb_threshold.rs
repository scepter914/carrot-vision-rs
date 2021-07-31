extern crate image;

use image::{GrayImage, Rgb, RgbImage};

pub struct RGBThreshold {
    pub r_high: u8,
    pub r_low: u8,
    pub g_high: u8,
    pub g_low: u8,
    pub b_high: u8,
    pub b_low: u8,
}

pub fn binalize_rgb(image: &RgbImage, rgb_threshold: &RGBThreshold) -> GrayImage {
    let width = image.width();
    let height = image.height();
    let mut binarized_image = image::GrayImage::new(width, height);

    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            let value: [u8; 1];
            if is_within_threshold(pixel, &rgb_threshold) {
                value = [255; 1];
            } else {
                value = [0; 1];
            }
            binarized_image.put_pixel(i, j, image::Luma(value));
        }
    }
    return binarized_image;
}

pub fn is_within_threshold(pixel: &image::Rgb<u8>, rgb_threshold: &RGBThreshold) -> bool {
    if rgb_threshold.r_low <= pixel[0]
        && pixel[0] <= rgb_threshold.r_high
        && rgb_threshold.g_low <= pixel[1]
        && pixel[1] <= rgb_threshold.g_high
        && rgb_threshold.b_low <= pixel[2]
        && pixel[2] <= rgb_threshold.b_high
    {
        return true;
    } else {
        return false;
    }
}

pub fn is_without_threshold(pixel: &image::Rgb<u8>, rgb_threshold: &RGBThreshold) -> bool {
    if is_within_threshold(pixel, rgb_threshold) {
        return false;
    } else {
        return true;
    }
}

pub fn combine_rgb_disassembled_image(image: &RgbImage) -> RgbImage {
    let width = image.width();
    let height = image.height();
    let (r_image, g_image, b_image) = disassemble_rgb(image);
    let rgb_r_image = gray_to_rgb_image(&r_image);
    let rgb_g_image = gray_to_rgb_image(&g_image);
    let rgb_b_image = gray_to_rgb_image(&b_image);

    let mut combined_image = image::RgbImage::new(2 * width, 2 * height);

    for i in 0..width {
        for j in 0..height {
            let pixel = rgb_r_image.get_pixel(i, j);
            combined_image.put_pixel(i, j, *pixel);
            let pixel = rgb_g_image.get_pixel(i, j);
            combined_image.put_pixel(width + i, j, *pixel);
            let pixel = rgb_b_image.get_pixel(i, j);
            combined_image.put_pixel(i, height + j, *pixel);
            let pixel = image.get_pixel(i, j);
            combined_image.put_pixel(width + i, height + j, *pixel);
        }
    }
    return combined_image;
}

fn disassemble_rgb(image: &RgbImage) -> (image::GrayImage, image::GrayImage, image::GrayImage) {
    let width = image.width();
    let height = image.height();

    // init
    let mut r_image = image::GrayImage::new(width, height);
    let mut g_image = image::GrayImage::new(width, height);
    let mut b_image = image::GrayImage::new(width, height);

    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            r_image.put_pixel(i, j, image::Luma([pixel[0]; 1]));
            g_image.put_pixel(i, j, image::Luma([pixel[1]; 1]));
            b_image.put_pixel(i, j, image::Luma([pixel[2]; 1]));
        }
    }
    return (r_image, g_image, b_image);
}

fn gray_to_rgb_image(image: &image::GrayImage) -> image::RgbImage {
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
