use image::{GrayImage, Luma, Rgb, RgbImage};

pub trait CarrotGrayImage {
    fn to_binary(&self, low_threshold: u8, high_threshold: u8) -> GrayImage;
    fn to_binary_by_otsu(&self) -> GrayImage;
    fn to_rgb(&self) -> RgbImage;
    fn to_r(&self) -> RgbImage;
    fn to_b(&self) -> RgbImage;
    fn to_g(&self) -> RgbImage;
}

impl CarrotGrayImage for GrayImage {
    /// - Function
    /// Convert gray image to binary image by threshold
    /// If low_threshold <= pixel_value <= high_threshold then, output pixel is 255 (white color)
    ///
    /// - Use case
    /// If you want to a get binary image from pixel above threshold of a gray image, you should use convert_gray_to_binary_within_threshold(gray_image, threshold, 255)
    fn to_binary(&self, low_threshold: u8, high_threshold: u8) -> GrayImage {
        let width = self.width();
        let height = self.height();
        let mut binary_image = GrayImage::new(width, height);
        for i in 0..width {
            for j in 0..height {
                let pixel = self.get_pixel(i, j);
                let value = binarize_pixel_by_threshold(pixel, low_threshold, high_threshold);
                binary_image.put_pixel(i, j, value);
            }
        }
        binary_image
    }

    fn to_binary_by_otsu(&self) -> GrayImage {
        let otsu_level = imageproc::contrast::otsu_level(self);
        imageproc::contrast::threshold(self, otsu_level)
    }

    /// convert from gray image to rgb image
    fn to_rgb(&self) -> RgbImage {
        let width = self.width();
        let height = self.height();
        let mut rgb_image = RgbImage::new(width, height);
        for i in 0..width {
            for j in 0..height {
                let pixel = self.get_pixel(i, j);
                rgb_image.put_pixel(i, j, convert_gray_to_rgb_pixel(&pixel))
            }
        }
        rgb_image
    }

    fn to_r(&self) -> RgbImage {
        let width = self.width();
        let height = self.height();
        let mut rgb_image = RgbImage::new(width, height);
        for i in 0..width {
            for j in 0..height {
                let pixel = self.get_pixel(i, j);
                rgb_image.put_pixel(i, j, convert_gray_to_r_pixel(&pixel))
            }
        }
        rgb_image
    }

    fn to_g(&self) -> RgbImage {
        let width = self.width();
        let height = self.height();
        let mut rgb_image = RgbImage::new(width, height);
        for i in 0..width {
            for j in 0..height {
                let pixel = self.get_pixel(i, j);
                rgb_image.put_pixel(i, j, convert_gray_to_g_pixel(&pixel))
            }
        }
        rgb_image
    }

    fn to_b(&self) -> RgbImage {
        let width = self.width();
        let height = self.height();
        let mut rgb_image = RgbImage::new(width, height);
        for i in 0..width {
            for j in 0..height {
                let pixel = self.get_pixel(i, j);
                rgb_image.put_pixel(i, j, convert_gray_to_b_pixel(&pixel))
            }
        }
        rgb_image
    }
}

fn binarize_pixel_by_threshold(
    pixel: &Luma<u8>,
    low_threshold: u8,
    high_threshold: u8,
) -> Luma<u8> {
    let value: [u8; 1] = if low_threshold <= pixel[0] && pixel[0] <= high_threshold {
        [255; 1]
    } else {
        [0; 1]
    };
    image::Luma(value)
}

fn convert_gray_to_rgb_pixel(pixel: &Luma<u8>) -> Rgb<u8> {
    Rgb([pixel[0], pixel[0], pixel[0]])
}

fn convert_gray_to_r_pixel(pixel: &Luma<u8>) -> Rgb<u8> {
    Rgb([pixel[0], 0, 0])
}

fn convert_gray_to_g_pixel(pixel: &Luma<u8>) -> Rgb<u8> {
    Rgb([0, pixel[0], 0])
}

fn convert_gray_to_b_pixel(pixel: &Luma<u8>) -> Rgb<u8> {
    Rgb([0, 0, pixel[0]])
}
