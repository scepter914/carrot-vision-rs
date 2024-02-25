use image::imageops;
use image::RgbImage;

pub fn concat_matrix_images(
    image_matrix: &Vec<Vec<RgbImage>>,
    width: u32,
    height: u32,
) -> RgbImage {
    let mut row_images: Vec<RgbImage> = vec![];
    for image_col in image_matrix {
        row_images.push(concat_col_images(image_col, width, height))
    }
    concat_row_images(&row_images, row_images[0].width(), height)
}

pub fn concat_col_images(images: &Vec<RgbImage>, width: u32, height: u32) -> RgbImage {
    let image_number = images.len();
    let mut combined_image = RgbImage::new(image_number as u32 * width, height);
    for k in 0..image_number {
        let image = imageops::resize(&images[k], width, height, imageops::Gaussian);
        for i in 0..width {
            for j in 0..height {
                let pixel = image.get_pixel(i, j);
                let x_value = k as u32 * width + i;
                combined_image.put_pixel(x_value, j, *pixel);
            }
        }
    }
    combined_image
}

pub fn concat_row_images(images: &Vec<RgbImage>, width: u32, height: u32) -> RgbImage {
    let image_number = images.len();
    let mut combined_image = RgbImage::new(width, image_number as u32 * height);
    for k in 0..image_number {
        let image = imageops::resize(&images[k], width, height, imageops::Gaussian);
        for i in 0..width {
            for j in 0..height {
                let pixel = image.get_pixel(i, j);
                let y_value = k as u32 * height + j;
                combined_image.put_pixel(i, y_value, *pixel);
            }
        }
    }
    combined_image
}
