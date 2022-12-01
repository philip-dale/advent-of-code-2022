use std::error::Error;
use std::fmt;

use crate::input_reader;

mod types;
mod year_2021;
mod year_2022;

#[derive(Debug)]
struct DayError(String);

impl fmt::Display for DayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for DayError {}

pub trait Day {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>>;
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>>;
    fn run(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        match ipr.stage.as_str() {
            "1" => self.run1(ipr),
            "2" => self.run2(ipr),
            _ => Err(Box::new(DayError("Unknown Stage".into()))),
        }
    }
}

pub fn get_day(day: String) -> Result<Box<dyn Day>, Box<dyn Error>> {
    match &day[0..4] {
        "2021" => year_2021::get_day(day[4..6].to_string()),
        "2022" => year_2022::get_day(day[4..6].to_string()),
        _ => Err(Box::new(DayError("Unknown Year".into()))),
    }
}