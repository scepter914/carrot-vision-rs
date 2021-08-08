extern crate chrono;
extern crate env_logger;

use chrono::Local;
use env_logger::fmt::Color;
use log::{debug, error, info, trace, warn, Level};
use std::io::Write;

pub struct Logger {
    logger_directory_path: String,
}

impl Logger {
    pub fn init(directory_path: impl Into<String>) -> Logger {
        let logger = Logger {
            logger_directory_path: directory_path.into(),
        };
        logger.logger_init();
        return logger;
    }

    fn logger_init(&self) -> () {
        env_logger::Builder::from_default_env()
            .format(|buf, record| {
                let level_color = match record.level() {
                    Level::Trace => Color::White,
                    Level::Debug => Color::Blue,
                    Level::Info => Color::Green,
                    Level::Warn => Color::Yellow,
                    Level::Error => Color::Red,
                };
                let mut level_style = buf.style();
                level_style.set_color(level_color);
                let ts = buf.timestamp();

                writeln!(
                    buf,
                    "[{time_stamp} {level} {target}] [{file} {line}] {args}",
                    time_stamp = ts,
                    level = level_style.value(record.level()),
                    target = record.target(),
                    file = &record.file().unwrap_or("____unknown")[4..],
                    // src/file.rs -> file.rs
                    line = record.line().unwrap_or(0),
                    args = record.args(),
                )
            })
            .init();
    }

    pub fn get_full_path(&self, file_path: impl Into<String>) -> String {
        return format!("{}{}", self.logger_directory_path, file_path.into());
    }

    pub fn get_time_path(
        &self,
        file_path: impl Into<String>,
        extension: impl Into<String>,
    ) -> String {
        let now_time = Local::now().format("%Y%m%d_%H%M%S").to_string();
        return format!(
            "{}{}{}.{}",
            self.logger_directory_path,
            file_path.into(),
            now_time,
            extension.into(),
        );
    }
}
