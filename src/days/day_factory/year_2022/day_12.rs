use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

pub struct Day12{}

impl Day for Day12 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        Ok(ipr.fullname()?)
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        Ok(ipr.fullname()?)
    }
}