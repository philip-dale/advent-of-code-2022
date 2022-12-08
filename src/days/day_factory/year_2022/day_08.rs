use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::{CharNumGrid, Point};

struct VisGrid {
    cells :Vec<Vec<u32>>,
}

impl VisGrid  {
    pub fn new(g: &CharNumGrid) -> Self{
        Self{
            cells: vec![vec![0; g.cells[0].len()]; g.cells.len()],
        }
    }

    pub fn set_visibe(& mut self) {
        let x_max_value = self.cells.len();
        let y_max_value = self.cells.len();
        for x in 0..self.cells.len() {
            for y in 0..self.cells[0].len() {
                let p = Point{x,y};
                let neigh = p.get_adjacent_neighbours(x_max_value, y_max_value);
                if neigh.len() < 4  {
                    self.cells[x][y] = 1;
                }
                
            }
        }
    }
}

pub struct Day08{}

impl Day for Day08 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: CharNumGrid = ipr.whole()?;

        Ok(ipr.fullname()?)
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        Ok(ipr.fullname()?)
    }
}