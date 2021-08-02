extern crate image;
// extern crate imageproc;

use imageproc::drawing::draw_filled_circle;
use imageproc::point::Point;
use riprs::object_detection::threshold_detection;
use riprs::riprs_cv_lib::convertor::rgb_to_binary;
use riprs::riprs_cv_lib::image::rgb_image;
use riprs::riprs_cv_lib::logger;

fn main() {
    let logger = logger::Logger::new("data/result/");

    let input_image_path = "data/ball_1.jpg";
    // let path_image = "data/ball_2.jpg";
    let input_image = image::open(input_image_path).unwrap().to_rgb8();

    let rgb_threshold = rgb_to_binary::RGBThreshold {
        high_threshold: image::Rgb([255, 255, 80]),
        low_threshold: image::Rgb([180, 0, 0]),
    };

    let binarized_image =
        rgb_to_binary::convert_to_binary_image_by_threshold(&input_image, &rgb_threshold);
    let cg: Point<f32> = threshold_detection::get_cg_from_binary(&binarized_image);

    let cg_image = draw_filled_circle(
        &input_image,
        (cg.x as i32, cg.y as i32),
        10,
        image::Rgb([255, 0, 0]),
    );
    println!("cg.x: {}, cg.y: {}", cg.x, cg.y);
    rgb_image::print_pixel(&input_image.get_pixel(cg.x as u32, cg.y as u32));
    cg_image.save(logger.get_full_path("result.png")).unwrap();
    let rgb_disassembled_image =
        rgb_to_binary::get_rgb_threshold_debug_image(&input_image, &rgb_threshold);
    rgb_disassembled_image
        .save(logger.get_full_path("result_rgb_layer.png"))
        .unwrap();

    // r_image.save(logger.get_full_path("result_r.png")).unwrap();
    // g_image.save(logger.get_full_path("result_g.png")).unwrap();
    // b_image.save(logger.get_full_path("result_b.png")).unwrap();
}
