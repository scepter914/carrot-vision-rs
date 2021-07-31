extern crate image;

use image::{GrayImage, RgbImage};

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
  return is_within(pixel, rgb_threshold, true);
}

pub fn is_withinout_threshold(pixel: &image::Rgb<u8>, rgb_threshold: &RGBThreshold) -> bool {
  return is_within(pixel, rgb_threshold, false);
}

/* is_within
 If is_within is true, pixel between low_threshold and high_threshold binarize to 1
 If is_within is false, pixel between low_threshold and high_threshold binarize to 0
*/
fn is_within(pixel: &image::Rgb<u8>, rgb_threshold: &RGBThreshold, is_within: bool) -> bool {
  if rgb_threshold.r_low < pixel[0]
    && pixel[0] < rgb_threshold.r_high
    && rgb_threshold.g_low < pixel[1]
    && pixel[1] < rgb_threshold.g_high
    && rgb_threshold.b_low < pixel[2]
    && pixel[2] < rgb_threshold.b_high
  {
    return is_within;
  } else {
    return !is_within;
  }
}
