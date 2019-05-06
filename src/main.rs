use csv::Writer;
use std::{
    error::Error,
    fs::{remove_file, OpenOptions},
    io,
    path::Path,
};

const EXPORT_FILE_NAME: &str = "running_paces.csv";
const CONVERSION: f64 = 0.621_371;

trait FormatRecord {
    fn km_pace(&self) -> String;
    fn km_per_hour(&self) -> String;
    fn mile_pace(&self) -> String;
    fn miles_per_hour(&self) -> String;
    fn distance_estimate(&self, km_dist: f64) -> String;
}

impl FormatRecord for u16 {
    fn km_pace(&self) -> String {
        let second = self % 60;
        let minute = self / 60;
        format!("0:{:02}:{:02}", minute, second)
    }

    fn km_per_hour(&self) -> String {
        let kph = 3600.0 / f64::from(*self);
        format!("{:.3}", kph)
    }

    fn mile_pace(&self) -> String {
        let kph = 3600.0 / f64::from(*self);
        let mph = kph * CONVERSION;
        let raw: f64 = 60.0 / mph;
        let second = (raw - raw.floor()) * 60.0;
        format!("0:{:02}:{:02}", raw.floor(), second.round())
    }

    fn miles_per_hour(&self) -> String {
        let kph = 3600.0 / f64::from(*self);
        let mph = kph * CONVERSION;
        format!("{:.3}", mph)
    }

    fn distance_estimate(&self, km_dist: f64) -> String {
        let total_seconds = (km_dist * f64::from(*self)) / 1000.0;
        let second = total_seconds % 60.0;
        let minute = (total_seconds / 60.0) % 60.0;
        let hour = (total_seconds / 60.0) / 60.0;

        format!(
            "{}:{:02}:{:02}",
            hour.floor(),
            minute.floor(),
            second.round()
        )
    }
}

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

    // Main records
    for km_pace in (165..601_u16).rev() {
        write_record(&mut csv_writer, km_pace)?;
    }

    Ok(())
}

fn write_record<W>(csv_writer: &mut Writer<W>, km_pace: u16) -> Result<(), Box<Error>>
where
    W: io::Write,
{
    // ToDo, take the seconds and work out all the things.
    csv_writer.write_record(&[
        km_pace.km_pace(),
        km_pace.km_per_hour(),
        km_pace.mile_pace(),
        km_pace.miles_per_hour(),
        km_pace.distance_estimate(5000_f64),
        km_pace.distance_estimate(8046.72),
        km_pace.distance_estimate(10000_f64),
        km_pace.distance_estimate(15000_f64),
        km_pace.distance_estimate(16093.4),
        km_pace.distance_estimate(20_000_f64),
        km_pace.distance_estimate(21082.41),
        km_pace.distance_estimate(24140.2),
        km_pace.distance_estimate(25_000_f64),
        km_pace.distance_estimate(32186.9),
        km_pace.distance_estimate(42164.81),
        km_pace.distance_estimate(50_000_f64),
        km_pace.distance_estimate(100_000_f64),
        km_pace.distance_estimate(160_934_f64),
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
