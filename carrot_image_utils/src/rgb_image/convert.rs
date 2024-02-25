use crate::{gray_image::CarrotGrayImage, rgb_image::rgb_threshold::RGBThreshold};

use image::{GrayImage, Luma, Rgb, RgbImage};

use crate::rgb_image::concat;

pub trait CarrotRgbImage {
    type Gray;
    type Rgb;
    fn to_gray(&self) -> Self::Gray;
    fn to_r_gray(&self) -> Self::Gray;
    fn to_g_gray(&self) -> Self::Gray;
    fn to_b_gray(&self) -> Self::Gray;
    fn to_binary(&self, threshold: &RGBThreshold) -> Self::Gray;
}

impl CarrotRgbImage for RgbImage {
    type Gray = GrayImage;
    type Rgb = RgbImage;

    fn to_gray(&self) -> GrayImage {
        let width = self.width();
        let height = self.height();
        let mut gray_image = GrayImage::new(width, height);
        for i in 0..width {
            for j in 0..height {
                let pixel = self.get_pixel(i, j);
                let gray_pixel = Luma(
                    [((pixel[0] as f32 * 0.2126) as u32
                        + (pixel[1] as f32 * 0.7152) as u32
                        + (pixel[2] as f32 * 0.0722) as u32) as u8; 1],
                );
                gray_image.put_pixel(i, j, gray_pixel);
            }
        }
        gray_image
    }

    /// convert R layer of RGB image to gray image
    fn to_r_gray(&self) -> Self::Gray {
        convert_channel_to_gray_image(self, 0)
    }

    /// convert G layer of RGB image to gray image
    fn to_g_gray(&self) -> Self::Gray {
        convert_channel_to_gray_image(self, 1)
    }

    /// convert B layer of RGB image to gray image
    fn to_b_gray(&self) -> Self::Gray {
        convert_channel_to_gray_image(self, 2)
    }

    /// Convert a rgb image to binary image
    /// If below condition satisfy, then it return 255 (white) and the others return 0 (block).
    /// R threshold low < pixel.r < R threshold high
    /// G threshold low < pixel.g < G threshold high
    /// B threshold low < pixel.b < B threshold high
    fn to_binary(&self, threshold: &RGBThreshold) -> Self::Gray {
        let width = self.width();
        let height = self.height();
        let mut binarized_image = GrayImage::new(width, height);
        for i in 0..width {
            for j in 0..height {
                let pixel = self.get_pixel(i, j);
                let value = threshold.convert_to_binary_pixel_by_threshold(pixel);
                binarized_image.put_pixel(i, j, Luma(value));
            }
        }
        binarized_image
    }
}

pub fn divide_channel(rgb_image: &RgbImage) -> Vec<RgbImage> {
    let width = rgb_image.width();
    let height = rgb_image.height();

    let mut r_image = RgbImage::new(width, height);
    let mut g_image = RgbImage::new(width, height);
    let mut b_image = RgbImage::new(width, height);

    for i in 0..width {
        for j in 0..height {
            let pixel = rgb_image.get_pixel(i, j);
            r_image.put_pixel(i, j, Rgb([pixel[0], 0, 0]));
            g_image.put_pixel(i, j, Rgb([0, pixel[1], 0]));
            b_image.put_pixel(i, j, Rgb([0, 0, pixel[2]]));
        }
    }
    vec![r_image, g_image, b_image]
}

fn convert_channel_to_gray_image(rgb_image: &RgbImage, channel: usize) -> GrayImage {
    let width = rgb_image.width();
    let height = rgb_image.height();
    let mut binary_image = GrayImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            let pixel = rgb_image.get_pixel(i, j);
            binary_image.put_pixel(i, j, Luma([pixel[channel]; 1]));
        }
    }
    binary_image
}

/// Get binarized images for each rgb value to debug
pub fn get_binarized_images_for_each_rgb(
    image: &RgbImage,
    rgb_threshold: &RGBThreshold,
) -> Vec<RgbImage> {
    let binary_r_image = image
        .to_r_gray()
        .to_binary(
            rgb_threshold.r_low_threshold(),
            rgb_threshold.r_high_threshold(),
        )
        .to_rgb();

    let binary_g_image = image
        .to_g_gray()
        .to_binary(
            rgb_threshold.g_low_threshold(),
            rgb_threshold.g_high_threshold(),
        )
        .to_rgb();

    let binary_b_image = image
        .to_b_gray()
        .to_binary(
            rgb_threshold.b_low_threshold(),
            rgb_threshold.b_high_threshold(),
        )
        .to_rgb();
    vec![binary_r_image, binary_g_image, binary_b_image]
}

pub fn get_rgb_threshold_debug_image(
    image: &RgbImage,
    rgb_threshold: &RGBThreshold,
    width: u32,
    height: u32,
) -> RgbImage {
    // input and result images
    let output_image = image.to_binary(rgb_threshold).to_rgb();
    let dummy_image = RgbImage::new(width, height);
    let input_and_results = vec![image.clone(), output_image, dummy_image];

    let rgb_each_images = divide_channel(image);
    let binarized_images = get_binarized_images_for_each_rgb(image, rgb_threshold);

    let image_matrix = vec![input_and_results, rgb_each_images, binarized_images];
    concat::concat_matrix_images(&image_matrix, width, height)
}
