use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::Point;

struct Folding {
    points: Vec<Point>,
    instructions: Vec<(char, usize)>,
}

impl Folding {
    #[allow(dead_code)]
    pub fn print(&self) {
        for p in &self.points {
            println!("x = {0}, y = {1}", p.x, p.y);
        }
        for i in &self.instructions {
            println!("{0}={1}", i.0, i.1);
        }
    }

    pub fn display(&self) {
        let mut x_max = 0;
        let mut y_max = 0;
        for p in self.points.iter() {
            if p.x > x_max {
                x_max = p.x;
            }
            if p.y > y_max {
                y_max = p.y;
            }
        }
        let mut disp: Vec<Vec<char>> = vec![vec![' '; x_max+1]; y_max+1];
        for p in self.points.iter() {
            disp[p.y][p.x] = '*';
        }
        for x in disp {
            for y in x {
                print!("{}", y);
            }
            println!();
        }
    }

    pub fn fold(& mut self, line: &(char, usize)) {
        for p in self.points.iter_mut() {
            if line.0 == 'x' {
                if p.x > line.1 {
                    p.x -= (p.x-line.1)*2;
                }
            } else if p.y > line.1 {
                p.y -= (p.y-line.1)*2;
            }
        }
        self.dedupe();
    }

    pub fn dedupe(& mut self) {
        self.points.sort();
        self.points.dedup();

    }
    pub fn fold_all(& mut self) {
        for i in self.instructions.to_vec().iter() {
            self.fold(i);
        }
    }
}


impl std::str::FromStr for Folding {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut folding = Folding{
            points: Vec::new(),
            instructions: Vec::new(),
        };

        let mut instructions = false;
        for l in s.lines() {
            if l.is_empty() {
                instructions = true;
                continue;
            }
            if instructions {
                let parts:Vec<&str> = l.split('=').collect();
                folding.instructions.push((parts[0].chars().last().unwrap(), parts[1].parse()?));
            } else {
                folding.points.push(l.parse()?)
            }

        }

        Ok(folding)
    }
}

pub struct Day13{}

impl Day for Day13 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut folding:Folding = ipr.whole()?;
        folding.fold(&(folding.instructions[0].0, folding.instructions[0].1));
        Ok(folding.points.len().to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut folding:Folding = ipr.whole()?;
        folding.fold_all();
        folding.display();
        Ok(folding.points.len().to_string())
    }
}