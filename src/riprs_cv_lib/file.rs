// extern crate chrono;
// use chrono::{Date, DateTime, Local, Utc};

pub struct FilePath {
  directory_path: String,
  file_path: String,
}

impl FilePath {
  pub fn new(directory_path: impl Into<String>, file_path: impl Into<String>) -> FilePath {
    FilePath {
      directory_path: directory_path.into(),
      file_path: file_path.into(),
    }
  }

  //pub fn new_from_time(directory_path: impl Into<String>) -> FilePath {
  //  file_path =
  //  FilePath {
  //    directory_path: directory_path.into(),
  //    file_path:
  //  }
  //}

  pub fn get_file_path(&self) -> String {
    return format!("{}{}", self.directory_path, self.file_path);
  }
}
