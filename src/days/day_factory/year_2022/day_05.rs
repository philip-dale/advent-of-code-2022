use regex::Regex;
use std::error::Error;

use crate::input_reader;
use crate::days::day_factory::Day;

#[cfg(windows)]
const GROUP_ENDING: & str = "\r\n\r\n";
#[cfg(not(windows))]
const GROUP_ENDING: & str = "\n\n";
struct CrateMoves {
    count: usize,
    source: usize,
    dest: usize,
}

impl std::str::FromStr for CrateMoves {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"move (?P<count>\d\d?) from (?P<from>\d) to (?P<to>\d)").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Self{
            count: caps["count"].parse()?,
            source: caps["from"].parse()?,
            dest: caps["to"].parse()?,
        })
    }
}

struct CrateUnload {
    stacks: Vec<Vec<char>>,
    moves: Vec<CrateMoves>,
}

impl std::str::FromStr for CrateUnload {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = s.split(GROUP_ENDING).collect();

        let mut crate_unload= Self{
            stacks: Vec::new(),
            moves: Vec::new(),
        };

        // Not so happy with this but it works!
        let mut processed_header = false;
        for l in sections[0].lines().rev() {
            let stack_count = (l.len()/4) + 1;
            if !processed_header {
                processed_header = true;
                crate_unload.stacks = vec![Vec::new(); stack_count];
                continue;
            }

            for i in 0..stack_count {
                let val = l.chars().nth((i*4)+1).unwrap();
                if val != ' ' {
                    crate_unload.stacks[i].push(val);
                }
            }
        }

        for l in sections[1].lines() {
            crate_unload.moves.push(l.parse()?);
        }

        Ok(crate_unload)
    }
}

impl CrateUnload {
    pub fn apply_moves(&mut self, in_order: bool) {
        let moves = &self.moves;

        for m in moves {
            for i in 0..m.count {
                if !in_order {
                    let val = *self.stacks[m.source-1].last().unwrap();
                    self.stacks[m.dest-1].push(val);
                    self.stacks[m.source-1].pop();
                } else {
                    let index = self.stacks[m.source-1].len() + i - m.count ;
                    let val = self.stacks[m.source-1][index];
                    self.stacks[m.dest-1].push(val);
                    self.stacks[m.source-1].remove(index);
                }
            }
        }
    }

    pub fn get_tops(&self) -> String {
        let mut res = String::from("");
        for (i, s) in self.stacks.iter().enumerate() {
            match s.last() {
                Some(v) => res.push(*v),
                _ => println!("Empty Stack = {}", i)
            }
        }
        res
    }
}
pub struct Day05{}

impl Day for Day05 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut data: CrateUnload = ipr.whole()?;
        data.apply_moves(false);
        Ok(data.get_tops())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut data: CrateUnload = ipr.whole()?;
        data.apply_moves(true);
        Ok(data.get_tops())
    }
}