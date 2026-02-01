use std::{error::Error, fs::File, process};

#[derive(Debug, serde::Deserialize, PartialEq, Eq, Clone)]
struct ExerciseRecord {
    #[serde(rename = "Weight (lbs)", default = "default_weight")]
    weight: u16,
    #[serde(rename = "Reps")]
    reps: u8,
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
    let mut exercise_records = Vec::new();
    let file = File::open("resources/sample.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let exercise_record: ExerciseRecord = result?;
        exercise_records.push(exercise_record);
    }

    if exercise_records.len() == 0 {
        return Ok(());
    }

    let mut curr_date = String::from("");
    let mut curr_exercise = String::from("");

    let mut buf = String::new();
    for record in exercise_records {
        if record.date != curr_date {
            // New Date == New Workout
            buf.push_str("## ");
            curr_date = record.date.clone();
            buf.push_str(curr_date.as_str());
            buf.push_str("\n");

            // Reset the exercise in case of duplicate in new date
            curr_exercise = String::from("");
        }
        if record.exercise != curr_exercise {
            buf.push_str("### ");
            curr_exercise = record.exercise.clone();
            buf.push_str(curr_exercise.as_str());
            buf.push_str("\n");
        }

        buf.push_str("- ");
        buf.push_str(record.weight.to_string().as_str());
        buf.push_str("x");
        buf.push_str(record.reps.to_string().as_str());
        buf.push_str(" (");
        buf.push_str(record.rir.to_string().as_str());
        buf.push_str(")\n");
    }

    println!("{}", buf);

    Ok(())
}

fn main() {
    if let Err(err) = read_mf_csv() {
        println!("Error Running Converter: {}", err);
        process::exit(1);
    }
}
