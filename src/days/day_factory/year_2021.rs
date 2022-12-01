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
pub mod day_13;
pub mod day_14;
pub mod day_15;

pub fn get_day(day: String) -> Result<Box<dyn Day>, Box<dyn Error>> {
    match day.as_str() {
        "01" => Ok(Box::new(day_01::Day01{})),
        "02" => Ok(Box::new(day_02::Day02{})),
        "03" => Ok(Box::new(day_03::Day03{})),
        "04" => Ok(Box::new(day_04::Day04{})),
        "05" => Ok(Box::new(day_05::Day05{})),
        "06" => Ok(Box::new(day_06::Day06{})),
        "07" => Ok(Box::new(day_07::Day07{})),
        "08" => Ok(Box::new(day_08::Day08{})),
        "09" => Ok(Box::new(day_09::Day09{})),
        "10" => Ok(Box::new(day_10::Day10{})),
        "11" => Ok(Box::new(day_11::Day11{})),
        "12" => Ok(Box::new(day_12::Day12{})),
        "13" => Ok(Box::new(day_13::Day13{})),
        "14" => Ok(Box::new(day_14::Day14{})),
        "15" => Ok(Box::new(day_15::Day15{})),
        _ => Err(Box::new(DayError("Unknown Day 2021".into()))),
    }
}