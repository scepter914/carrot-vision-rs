use image::imageops;
use image::{GrayImage, RgbImage};
use imageproc::point::Point;
use log::debug;
use log::LevelFilter;
use log::{log_enabled, Level};
use std::env;

use carrot_cv_utils::image::convertor::rgb_to_binary;
use carrot_cv_utils::image::rgb_image::debug;
use carrot_cv_utils::image::rgb_image::file;
use carrot_cv_utils::util::benchmark::Benchmark;
use carrot_cv_utils::util::logger::Logger;
use carrot_cv_utils::util::path;

use realtime_cv::object_detection::threshold_detection;

fn set_log_level(log_level: &str) -> LevelFilter {
    let logger_level: LevelFilter;
    match &log_level[..] {
        "Error" => logger_level = LevelFilter::Error,
        "Warn" => logger_level = LevelFilter::Warn,
        "Info" => logger_level = LevelFilter::Info,
        "Debug" => logger_level = LevelFilter::Debug,
        "Trace" => logger_level = LevelFilter::Trace,
        _ => logger_level = LevelFilter::Error,
    }
    logger_level
}

fn main() {
    // Set logger and benchmark
    let args: Vec<String> = env::args().collect();
    let log_level_input: &str;
    if args.len() < 2 {
        log_level_input = "Error";
    } else {
        log_level_input = &args[1];
    }
    let logger_level = set_log_level(log_level_input);
    let logger = Logger::init("./data/result", logger_level, logger_level);
    let mut benchmark: Benchmark = Benchmark::new();

    // Set input
    let input_image_path = "./data/ball.jpg";
    let input_image_ = image::open(input_image_path).unwrap().to_rgb8();
    let input_image = imageops::resize(&input_image_, 640, 360, imageops::Gaussian);

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

    debug!("{}", benchmark.get_bench_time());
    debug!(
        "cg: {:?}, {}",
        cg,
        debug::get_pixel_string(&input_image, cg)
    );

    // Debug image
    if log_enabled!(Level::Debug) {
        let cg_image = debug::draw_point(&input_image, &cg, image::Rgb([0, 0, 255]));
        let file_path = logger
            .root_result_directory
            .join(path::get_time_filepath("threshold_detection_", "ppm"));
        file::save_ppm_file(&cg_image, file_path.as_path()).unwrap();
    }

    // If log level is Trace, then save the image of threshold rgb debug image
    if log_enabled!(Level::Trace) {
        let rgb_disassembled_image =
            rgb_to_binary::get_rgb_threshold_debug_image(&input_image, &rgb_threshold);
        let rgb_disassembled_image_path = logger.root_result_directory.join("result_rgb_layer.png");
        rgb_disassembled_image
            .save(rgb_disassembled_image_path)
            .unwrap();
    }
}
