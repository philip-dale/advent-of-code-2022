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
            //println!("****");
        }
    }

    fn move_tail(& mut self, i: usize) {
        let x_delta = self.knots[i-1].0 - self.knots[i].0;
        let y_delta = self.knots[i-1].1 - self.knots[i].1;

        if x_delta.abs() > 1 || y_delta.abs() > 1 {
            //println!("{0}: {1},{2} - {3},{4} : dx = {5}, dy = {6}", i, self.knots[i-1].0, self.knots[i-1].1, self.knots[i].0, self.knots[i].1, x_delta, y_delta);
            if x_delta.abs() == 2 && y_delta.abs() == 2{
                if x_delta > 0 {
                    self.knots[i].0 = self.knots[i-1].0-1;
                } else {
                    self.knots[i].0 = self.knots[i-1].0+1;
                }

                if y_delta > 0 {
                    self.knots[i].1 = self.knots[i-1].1-1;
                } else {
                    self.knots[i].1 = self.knots[i-1].1+1;
                }
                //self.knots[i].0 += x_delta/2;
                //self.knots[i].1 += y_delta/2;
            } else if x_delta.abs() > y_delta.abs() {
                //self.knots[i].0 += x_delta;
                if x_delta > 0 {
                    self.knots[i].0 = self.knots[i-1].0-1;
                } else {
                    self.knots[i].0 = self.knots[i-1].0+1;
                }
                self.knots[i].1 = self.knots[i-1].1;
            } else {
                //self.knots[i].1 += y_delta;
                if y_delta > 0 {
                    self.knots[i].1 = self.knots[i-1].1-1;
                } else {
                    self.knots[i].1 = self.knots[i-1].1+1;
                }
                self.knots[i].0 = self.knots[i-1].0;
            }
            //println!("{0},{1} : {2},{3}", self.knots[i-1].0, self.knots[i-1].1, self.knots[i].0, self.knots[i].1);
        }
        //println!("h = {0},{1} - t = {2},{3}", self.knots[0].0, self.knots[0].1, self.knots[1].0, self.knots[1].1);
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
            //for k in &rope.knots {
                //println!("{0},{1}", k.0, k.1);
            //}
            //println!("___");
        }
        //for k in rope.visited.keys() {
        //    println!("{0},{1}",k.0, k.1);
        //}
        Ok(rope.visited.len().to_string())
    }
}