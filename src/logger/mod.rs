use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::io;
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
    let file_name: String = format!("logs/{}.log", curr.format("%d-%m-%y"));
    let path_string: PathBuf = get_directory().join(format!("/{file_name}"));

    let file: Result<File, io::Error> = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name);

    match file {
        Ok(file) => file,
        Err(err) => {
            panic!("{:?}", err);
        }
    }

}

pub fn get_directory() -> PathBuf {
    current_dir().expect("could not get current dir").join("logs")
}