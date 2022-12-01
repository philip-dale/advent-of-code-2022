use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

pub struct Day08{}

struct LcdInput {
    pub samples: Vec<String>,
    pub samples_nos: Vec<usize>,
    pub actuals: Vec<String>,
    pub actual_nos: Vec<usize>,
}

const UNSET_MAP: usize = 10;

impl LcdInput {
    pub fn new() -> Self {
        LcdInput {
            samples: Vec::new(),
            samples_nos: Vec::new(),
            actuals: Vec::new(),
            actual_nos: Vec::new(),
        }
    }

    pub fn count_after_dedupe(main: &String, test: &str) -> usize {
        let mut count = main.len();
        for c in main.chars() {
            if test.contains(c) {
                count -= 1;
            }
        }
        count
    }

    pub fn sample_compare(a: &String, b: &String) -> bool{
        if a.len() != b.len() {
            return false;
        }

        for c in a.chars() {
            if !b.contains(c) {
                return false;
            }
        }
        true
    }

    pub fn find_sample_index(&self, n: usize) -> usize {
        for (i, s) in self.samples_nos.iter().enumerate() {
            if *s == n {
                return i;
            }
        }
        UNSET_MAP
    }

    pub fn parse1(&mut self) {
        for (i, s) in self.samples.iter().enumerate() {
            let val = match s.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                _ => continue,
            };
            self.samples_nos[i] = val;
        }
    }

    pub fn parse2(&mut self) {
        let one_val = &self.samples[self.find_sample_index(1)];
        let four_val = &self.samples[self.find_sample_index(4)];
        for (i, s) in self.samples.iter().enumerate() {
            if s.len() == 5 {
                if LcdInput::count_after_dedupe(s, one_val) == 3{
                    self.samples_nos[i] = 3;
                    continue;
                }
                if LcdInput::count_after_dedupe(s, four_val) == 3{
                    self.samples_nos[i] = 2;
                    continue;
                }
                self.samples_nos[i] = 5;
            }
        }
    }

    pub fn parse3(&mut self) {
        
        let four_val = &self.samples[self.find_sample_index(4)];
        let five_val = &self.samples[self.find_sample_index(5)];
        let seven_val = &self.samples[self.find_sample_index(7)];
        for (i, s) in self.samples.iter().enumerate() {
            if s.len() == 6 {
                if LcdInput::count_after_dedupe(s, &(seven_val.to_string() + &four_val[..])) == 1{
                    self.samples_nos[i] = 9;
                    continue;
                }
                if LcdInput::count_after_dedupe(s, five_val) == 2{
                    self.samples_nos[i] = 0;
                    continue;
                }
                self.samples_nos[i] = 6;
            }
        }
    }

    pub fn set_actuals(&mut self) {
        for (ia, a) in self.actuals.iter().enumerate() {
            for (is, s) in self.samples.iter().enumerate() {
                if LcdInput::sample_compare(a, s) {
                    self.actual_nos[ia] = self.samples_nos[is];
                }
            }
        }
    }

    pub fn get_val(&self) -> usize {
        (self.actual_nos[0] * 1000) +
        (self.actual_nos[1] * 100) + 
        (self.actual_nos[2] * 10) +
        self.actual_nos[3]
    }

}

impl std::str::FromStr for LcdInput {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lcd = LcdInput::new();
        let sections:Vec<&str> = s.split('|').collect();

        let samples = sections[0].split_whitespace();
        let actuals = sections[1].split_whitespace();

        for s in samples {
            lcd.samples.push(s.to_string());
        }
        lcd.samples_nos = vec![UNSET_MAP; lcd.samples.len()];
        
        for a in actuals {
            lcd.actual_nos.push(UNSET_MAP);
            lcd.actuals.push(a.to_string());
        }

        Ok(lcd)
    }
}

impl Day for Day08 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut data: Vec<LcdInput> = ipr.vec_1d_newln()?;

        for d in &mut data {
            d.parse1();
            d.set_actuals();
        }

        let mut count = 0;
        for d in data {
            for an in d.actual_nos {
                if an != UNSET_MAP {
                    count += 1;
                }
            }
        }
        Ok(count.to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut data: Vec<LcdInput> = ipr.vec_1d_newln()?;

        for d in &mut data {
            d.parse1();
            d.parse2();
            d.parse3();
            d.set_actuals();
        }

        let mut count = 0;
        for d in data {
            count += d.get_val();
        }
        Ok(count.to_string())
    }
}