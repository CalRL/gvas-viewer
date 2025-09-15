use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::SystemTime;
use chrono::{DateTime, Utc};

pub fn info(message: &str) {
    let mut file: File = get_log_file();

    writeln!(file, "[INFO] {message}").expect("failed to write to log file");


}

pub fn get_log_file() -> File {
    let dir: PathBuf = get_directory();
    let curr: DateTime<Utc> = Utc::now();
    let file_name: String = format!("{}.log", curr.format("%d-%m-%y"));
    let path_string: PathBuf = get_directory().join(format!("/{file_name}"));

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path_string)
        .expect("failed to open log file...")
}

pub fn get_directory() -> PathBuf {
    current_dir().expect("could not get current dir").join("logs")
}