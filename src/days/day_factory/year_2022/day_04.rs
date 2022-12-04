use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

struct DashRange {
    pub start: usize,
    pub end: usize,
}

impl DashRange {
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

impl std::str::FromStr for DashRange {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<&str> = s.split('-').collect();

        Ok(DashRange {
            start: vals[0].parse()?,
            end: vals[1].parse()?,
        })
    }
}

struct DashRangePair {
    short: DashRange,
    long: DashRange,
}

impl DashRangePair {
    pub fn contained(&self) -> bool {
        self.short.start >= self.long.start && self.short.end <= self.long.end
    }

    pub fn overlaps(&self) -> bool {
        if self.short.start >= self.long.start && self.short.start <= self.long.end {
            return true
        }
        self.short.end >= self.long.start && self.short.end <= self.long.end
    }
}

impl std::str::FromStr for DashRangePair {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp : Vec<&str> = s.split(',').collect();
        let r1 : DashRange = sp[0].parse()?;
        let r2 : DashRange = sp[1].parse()?;

        match r1 {
            r1 if r1.len() < r2.len() => Ok(DashRangePair{
                short : r1,
                long : r2,
            }),
            _ => Ok(DashRangePair{
                short : r2,
                long : r1,
            }),
        }
    }
}


pub struct Day04{}

impl Day for Day04 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data : Vec<DashRangePair> = ipr.vec_1d_newln()?;
        let mut total:u32 = 0;
        for p in data {
            if p.contained() {
                total += 1;
            }
        }
        
        Ok(total.to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data : Vec<DashRangePair> = ipr.vec_1d_newln()?;
        let mut total:u32 = 0;
        for p in data {
            if p.overlaps() {
                total += 1;
            }
        }

        Ok(total.to_string())
    }
}