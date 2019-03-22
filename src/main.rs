use std::{error::Error, fs::File};

fn main() {
    match create_csv() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Failed to process file {}", err);
        }
    };
}

fn create_csv() -> Result<(), Box<Error>> {
    // Open the file.
    let _file = File::open("running_paces.csv")?;

    Ok(())
}
