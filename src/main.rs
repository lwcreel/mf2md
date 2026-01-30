use csv::Reader;
use serde::Deserialize;
use std::{error::Error, fs::File, process};

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
struct ExerciseRecord {
    #[serde(rename = "Weight")]
    weight: u16,
    #[serde(skip_deserializing)]
    num_sets: u8,
    #[serde(rename = "RIR")]
    rir: u8,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Exercise")]
    exercise: String,
}

fn read_mf_csv() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("resources/sample.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let exercise_record: ExerciseRecord = result?;
        println!("{:?}", exercise_record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_mf_csv() {
        println!("Error Running Example: {}", err);
        process::exit(1);
    }
}
