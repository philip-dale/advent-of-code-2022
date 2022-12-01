use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

struct Calories {
    items: Vec<Vec<u64>>,
    total: Vec<u64>,
}

impl Calories {
    pub fn top_n(&self, items: usize) -> u64 {
        let mut vals: Vec<u64> = vec![0; items];

        for v in &self.total {
            let mut val = *v;
            
            for stored in vals.iter_mut().rev() {
                if val > *stored {
                    let tmp = val;
                    val = *stored;
                    *stored = tmp;
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
            items: Vec::new(),
            total: Vec::new(),
        };
        let ls:Vec<&str> = s.lines().collect();
        let mut total = 0;
        let mut items: Vec<u64> = Vec::new();

        for l in ls {
            if l.len() > 0 {
                let val = l.parse()?;
                items.push(val);
                total += val;
            } else {
                cal.items.push(items);
                items = Vec::new();
                cal.total.push(total);
                total = 0;
            }
        }

        if items.len() > 0 {
            cal.items.push(items);
            cal.total.push(total);
        }

        return Ok(cal);
    }
}

pub struct Day01{}


impl Day for Day01 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Calories = ipr.whole()?;
        return Ok(data.total.iter().max().unwrap().to_string());
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Calories = ipr.whole()?;
        return Ok(data.top_n(3).to_string());
    }
}