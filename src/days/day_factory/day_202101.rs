use std::error::Error;
use super::input_reader;
use crate::days::day_factory::Day;

pub struct Day202101{}

impl Day for Day202101 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<u64> = ipr.vec_1d()?;
        let mut last = data[0];
        let mut count = 0;
        for p in 1..data.len() {
            if data[p] > last {
                count += 1;
            }
            last = data[p];
        }

        return Ok(count.to_string());
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<u64> = ipr.vec_1d()?;
        let mut last = data[0] + data[1] + data[2];
        let mut count = 0;
        for p in 3..data.len()-2 {
            let val = data[p] + data[p+1] + data[p+2];
            if val > last {
                count += 1;
            }
            last = val;
        }
        return Ok(count.to_string());
    }
}



