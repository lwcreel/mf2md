use std::{collections::HashMap, error::Error, fs::File, process};

#[derive(Debug, serde::Deserialize, PartialEq, Eq, Clone)]
struct ExerciseRecord {
    #[serde(rename = "Weight", default = "default_weight")]
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
    let mut res = Vec::new();
    let file = File::open("resources/sample.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let exercise_record: ExerciseRecord = result?;
        res.push(exercise_record);
    }

    let mut output = HashMap::new();

    for row in res {
        let exercise_name = row.exercise.to_string();

        let mut values = String::from("\n");
        values.push_str(row.weight.to_string().as_str());
        values.push_str("x");
        values.push_str(row.reps.to_string().as_str());
        values.push_str(" (");
        values.push_str(row.rir.to_string().as_str());
        values.push_str(" RIR)");

        if output.contains_key(exercise_name.as_str()) {
            let mut t = String::from(output.get(exercise_name.as_str()).unwrap());
            t.push_str(values.as_str());
            output.insert(exercise_name, t);
        } else {
            output.insert(exercise_name, values);
        }
    }

    for (exercise, values) in output {
        println!("{exercise}: {values}");
    }

    Ok(())
}

fn main() {
    if let Err(err) = read_mf_csv() {
        println!("Error Running Converter: {}", err);
        process::exit(1);
    }
}
