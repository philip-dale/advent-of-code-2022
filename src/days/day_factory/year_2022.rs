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
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;

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
        "13" => return Ok(Box::new(day_13::Day13{})),
        "14" => return Ok(Box::new(day_14::Day14{})),
        "15" => return Ok(Box::new(day_15::Day15{})),
        "16" => return Ok(Box::new(day_16::Day16{})),
        "17" => return Ok(Box::new(day_17::Day17{})),
        "18" => return Ok(Box::new(day_18::Day18{})),
        "19" => return Ok(Box::new(day_19::Day19{})),
        "20" => return Ok(Box::new(day_20::Day20{})),
        "21" => return Ok(Box::new(day_21::Day21{})),
        "22" => return Ok(Box::new(day_22::Day22{})),
        "23" => return Ok(Box::new(day_23::Day23{})),
        "24" => return Ok(Box::new(day_24::Day24{})),
        _ => return Err(Box::new(DayError("Unknown Day 2022".into()))),
    }
}