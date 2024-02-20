use image::imageops;
use image::GrayImage;
use imageproc::drawing;
use imageproc::point::Point;
use imageproc::rect::Rect;

use log::debug;
use log::LevelFilter;
use log::{log_enabled, Level};
use std::env;

use carrot_image_utils::geometry::rect;
use carrot_image_utils::rgb_image::draw;
use carrot_image_utils::rgb_image::file;
use carrot_image_utils::rgb_image::rgb_to_binary;
use carrot_utils::benchmark::Benchmark;
use carrot_utils::logger::Logger;
use carrot_utils::path::format_time_macro;

use object_detection::object_detection::threshold_detection;

fn set_log_level(log_level: &str) -> LevelFilter {
    match &log_level[..] {
        "Error" => LevelFilter::Error,
        "Warn" => LevelFilter::Warn,
        "Info" => LevelFilter::Info,
        "Debug" => LevelFilter::Debug,
        "Trace" => LevelFilter::Trace,
        _ => LevelFilter::Error,
    }
}

fn main() {
    // Set logger and benchmark
    let args: Vec<String> = env::args().collect();
    let log_level_input: &str = if args.len() < 2 { "Error" } else { &args[1] };
    let logger_level = set_log_level(log_level_input);
    let logger = Logger::new(
        "./data/result/{TIME_SEC}",
        "{TIME_SEC}.txt",
        logger_level,
        logger_level,
    );
    let mut benchmark: Benchmark = Benchmark::new();

    // Set input
    let input_image_path = "./data/ball.jpg";
    let input_image_ = image::open(input_image_path).unwrap().to_rgb8();
    let input_image = imageops::resize(&input_image_, 1280, 720, imageops::Gaussian);

    // Threshold detection
    let rgb_threshold = rgb_to_binary::RGBThreshold {
        high_threshold: image::Rgb([255, 255, 80]),
        low_threshold: image::Rgb([100, 0, 0]),
    };
    if log_enabled!(Level::Debug) {
        benchmark.start();
    }

    let binarized_image: GrayImage =
        rgb_to_binary::convert_by_threshold(&input_image, &rgb_threshold);
    let cg: Point<f32> = threshold_detection::get_cg_from_binary(&binarized_image);

    debug!("{}", benchmark.stop());

    // Debug image
    if log_enabled!(Level::Debug) {
        let mut cg_image = draw::draw_point(&input_image, cg, image::Rgb([0, 0, 255]));

        let rect_: Rect = rect::make_rect_from_chw(cg, 480, 480);
        cg_image = drawing::draw_hollow_rect(&cg_image, rect_, image::Rgb([0, 0, 255]));
        let file_path = logger.log_directory_path.join(format_time_macro(
            "threshold_detection_{TIME_MS}.ppm".to_string(),
        ));
        file::save_ppm_file(&cg_image, file_path.as_path()).unwrap();
    }

    // If log level is Trace, then save the image of threshold rgb debug image
    if log_enabled!(Level::Trace) {
        let rgb_disassembled_image =
            rgb_to_binary::get_rgb_threshold_debug_image(&input_image, &rgb_threshold);
        let rgb_disassembled_image_path = logger.log_directory_path.join("result_rgb_layer.png");
        rgb_disassembled_image
            .save(rgb_disassembled_image_path)
            .unwrap();
    }
}
