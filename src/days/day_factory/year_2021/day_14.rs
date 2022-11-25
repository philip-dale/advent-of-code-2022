use std::error::Error;
use std::collections::HashMap;

use crate::input_reader;
use crate::days::day_factory::Day;

struct Polymer {
    val: String,
    instructions: HashMap<String, String>,
}

impl Polymer {
    pub fn step(& mut self) {
        let mut insert: Vec<String> = Vec::new();
        for i in 0..self.val.len()-1 {
            insert.push(self.instructions.get(&self.val[i..i+2]).unwrap().to_string());
        }
        for i in (0..self.val.len()-1).rev() {
            self.val.insert_str(i+1, &insert[i]);
        }
    }

    pub fn steps(& mut self, count: usize) {
        for i in 0..count {
            println!("{}", i);
            self.step();
        }
    }

    pub fn result(&self) -> usize{
        let mut poly_str: Vec<char> = self.val.chars().collect();
        poly_str.sort();
        let mut min_val = poly_str.len();
        let mut max_val = 0;

        let mut current = poly_str[0];
        let mut current_count = 0;
        for c in poly_str{
            if c == current {
                current_count += 1;
            } else {
                if current_count > max_val {
                    max_val = current_count;
                }
                if current_count < min_val {
                    min_val = current_count;
                }
                current = c;
                current_count = 1;
            }
        }
        return max_val - min_val;
    }
}

impl std::str::FromStr for Polymer {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = Polymer{
            val: String::from(""),
            instructions: HashMap::new(),
        };
        for (i, l) in s.lines().enumerate(){
            if i == 0 {
                p.val = l.to_string();
            } else if i > 1 {
                let s:Vec<&str> = l.split(" -> ").collect();
                p.instructions.insert(s[0].trim().to_string(), s[1].trim().to_string());
            }
        }
        return Ok(p);
    }
}

pub struct Day14{}

impl Day for Day14 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut poly: Polymer = ipr.whole()?;
        poly.steps(10);
        return Ok(poly.result().to_string());
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut poly: Polymer = ipr.whole()?;
        poly.steps(40);
        return Ok(poly.result().to_string());
    }
}