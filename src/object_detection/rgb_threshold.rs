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

impl RGBThreshold {
    pub fn get_low_threshold_from_channel(&self, channel: i32) -> u8 {
        if channel == 0 {
            return self.r_low;
        } else if channel == 1 {
            return self.g_low;
        } else {
            return self.b_low;
        }
    }
    pub fn get_high_threshold_from_channel(&self, channel: i32) -> u8 {
        if channel == 0 {
            return self.r_high;
        } else if channel == 1 {
            return self.g_high;
        } else {
            return self.b_high;
        }
    }
}

pub fn binalize_rgb(image: &RgbImage, rgb_threshold: &RGBThreshold) -> GrayImage {
    let width = image.width();
    let height = image.height();
    let mut binarized_image = image::GrayImage::new(width, height);

    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            let value: [u8; 1];
            if is_within_rgb_threshold(pixel, &rgb_threshold) {
                value = [255; 1];
            } else {
                value = [0; 1];
            }
            binarized_image.put_pixel(i, j, image::Luma(value));
        }
    }
    return binarized_image;
}

pub fn is_within_rgb_threshold(pixel: &image::Rgb<u8>, rgb_threshold: &RGBThreshold) -> bool {
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

pub fn is_without_rgb_threshold(pixel: &image::Rgb<u8>, rgb_threshold: &RGBThreshold) -> bool {
    if is_within_rgb_threshold(pixel, rgb_threshold) {
        return false;
    } else {
        return true;
    }
}

pub fn combine_rgb_disassembled_image(image: &RgbImage, rgb_threshold: &RGBThreshold) -> RgbImage {
    let width = image.width();
    let height = image.height();

    let binarized_image = binalize_rgb(image, &rgb_threshold);
    let rgb_binarized_image = gray_to_rgb_image(&binarized_image);

    let (r_image, g_image, b_image) = disassemble_rgb(image);

    let rgb_r_image = gray_to_rgb_image(&r_image);
    let rgb_g_image = gray_to_rgb_image(&g_image);
    let rgb_b_image = gray_to_rgb_image(&b_image);

    let r_thres_image = binalize_gray(&r_image, rgb_threshold, 0);
    let g_thres_image = binalize_gray(&g_image, rgb_threshold, 1);
    let b_thres_image = binalize_gray(&b_image, rgb_threshold, 2);

    let rgb_r_thres_image = gray_to_rgb_image(&r_thres_image);
    let rgb_g_thres_image = gray_to_rgb_image(&g_thres_image);
    let rgb_b_thres_image = gray_to_rgb_image(&b_thres_image);

    let mut combined_image = image::RgbImage::new(2 * width, 4 * height);

    for i in 0..width {
        for j in 0..height {
            // input image
            let pixel = image.get_pixel(i, j);
            combined_image.put_pixel(i, j, *pixel);
            // binalized image
            let pixel = rgb_binarized_image.get_pixel(i, j);
            combined_image.put_pixel(width + i, j, *pixel);
            // r, r thres
            let pixel = rgb_r_image.get_pixel(i, j);
            combined_image.put_pixel(i, height + j, *pixel);
            let pixel = rgb_r_thres_image.get_pixel(i, j);
            combined_image.put_pixel(width + i, height + j, *pixel);
            // g, g thres
            let pixel = rgb_g_image.get_pixel(i, j);
            combined_image.put_pixel(i, 2 * height + j, *pixel);
            let pixel = rgb_g_thres_image.get_pixel(i, j);
            combined_image.put_pixel(width + i, 2 * height + j, *pixel);
            // b, b thres
            let pixel = rgb_b_image.get_pixel(i, j);
            combined_image.put_pixel(i, 3 * height + j, *pixel);
            let pixel = rgb_b_thres_image.get_pixel(i, j);
            combined_image.put_pixel(width + i, 3 * height + j, *pixel);
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

pub fn binalize_gray(image: &GrayImage, rgb_threshold: &RGBThreshold, channel: i32) -> GrayImage {
    let width = image.width();
    let height = image.height();
    let mut binarized_image = image::GrayImage::new(width, height);

    for i in 0..width {
        for j in 0..height {
            let pixel = image.get_pixel(i, j);
            // channel
            let low_threshold: u8 = rgb_threshold.get_low_threshold_from_channel(channel);
            let high_threshold: u8 = rgb_threshold.get_high_threshold_from_channel(channel);

            let value: [u8; 1];
            if low_threshold <= pixel[0] && pixel[0] <= high_threshold {
                value = [255; 1];
            } else {
                value = [0; 1];
            }
            binarized_image.put_pixel(i, j, image::Luma(value));
        }
    }
    return binarized_image;
}
