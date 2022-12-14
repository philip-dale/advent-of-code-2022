use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

pub struct Day01{}

impl Day for Day01 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<u64> = ipr.vec_1d_newln()?;
        let mut last = data[0];
        let mut count = 0;
        for p in data{
            if p > last {
                count += 1;
            }
            last = p;
        }

        Ok(count.to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<u64> = ipr.vec_1d_newln()?;
        let mut last = data[0] + data[1] + data[2];
        let mut count = 0;
        for p in 3..data.len()-2 {
            let val = data[p] + data[p+1] + data[p+2];
            if val > last {
                count += 1;
            }
            last = val;
        }
        Ok(count.to_string())
    }
}



