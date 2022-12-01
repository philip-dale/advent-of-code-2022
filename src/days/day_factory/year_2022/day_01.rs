use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

#[cfg(windows)]
const GROUP_ENDING: & str = "\r\n\r\n";
#[cfg(not(windows))]
const GROUP_ENDING: & str = "\n\n";

struct Calories {
    total: Vec<u64>,
}

impl Calories {
    pub fn top_n(&self, items: usize) -> u64 {
        let mut vals: Vec<u64> = vec![0; items];

        for v in &self.total {
            let mut val = *v;
            
            for stored in vals.iter_mut().rev() {
                if val > *stored {

                    val = std::mem::replace(stored, val)
                }
            }
        }
        return vals.iter().sum();
    }
}

impl std::str::FromStr for Calories {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cal = Calories {
            total: Vec::new(),
        };

        for g in s.split(GROUP_ENDING).collect::<Vec<&str>>() {
            cal.total.push(0);
            for l in g.lines().collect::<Vec<&str>>() {
                *cal.total.last_mut().unwrap() += l.parse::<u64>()?;
            }
        }
        Ok(cal)
    }
}

pub struct Day01{}


impl Day for Day01 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Calories = ipr.whole()?;
        Ok(data.total.iter().max().unwrap().to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Calories = ipr.whole()?;
        Ok(data.top_n(3).to_string())
    }
}