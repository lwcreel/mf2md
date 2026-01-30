use csv::Reader;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
struct Workout {
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

fn main() {
    println!("Hello, world!");
}
