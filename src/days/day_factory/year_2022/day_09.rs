use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

use std::collections::HashMap;

struct Instruction {
    direction: char,
    distance: u32,
}

impl std::str::FromStr for Instruction {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp: Vec<&str> = s.split_whitespace().collect();
        Ok(Self { direction: sp[0].chars().next().unwrap(), distance: sp[1].parse()? })
    }
}

struct Rope {
    h: (i32, i32),
    t: (i32, i32),
    visited: HashMap<(i32, i32), u32>,
}

impl Rope {
    pub fn apply_instruction(& mut self, instruction: &Instruction) {
        let direction_move: HashMap<char, (i32, i32)> = HashMap::from([('R', (1,0)),('L', (-1,0)),('U', (0,1)),('D', (0,-1))]);
        let delta = direction_move.get(&instruction.direction).unwrap();
        for _c in 0..instruction.distance {
            self.h.0 += delta.0;
            self.h.1 += delta.1;

            self.move_tail();
        }
    }

    fn move_tail(& mut self) {
        let x_delta = self.h.0 - self.t.0;
        let y_delta = self.h.1 - self.t.1;

        if x_delta.abs() > 1 || y_delta.abs() > 1 {
            if x_delta.abs() > y_delta.abs() {
                self.t.0 += x_delta;
                if x_delta > 0 {
                    self.t.0 -= 1;
                } else {
                    self.t.0 += 1;
                }
                self.t.1 = self.h.1;
            } else {
                self.t.1 += y_delta;
                if y_delta > 0 {
                    self.t.1 -= 1;
                } else {
                    self.t.1 += 1;
                }
                self.t.0 = self.h.0;
            }
            self.visited.entry(self.t).and_modify(|c| *c += 1).or_insert(0);
        }
        // println!("h = {0},{1} - t = {2},{3}", self.h.0, self.h.1, self.t.0, self.t.1);
    }
}

pub struct Day09{}

impl Day for Day09 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<Instruction> = ipr.vec_1d_newln()?;

        let mut rope = Rope{
            h: (0,0),
            t: (0,0),
            visited: HashMap::new(),
        };
        rope.visited.insert((0,0), 0);

        for i in data {
            rope.apply_instruction(&i);
            
        }

        Ok(rope.visited.len().to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        Ok(ipr.fullname()?)
    }
}