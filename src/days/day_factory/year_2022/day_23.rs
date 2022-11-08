use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

pub struct Day23{}

impl Day for Day23 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        return Ok(ipr.fullname()?);
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        return Ok(ipr.fullname()?);
    }
}