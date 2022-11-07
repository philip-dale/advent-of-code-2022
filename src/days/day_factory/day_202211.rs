use std::error::Error;
use super::input_reader;
use crate::days::day_factory::Day;

pub struct Day202211{}

impl Day for Day202211 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        return Ok(ipr.fullname()?);
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        return Ok(ipr.fullname()?);
    }
}