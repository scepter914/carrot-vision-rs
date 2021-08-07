extern crate chrono;

use chrono::Local;

pub struct Logger {
    logger_directory_path: String,
}

impl Logger {
    pub fn new(directory_path: impl Into<String>) -> Logger {
        Logger {
            logger_directory_path: directory_path.into(),
        }
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
