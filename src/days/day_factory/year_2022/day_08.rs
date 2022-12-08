use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::{CharNumGrid, Point};

impl CharNumGrid  {
    pub fn check_points(&self) -> u64{
        let mut total = 0;
        for x in 0..self.cells.len() {
            for y in 0..self.cells[0].len() {
                if !self.check_point(Point{x,y}) {
                    total += 1;
                }
            }
        }
        total
    }

    pub fn check_point(&self, p: Point) -> bool {
        let x_max_value = self.cells.len();
        let y_max_value = self.cells[0].len();

        let mut hidden1 = false;
        for x in p.x+1..x_max_value {
            if self.cells[x][p.y] >= self.cells[p.x][p.y] {
                hidden1 = true;
            }
        }

        let mut hidden2 = false;
        for x in (0..p.x).rev() {
            if self.cells[x][p.y] >= self.cells[p.x][p.y] {
                hidden2 = true;
            }
        }

        let mut hidden3 = false;
        for y in p.y+1..y_max_value {
            if self.cells[p.x][y] >= self.cells[p.x][p.y] {
                hidden3 = true;
            }
        }

        let mut hidden4 = false;
        for y in (0..p.y).rev() {
            if self.cells[p.x][y] >= self.cells[p.x][p.y] {
                hidden4 = true;
            }
        }

        if hidden1 && hidden2 && hidden3 && hidden4 {
            return true;
        }
        false
    }

    pub fn check_views(&self) -> u32{
        let mut max = 0;
        for x in 0..self.cells.len() {
            for y in 0..self.cells[0].len() {
                let val = self.check_view(Point{x,y});
                if val > max {
                    max = val;
                }
            }
        }
        max
    }

    pub fn check_view(&self, p: Point) -> u32 {
        let x_max_value = self.cells.len();
        let y_max_value = self.cells[0].len();

        let mut dist1 = 0;
        for x in p.x+1..x_max_value {
            dist1 += 1;
            if self.cells[x][p.y] >= self.cells[p.x][p.y] {
                break;
            }
        }

        let mut dist2 = 0;
        for x in (0..p.x).rev() {
            dist2 += 1;
            if self.cells[x][p.y] >= self.cells[p.x][p.y] {
                break;
            }
        }

        let mut dist3 = 0;
        for y in p.y+1..y_max_value {
            dist3 += 1;
            if self.cells[p.x][y] >= self.cells[p.x][p.y] {
                break;
            }
        }

        let mut dist4 = 0;
        for y in (0..p.y).rev() {
            dist4 += 1;
            if self.cells[p.x][y] >= self.cells[p.x][p.y] {
                break;
            }
        }

        dist1 * dist2 * dist3 *dist4
    }

}

pub struct Day08{}

impl Day for Day08 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: CharNumGrid = ipr.whole()?;
        let total = data.check_points();
        Ok(total.to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: CharNumGrid = ipr.whole()?;
        let max = data.check_views();
        Ok(max.to_string())
    }
}