use std::error::Error;
use regex::Regex;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate ::days::day_factory::types::DOUBLE_NEW_LINE;

#[derive(Debug)]
enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}
#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    test: u64,
    if_true: u64,
    if_false: u64,
    inspected: u64,
}

impl std::str::FromStr for Monkey {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"items: ([^\n]+)\n.+new = old (.) ([^\n]+)\n.+by (.+)\n.+?monkey (.)\n.+monkey (.)").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Self{
            items : {
                let mut v = Vec::new();
                    let sp: Vec<&str> = caps[1].split(',').collect();
                    for val in sp {
                        v.push(val.trim().parse()?);
                    }
                v
            },
            op: { 
                match &caps[2] {
                    "+" => {Operation::Add(caps[3].parse()?)},
                    _ => {
                        match &caps[3] {
                            "old" => Operation::Square,
                            _ => Operation::Mult(caps[3].parse()?),
                        }
                    },
                }
            },
            test : caps[4].trim().parse()?,
            if_true: caps[5].parse()?,
            if_false: caps[6].parse()?,
            inspected: 0,
        })
    }
}

fn process_monkeys(monkeys:& mut Vec<Monkey>, div: u64) {
    for m in 0..monkeys.len() {
        while !monkeys[m].items.is_empty() {
            let mut val = monkeys[m].items.remove(0);
            match monkeys[m].op {
                Operation::Add(v) => val += v,
                Operation::Mult(v) => val *= v,
                Operation::Square => val *= val,
            }
            val /= div;
            //val = val.floor();
            let pos = if val % monkeys[m].test == 0{
                monkeys[m].if_true
            } else {
                monkeys[m].if_false
            };
            monkeys[pos as usize].items.push(val);
            monkeys[m].inspected += 1;
        }
    }
}

fn get_score (monkeys: & Vec<Monkey>) -> u64{
    let mut first =0;
    let mut second = 0;
    for m in monkeys {
        if m.inspected >= second {
            if m.inspected >= first {
                second = first;
                first = m.inspected;
            } else {
                second = m.inspected;
            }
        }
    }
    first * second
}

pub struct Day11{}

impl Day for Day11 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut data:Vec<Monkey> = ipr.vec_1d_sep(&DOUBLE_NEW_LINE.to_string())?;
        //for m in &data {
        //    println!("{:?}", m);
        //}
        for _l in 0..20 {
            process_monkeys(&mut data, 3);
        }
        for m in &data {
            println!("{:?}", m);
        }
        Ok(get_score(&data).to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut data : Vec<Monkey> = ipr.vec_1d_sep(&DOUBLE_NEW_LINE.to_string())?;
        for _l in 0..1 {
            process_monkeys(&mut data, 1);
        }
        for m in &data {
            println!("{:?}", m);
        }
        Ok(get_score(&data).to_string())
    }
}