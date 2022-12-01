use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

pub struct Day07{}

fn calc_diffs(data: &Vec<usize>, target: usize) -> usize{
    let mut total = 0;
    for v in data {
        
        if v > &target {
            total += v-target;
        } else {
            total += target-v;
        };
    }
    total
}

fn calc_diffs_lookup(data: &Vec<usize>, target: usize, lookup: &[usize]) -> usize{
    let mut total = 0;
    for v in data {
        
        let diff = if v > &target {
            v-target
        } else {
            target-v
        };

        total += lookup[diff];
    }
    total
}

impl Day for Day07 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut data: Vec<usize> = ipr.vec_1d_sep(&String::from(","))?;
        data.sort();
        let target = data[data.len()/2];
        Ok(calc_diffs(&data, target).to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut data: Vec<usize> = ipr.vec_1d_sep(&String::from(","))?;
        data.sort();

        let mut fact_table: Vec<usize> = vec![0; data[data.len()-1]+1];
        for f in 1..fact_table.len() {
            fact_table[f] = fact_table[f-1] + f;
        }

        let mut min_fuel = calc_diffs_lookup(&data, 0, &fact_table);
        
        for i in 1..data[data.len()-1] {
            let fuel = calc_diffs_lookup(&data, i, &fact_table);
            if fuel < min_fuel {
                min_fuel = fuel;
            }
        }
        Ok(min_fuel.to_string())
    }
}