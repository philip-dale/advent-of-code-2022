use std::error::Error;
use super::{Day, DayError};

pub mod day_01;
pub mod day_02;
pub mod day_03;

pub fn get_day(day: String) -> Result<Box<dyn Day>, Box<dyn Error>> {
    match day.as_str() {
        "01" => return Ok(Box::new(day_01::Day01{})),
        "02" => return Ok(Box::new(day_02::Day02{})),
        "03" => return Ok(Box::new(day_03::Day03{})),
        _ => return Err(Box::new(DayError("Unknown Day 2021".into()))),
    }
}