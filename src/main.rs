use std::{
    error::Error,
    fs::{remove_file, OpenOptions},
    path::Path,
};

pub const EXPORT_FILE_NAME: &str = "racing_paces.csv";

fn main() {
    match create_csv() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Failed to process file {}", err);
        }
    };
}

fn create_csv() -> Result<(), Box<Error>> {
    // try and delete existing files.
    if Path::new(EXPORT_FILE_NAME).exists() {
        remove_file(EXPORT_FILE_NAME)?;
    }

    // Open the file.
    let _writer = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(EXPORT_FILE_NAME)?;

    Ok(())
}
