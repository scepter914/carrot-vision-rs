extern crate image;
// extern crate imageproc;

use riprs::object_detection::rgb_threshold;
use riprs::riprs_cv_lib::logger;

fn main() {
    let logger = logger::Logger::new("data/result/");

    let input_image_path = "data/ball_1.jpg";
    // let path_image = "data/ball_2.jpg";
    let input_image = image::open(input_image_path).unwrap().to_rgb8();

    let rgb_threshold = rgb_threshold::RGBThreshold {
        r_high: 255,
        r_low: 0,
        g_high: 255,
        g_low: 0,
        b_high: 45,
        b_low: 0,
    };
    let binarized_image = rgb_threshold::binalize_rgb(&input_image, &rgb_threshold);
    binarized_image
        .save(logger.get_full_path("result.png"))
        .unwrap();
    let rgb_disassembled_image = rgb_threshold::combine_rgb_disassembled_image(&input_image);
    rgb_disassembled_image
        .save(logger.get_full_path("result_rgb_layer.png"))
        .unwrap();

    // r_image.save(logger.get_full_path("result_r.png")).unwrap();
    // g_image.save(logger.get_full_path("result_g.png")).unwrap();
    // b_image.save(logger.get_full_path("result_b.png")).unwrap();
}
