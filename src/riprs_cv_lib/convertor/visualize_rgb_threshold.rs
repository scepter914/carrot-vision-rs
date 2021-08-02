use image::RgbImage;

use riprs_cv_lib::convertor::rgb_convertor;

// debug image
// rgb: rgb, binary
// r: gray, binary
// g: gray, binary
// b: gray, binary
pub fn get_rgb_threshold_debug_image(image: &RgbImage, rgb_threshold: &RGBThreshold) -> RgbImage {
    let width = image.width();
    let height = image.height();
    let mut combined_image = image::RgbImage::new(2 * width, 4 * height);

    let rgb_binarized_image =
        rgb_convertor::image_rgb_to_binary_by_rgb_threshold(image, rgb_threshold);

    // r
    let gray_r_image = image_r_to_gray(&image);
    let rgb_r_image = image_gray_to_rgb(&gray_r_image);
    let binary_r_image = image_r_to_binary(&image, rgb_threshold);

    // g
    let gray_g_image = image_g_to_gray(&image);
    let rgb_g_image = image_gray_to_rgb(&gray_g_image);
    let binary_g_image = image_g_to_binary(&image, rgb_threshold);

    // b
    let gray_b_image = image_b_to_gray(&image);
    let rgb_b_image = image_gray_to_rgb(&gray_b_image);
    let binary_b_image = image_b_to_binary(&image, rgb_threshold);

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
            let pixel = binary_r_image.get_pixel(i, j);
            combined_image.put_pixel(width + i, height + j, *pixel);

            // g, g thres
            let pixel = rgb_g_image.get_pixel(i, j);
            combined_image.put_pixel(i, 2 * height + j, *pixel);
            let pixel = binary_g_image.get_pixel(i, j);
            combined_image.put_pixel(width + i, 2 * height + j, *pixel);

            // b, b thres
            let pixel = rgb_b_image.get_pixel(i, j);
            combined_image.put_pixel(i, 3 * height + j, *pixel);
            let pixel = binary_b_image.get_pixel(i, j);
            combined_image.put_pixel(width + i, 3 * height + j, *pixel);
        }
    }
    return combined_image;
}
