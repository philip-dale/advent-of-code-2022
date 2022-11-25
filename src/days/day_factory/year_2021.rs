use std::error::Error;
use super::{Day, DayError};

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;

pub fn get_day(day: String) -> Result<Box<dyn Day>, Box<dyn Error>> {
    match day.as_str() {
        "01" => return Ok(Box::new(day_01::Day01{})),
        "02" => return Ok(Box::new(day_02::Day02{})),
        "03" => return Ok(Box::new(day_03::Day03{})),
        "04" => return Ok(Box::new(day_04::Day04{})),
        "05" => return Ok(Box::new(day_05::Day05{})),
        "06" => return Ok(Box::new(day_06::Day06{})),
        "07" => return Ok(Box::new(day_07::Day07{})),
        "08" => return Ok(Box::new(day_08::Day08{})),
        "09" => return Ok(Box::new(day_09::Day09{})),
        "10" => return Ok(Box::new(day_10::Day10{})),
        "11" => return Ok(Box::new(day_11::Day11{})),
        "12" => return Ok(Box::new(day_12::Day12{})),
        _ => return Err(Box::new(DayError("Unknown Day 2021".into()))),
    }
}