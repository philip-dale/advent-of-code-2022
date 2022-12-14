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
    knots: Vec<(i32, i32)>,
    visited: HashMap<(i32, i32), u32>,
}

impl Rope {
    pub fn new(size: usize) -> Self {
        Self{
            knots: vec![(0,0); size],
            visited: HashMap::from([((0,0),0)]),
        }
    }

    pub fn apply_instruction(& mut self, instruction: &Instruction) {
        let direction_move: HashMap<char, (i32, i32)> = HashMap::from([('R', (1,0)),('L', (-1,0)),('U', (0,1)),('D', (0,-1))]);
        let delta = direction_move.get(&instruction.direction).unwrap();
        for _c in 0..instruction.distance {
            self.knots[0].0 += delta.0;
            self.knots[0].1 += delta.1;

            for k in 1..self.knots.len(){
                self.move_tail(k);
            }
            self.visited.entry(*self.knots.last().unwrap()).and_modify(|c| *c += 1).or_insert(0);
        }
    }

    fn move_tail(& mut self, i: usize) {
        let x_delta = self.knots[i-1].0 - self.knots[i].0;
        let y_delta = self.knots[i-1].1 - self.knots[i].1;

        if x_delta.abs() > 1 || y_delta.abs() > 1 {
            if x_delta.abs() == 2 {
                if x_delta > 0 {
                    self.knots[i].0 = self.knots[i-1].0-1;
                } else {
                    self.knots[i].0 = self.knots[i-1].0+1;
                }
            } else  {
                self.knots[i].0 = self.knots[i-1].0;
            }

            if y_delta.abs() == 2{
                if y_delta > 0 {
                    self.knots[i].1 = self.knots[i-1].1-1;
                } else {
                    self.knots[i].1 = self.knots[i-1].1+1;
                }
            } else {
                self.knots[i].1 = self.knots[i-1].1;
            }
        }
    }
}

pub struct Day09{}

impl Day for Day09 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<Instruction> = ipr.vec_1d_newln()?;
        let mut rope = Rope::new(2);
        for i in &data {
            rope.apply_instruction(i);
        }
        Ok(rope.visited.len().to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<Instruction> = ipr.vec_1d_newln()?;
        let mut rope = Rope::new(10);
        for i in &data {
            rope.apply_instruction(i);
        }
        Ok(rope.visited.len().to_string())
    }
}