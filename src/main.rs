use csv::Writer;
use std::{
    error::Error,
    fs::{remove_file, OpenOptions},
    io,
    path::Path,
};

pub const EXPORT_FILE_NAME: &str = "running_paces.csv";

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
    let writer = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(EXPORT_FILE_NAME)?;

    // Create the CSV writer to save the output.
    let mut csv_writer = csv::WriterBuilder::new().from_writer(writer);

    // Headers.
    write_header(&mut csv_writer)?;

    for seconds in (120..601_u16).rev() {
        write_record(&mut csv_writer, seconds)?;
    }

    Ok(())
}

fn write_record<W>(csv_writer: &mut Writer<W>, second: u16) -> Result<(), Box<Error>>
where
    W: io::Write,
{
    // ToDo, take the seconds and work out all the things.
    csv_writer.write_record(&[
        second.to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    ])?;

    Ok(())
}

fn write_header<W>(csv_writer: &mut Writer<W>) -> Result<(), Box<Error>>
where
    W: io::Write,
{
    csv_writer.write_record(&[
        "KM Pace",
        "KPH",
        "Mile Pace",
        "MPH",
        "5k",
        "5m",
        "10k",
        "15k",
        "10m",
        "20k",
        "Half",
        "15m",
        "25k",
        "20m",
        "Marathon",
        "50k",
        "100k",
        "100m",
    ])?;

    Ok(())
}
