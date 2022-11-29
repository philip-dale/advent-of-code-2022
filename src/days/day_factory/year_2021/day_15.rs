use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::CharNumGrid;

struct PathItem {
    pub weight: u32,
    pub score: u64,
    pub visited: bool,
}

impl PathItem {
    pub fn new(weight: u32) -> Self {
        return Self {
            weight: weight,
            score: u64::MAX,
            visited: false,
        }
    }
}



pub struct Day15{}

impl Day for Day15 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: CharNumGrid = ipr.whole()?;
        let mut search_path: Vec<Vec<PathItem>> = Vec::new();
        for x in data.cells {
            let mut search_line: Vec<PathItem> = Vec::new();
            for y in x {
                search_line.push(PathItem::new(y));
            }
            search_path.push(search_line);
        }

        return Ok(ipr.fullname()?);
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        return Ok(ipr.fullname()?);
    }
}