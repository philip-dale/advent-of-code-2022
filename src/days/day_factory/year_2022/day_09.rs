use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::Point;

use std::collections::HashMap;

static DIRECTION_MOVE: HashMap<char, (i32, i32)> = HashMap::from([('R', (1,0)),('L', (-1,0)),('U', (0,1)),('D', (0,-1))]);

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
    visited: HashMap<Point, u32>,
}

impl Rope {
    pub fn apply_instruction(& mut self, instruction: &Instruction) {
        let delta = DIRECTION_MOVE.get(&instruction.direction).unwrap();
        for c in 0..instruction.distance {
            self.h.0 += delta.0;
            self.h.1 += delta.1;


        }
    }

    fn move_tail(& mut self) {
        let x_delta = self.h.0 - self.y.0;
        let y_delta = self.h.0 - self.t.0;

        if x_delta.abs() > 1 | y_delta.abs() > 1 {
            if x_delta.abs() > y_delta.abs() {
                self.t.
            } else {

            }
        }
    }
}

pub struct Day09{}

impl Day for Day09 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<Instruction> = ipr.vec_1d_newln()?;

        let mut rope = Rope{
            h: Point{x: 0, y:0},
            t: Point{x: 0, y:0},
            visited: HashMap::new(),
        };



        Ok(ipr.fullname()?)
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        Ok(ipr.fullname()?)
    }
}