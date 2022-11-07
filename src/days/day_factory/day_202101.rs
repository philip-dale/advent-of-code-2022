use std::error::Error;
use super::input_reader;
use crate::days::day_factory::Day;

pub struct Day202101{}

impl Day for Day202101 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<u64> = ipr.vec_1d()?;
        return Ok(data.len().to_string());
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<u64> = ipr.vec_1d()?;
        return Ok(data.len().to_string());
    }
}



