use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

struct Instruction {
    op: String,
    val: i64,
}

impl std::str::FromStr for Instruction {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp: Vec<&str> = s.split_whitespace().collect();
        if sp.len() == 2 {
            Ok(Self { op: sp[0].to_string(), val: sp[1].parse()? })
        } else {
            Ok(Self{op: sp[0].to_string(), val: 0})
        }
    }
}

struct RegState {
    x: i64,
    history: Vec<i64>,
}

impl RegState {
    pub fn new() -> Self {
        Self{
            x: 1,
            history: Vec::new(),
        }
    }

    pub fn apply_instruction(& mut self, instruction: &Instruction) {
        self.history.push(self.x);
        if instruction.op == "addx" {
            self.history.push(self.x);
            self.x += instruction.val;
        }
    }

    pub fn get_score (&self) -> i64 {
        let pos = [20, 60, 100, 140, 180, 220];
        let mut total = 0;
        for p in pos {
            println!("{1} * {0}", self.history[p-1], p);
            total += self.history[p-1] * p as i64;
        }
        total
    }

    pub fn print(&self) {
        let per_row = 40;
        for r in 0..6 {
            for i in 0..per_row {
                let val = self.history[(r * per_row)  + i];
                if i as i64 >= val -1 && i as i64 <= val + 1{
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub struct Day10{}

impl Day for Day10 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data : Vec<Instruction> = ipr.vec_1d_newln()?;
        let mut cpu = RegState::new();
        for i in data {
            cpu.apply_instruction(&i);
        }
        Ok(cpu.get_score().to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<Instruction> = ipr.vec_1d_newln()?;
        let mut cpu = RegState::new();
        for i in data {
            cpu.apply_instruction(&i)
        }
        cpu.print();
        Ok(String::from("done"))
    }
}