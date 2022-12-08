use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::{CharNumGrid, Point};

static DELTAS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

impl CharNumGrid  {
    pub fn check_points(&self) -> u64{
        let mut total = 0;
        for x in 0..self.cells.len() {
            for y in 0..self.cells[0].len() {
                if self.check_point(Point{x,y}) {
                    total += 1;
                }
            }
        }
        total
    }

    pub fn check_point(&self, p: Point) -> bool {
        let x_max_value = self.cells.len() as i32;
        let y_max_value = self.cells[0].len() as i32;

        for (dx, dy) in DELTAS {
            let mut x = p.x as i32;
            let mut y = p.y as i32;
            let mut loop_vis = true;
            loop {
                x += dx;
                y += dy;
                if x < 0 || x >= x_max_value || y < 0 || y >= y_max_value {
                    break;
                }
                if self.cells[x as usize][y as usize] >= self.cells[p.x][p.y] {
                    loop_vis = false;
                    break;
                }
            }
            if loop_vis {
                return true;
            }
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
        let mut count = 1; // using multiples
        let x_max_value = self.cells.len() as i32;
        let y_max_value = self.cells[0].len() as i32;

        for (dx, dy) in DELTAS {
            let mut x = p.x as i32;
            let mut y = p.y as i32;
            let mut loop_count = 0;
            loop {
                x += dx;
                y += dy;
                if x < 0 || x >= x_max_value || y < 0 || y >= y_max_value {
                    break;
                }
                loop_count += 1;
                if self.cells[x as usize][y as usize] >= self.cells[p.x][p.y] {
                    break;
                }
            }
            count *= loop_count;
        }
        count
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