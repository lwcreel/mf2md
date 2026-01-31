use std::{error::Error, fs::File, process};

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
struct ExerciseRecord {
    #[serde(rename = "Weight", default = "default_weight")]
    weight: u16,
    #[serde(skip)]
    num_sets: u8,
    #[serde(rename = "RIR", default = "default_rir")]
    rir: u8,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Exercise")]
    exercise: String,
}

fn default_weight() -> u16 {
    199
}

fn default_rir() -> u8 {
    5
}

fn read_mf_csv() -> Result<(), Box<dyn Error>> {
    let file = File::open("resources/sample.csv")?;
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
