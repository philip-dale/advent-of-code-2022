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
        "16" => Ok(Box::new(day_16::Day16{})),
        "17" => Ok(Box::new(day_17::Day17{})),
        "18" => Ok(Box::new(day_18::Day18{})),
        "19" => Ok(Box::new(day_19::Day19{})),
        "20" => Ok(Box::new(day_20::Day20{})),
        "21" => Ok(Box::new(day_21::Day21{})),
        "22" => Ok(Box::new(day_22::Day22{})),
        "23" => Ok(Box::new(day_23::Day23{})),
        "24" => Ok(Box::new(day_24::Day24{})),
        _ => Err(Box::new(DayError("Unknown Day 2022".into()))),
    }
}