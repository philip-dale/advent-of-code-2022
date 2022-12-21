use crate::days::day_factory::Day;
use crate::input_reader;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

#[derive(PartialEq, Debug)]
enum Shout {
    Val(i64),
    Ins(Instruction),
    Unknown,
}

impl Shout {
    pub fn get_instruction(&self) -> Option<&Instruction> {
        match self {
            Self::Ins(x) => Some(x),
            _ => None,
        }
    }
}

impl std::str::FromStr for Shout {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(char::is_numeric) {
            Ok(Self::Val(s.parse()?))
        } else {
            let re = Regex::new(r"(....) ([+-\\*/]) (....)").unwrap();
            let caps = re.captures(s).unwrap();
            Ok(Self::Ins(
                Instruction::new(caps[1].to_string(), caps[3].to_string(), &caps[2]).unwrap(),
            ))
        }
    }
}

struct Shouts {
    m: HashMap<String, Shout>,
}

impl Shouts {
    pub fn solve(&self, monkey: &str) -> i64 {
        match self.m.get(monkey).unwrap() {
            Shout::Val(x) => *x,
            Shout::Ins(x) => x.solve(self),
            Shout::Unknown => todo!(),
        }
    }

    pub fn solve_unknown(&self, monkey: &str, result: i64) -> i64 {
        match self.m.get(monkey).unwrap() {
            Shout::Val(x) => *x,
            Shout::Ins(x) => {
                let (lhs, rhs) = x.get_monkeys();
                if self.has_unknown(lhs) {
                    let r_val = self.solve(rhs);
                    let next_result = x.reverse_solve_lhs(result, r_val);
                    self.solve_unknown(lhs, next_result)
                } else {
                    let l_val = self.solve(lhs);
                    let next_result = x.reverse_solve_rhs(result, l_val);
                    self.solve_unknown(rhs, next_result)
                }
            }
            Shout::Unknown => result,
        }
    }

    pub fn has_unknown(&self, monkey: &str) -> bool {
        match self.m.get(monkey).unwrap() {
            Shout::Val(_x) => false,
            Shout::Ins(x) => {
                self.has_unknown(x.get_monkeys().0) || self.has_unknown(x.get_monkeys().1)
            }
            Shout::Unknown => true,
        }
    }

    pub fn set_unknown(&mut self, monkey: &str) {
        *self.m.get_mut(monkey).unwrap() = Shout::Unknown;
    }

    pub fn set_equal(&mut self, monkey: &str) {
        let (lhs, rhs) = self
            .m
            .get_mut(monkey)
            .unwrap()
            .get_instruction()
            .unwrap()
            .get_monkeys();
        *self.m.get_mut(monkey).unwrap() =
            Shout::Ins(Instruction::Equ(lhs.to_string(), rhs.to_string()));
    }

    pub fn set_val(&mut self, monkey: &str, val: i64) {
        *self.m.get_mut(monkey).unwrap() = Shout::Val(val);
    }

    pub fn set_sub(&mut self, monkey: &str) {
        let (lhs, rhs) = self
            .m
            .get_mut(monkey)
            .unwrap()
            .get_instruction()
            .unwrap()
            .get_monkeys();
        *self.m.get_mut(monkey).unwrap() =
            Shout::Ins(Instruction::Sub(lhs.to_string(), rhs.to_string()));
    }
}

impl std::str::FromStr for Shouts {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lines: Vec<&str> = s.lines().collect();
        Ok(Self {
            m: {
                let mut m = HashMap::new();
                for l in s_lines {
                    let l_split: Vec<&str> = l.split(':').collect();
                    m.insert(l_split[0].trim().to_string(), l_split[1].trim().parse()?);
                }
                m
            },
        })
    }
}

#[derive(PartialEq, Debug)]
enum Instruction {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Equ(String, String),
}

impl Instruction {
    pub fn new(x: String, y: String, operator: &str) -> Option<Self> {
        match operator {
            "+" => Some(Self::Add(x, y)),
            "-" => Some(Self::Sub(x, y)),
            "*" => Some(Self::Mul(x, y)),
            "/" => Some(Self::Div(x, y)),
            "=" => Some(Self::Equ(x, y)),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn get_operator(&self) -> &str {
        match self {
            Self::Add(_, _) => "+",
            Self::Sub(_, _) => "-",
            Self::Mul(_, _) => "*",
            Self::Div(_, _) => "/",
            Self::Equ(_, _) => "==",
        }
    }

    pub fn get_monkeys(&self) -> (&str, &str) {
        match self {
            Self::Add(x, y) => (x, y),
            Self::Sub(x, y) => (x, y),
            Self::Mul(x, y) => (x, y),
            Self::Div(x, y) => (x, y),
            Self::Equ(x, y) => (x, y),
        }
    }

    pub fn solve(&self, shouts: &Shouts) -> i64 {
        match self {
            Self::Add(x, y) => shouts.solve(x) + shouts.solve(y),
            Self::Sub(x, y) => shouts.solve(x) - shouts.solve(y),
            Self::Mul(x, y) => shouts.solve(x) * shouts.solve(y),
            Self::Div(x, y) => shouts.solve(x) / shouts.solve(y),
            Self::Equ(x, y) => shouts.solve(x) / shouts.solve(y),
        }
    }

    pub fn reverse_solve_lhs(&self, result: i64, rhs: i64) -> i64 {
        match self {
            Instruction::Add(_, _) => result - rhs,
            Instruction::Sub(_, _) => result + rhs,
            Instruction::Mul(_, _) => result / rhs,
            Instruction::Div(_, _) => result * rhs,
            Instruction::Equ(_, _) => rhs,
        }
    }

    pub fn reverse_solve_rhs(&self, result: i64, lhs: i64) -> i64 {
        match self {
            Instruction::Add(_, _) => result - lhs,
            Instruction::Sub(_, _) => lhs - result,
            Instruction::Mul(_, _) => result / lhs,
            Instruction::Div(_, _) => lhs / result,
            Instruction::Equ(_, _) => lhs,
        }
    }
}

pub struct Day21 {}

impl Day for Day21 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let shouts: Shouts = ipr.whole()?;
        let val = shouts.solve("root");
        Ok(val.to_string())
    }

    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut shouts: Shouts = ipr.whole()?;
        shouts.set_unknown("humn");
        shouts.set_equal("root");
        let val = shouts.solve_unknown("root", 0);

        // Test answer
        shouts.set_val("humn", val);
        shouts.set_sub("root");
        let res = shouts.solve("root");
        println!("res = {}", res);

        Ok(val.to_string())
    }
}
