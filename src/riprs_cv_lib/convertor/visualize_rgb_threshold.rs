use image::RgbImage;

use crate::riprs_cv_lib::convertor::{gray_convertor, rgb_convertor};

// debug image
// rgb: rgb, binary
// r: gray, binary
// g: gray, binary
// b: gray, binary
pub fn get_rgb_threshold_debug_image(
    image: &RgbImage,
    rgb_threshold: &rgb_convertor::RGBThreshold,
) -> RgbImage {
    let width = image.width();
    let height = image.height();
    let mut combined_image = image::RgbImage::new(2 * width, 4 * height);

    let binarized_image_by_rgb_threshold =
        rgb_convertor::image_rgb_to_binary_by_rgb_threshold(image, rgb_threshold);
    let rgb_binarized_image_by_rgb_threshold =
        gray_convertor::image_gray_to_rgb(&binarized_image_by_rgb_threshold);

    // r
    let gray_r_image = rgb_convertor::image_r_to_gray(&image);
    let binary_r_image = rgb_convertor::image_r_to_binary(&image, rgb_threshold);

    // g
    let gray_g_image = rgb_convertor::image_g_to_gray(&image);
    let binary_g_image = rgb_convertor::image_g_to_binary(&image, rgb_threshold);

    // b
    let gray_b_image = rgb_convertor::image_b_to_gray(&image);
    let binary_b_image = rgb_convertor::image_b_to_binary(&image, rgb_threshold);

    for i in 0..width {
        for j in 0..height {
            // input image
            let pixel = image.get_pixel(i, j);
            combined_image.put_pixel(i, j, *pixel);

            // binalized image
            let pixel = rgb_binarized_image_by_rgb_threshold.get_pixel(i, j);
            combined_image.put_pixel(width + i, j, *pixel);

            // r, r thres
            let pixel = gray_r_image.get_pixel(i, j);
            combined_image.put_pixel(i, height + j, gray_convertor::pixel_gray_to_rgb(&pixel));
            let pixel = binary_r_image.get_pixel(i, j);
            combined_image.put_pixel(
                width + i,
                height + j,
                gray_convertor::pixel_gray_to_rgb(&pixel),
            );

            // g, g thres
            let pixel = gray_g_image.get_pixel(i, j);
            combined_image.put_pixel(i, 2 * height + j, gray_convertor::pixel_gray_to_rgb(&pixel));
            let pixel = binary_g_image.get_pixel(i, j);
            combined_image.put_pixel(
                width + i,
                2 * height + j,
                gray_convertor::pixel_gray_to_rgb(&pixel),
            );

            // b, b thres
            let pixel = gray_b_image.get_pixel(i, j);
            combined_image.put_pixel(i, 3 * height + j, gray_convertor::pixel_gray_to_rgb(&pixel));
            let pixel = binary_b_image.get_pixel(i, j);
            combined_image.put_pixel(
                width + i,
                3 * height + j,
                gray_convertor::pixel_gray_to_rgb(&pixel),
            );
        }
    }
    return combined_image;
}
