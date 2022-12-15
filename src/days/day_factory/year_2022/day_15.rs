use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

use std::collections::HashSet;
use regex::Regex;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SPoint {
    pub x: i64,
    pub y: i64,
}

impl SPoint {
    pub fn mhd(&self, d:&Self) -> i64 {
        (self.x - d.x).abs() + (self.y - d.y).abs()
    }
}

impl std::str::FromStr for SPoint {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Self{
            x: caps[1].trim().parse()?,
            y: caps[2].trim().parse()?,
        })
    }
}

struct PointSet {
    s: HashSet<SPoint>,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl PointSet {
    pub fn add_sensor(& mut self, s:SPoint, d:i64) {
        for x in 0..d+1 {
            for y in 0..d+1-x {
                let x_pos = s.x+x;
                let y_pos = s.y+y;
                let x_neg = s.x-x;
                let y_neg = s.y-y;
                self.add_point(SPoint{x: x_pos,y: y_pos});
                self.add_point(SPoint{x: x_neg,y: y_pos});
                self.add_point(SPoint{x: x_pos,y: y_neg});
                self.add_point(SPoint{x: x_neg,y: y_neg});
            }
        }
    }

    pub fn add_point(& mut self, s:SPoint) {
        if s.x < self.x_min {
            self.x_min = s.x;
        }
        if s.x > self.x_max {
            self.x_max = s.x;
        }
        if s.y < self.y_min {
            self.y_min = s.y;
        }
        if s.y > self.y_max {
            self.y_max = s.y;
        }
        self.s.insert(s);
    }

    pub fn count_y_line(&self, y:i64) -> u64{
        let mut total = 0;
        for x in self.x_min..self.x_max {
            if self.s.contains(&SPoint{x,y}) {
                total += 1;
            }
        }
        total
    }

    pub fn print(&self) {
        for y in self.y_min..self.y_max+1 {
            print!("{} ", y);
            for x in self.x_min..self.x_max+1 {
                if self.s.contains(&SPoint{x,y}) {
                    print!("#", );
                } else {
                    print!(".", );
                }
                    
            }
            println!();
        }
    }
}

impl std::str::FromStr for PointSet {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ls: Vec<&str> = s.lines().collect();
        let mut ps = Self{
            s: HashSet::new(),
            x_min: i64::MAX,
            x_max: i64::MIN,
            y_min: i64::MAX,
            y_max: i64::MIN,
        };
        for l in ls {
            let re = Regex::new(r"Sensor at (x=-?\d+, y=-?\d+): closest beacon is at (x=-?\d+, y=-?\d+)").unwrap();
            let caps = re.captures(l).unwrap();
            println!("{} - {}", &caps[1], &caps[2]);
            let sensor: SPoint = caps[1].parse()?;
            let beacon: SPoint = caps[2].parse()?;
            let d = sensor.mhd(&beacon);
            ps.add_sensor(sensor, d);
        }

        Ok(ps)
    }
}

pub struct Day15{}

impl Day for Day15 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data:PointSet = ipr.whole()?;
        data.print();
        Ok(data.count_y_line(10).to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        Ok(ipr.fullname()?)
    }
}