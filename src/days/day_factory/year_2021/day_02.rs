use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types as DayTypes;

pub struct Day02{}

struct Pos1 {
    horizontal: u64,
    depth: u64,
}

struct Pos2 {
    horizontal: u64,
    depth: u64,
    aim: u64,
}

impl Day for Day02 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<DayTypes::StrU64> = ipr.vec_1d_newln()?;

        let mut pos = Pos1{horizontal: 0, depth: 0};
        for m in data {
            if m.dir == "down" {
                pos.depth += m.val;
            } else if m.dir == "up" {
                pos.depth -= m.val;
            } else {
                pos.horizontal += m.val;
            }
        }
        Ok((pos.depth * pos.horizontal).to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<DayTypes::StrU64> = ipr.vec_1d_newln()?;

        let mut pos = Pos2{horizontal: 0, depth: 0, aim: 0};
        for m in data {
            if m.dir == "down" {
                pos.aim += m.val;
            } else if m.dir == "up" {
                pos.aim -= m.val;
            } else {
                pos.horizontal += m.val;
                pos.depth += pos.aim * m.val;
            }
        }
        Ok((pos.depth * pos.horizontal).to_string())
    }
}



