use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::CharNumGrid;

pub struct Day09{}

impl CharNumGrid {
    pub fn find_low(&self) -> Vec<(usize, usize)> {
        let mut low_points: Vec<(usize, usize)> = Vec::new();

        let x_max = self.cells.len()-1;
        let y_max = self.cells[0].len()-1;

        for (ix, x) in self.cells.iter().enumerate() {
            for (iy, v) in x.iter().enumerate() {
                if ix != 0 && self.cells[ix-1][iy] <= *v{
                    continue;
                }
                if ix != x_max && self.cells[ix+1][iy] <= *v{
                    continue;
                }
                if iy != 0 && self.cells[ix][iy-1] <= *v{
                    continue;
                }
                if iy != y_max && self.cells[ix][iy+1] <= *v{
                    continue;
                }
                low_points.push((ix, iy));
            }
        }
        low_points
    }
 
    pub fn calc_low_risk(&self, lps: &Vec<(usize, usize)>) -> u32 {
        let mut total = 0;
        for lp in lps {
            total += self.cells[lp.0][lp.1] + 1;
        }
        total
    }
}

fn fill_basin(cells: &Vec<Vec<u32>>, basin_map: &mut Vec<Vec<usize>>, x: usize, y: usize, count: usize, id: usize) -> usize {
    if basin_map[x][y] > 0 || cells[x][y] == 9{
        return count;
    }

    basin_map[x][y] = id;

    let x_max = cells.len()-1;
    let y_max = cells[0].len()-1;

    let mut count = count + 1;
    if x != 0 {
        count = fill_basin(cells, basin_map, x-1, y, count, id);
    }
    if x != x_max {
        count = fill_basin(cells, basin_map, x+1, y, count, id);
    }
    if y != 0 {
        count = fill_basin(cells, basin_map, x, y-1, count, id);
    }
    if y != y_max {
        count = fill_basin(cells, basin_map, x, y+1, count, id);
    }
    count
}

impl Day for Day09 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: CharNumGrid = ipr.whole()?;
        let low_points = data.find_low();
        Ok(data.calc_low_risk(&low_points).to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: CharNumGrid = ipr.whole()?;
        let low_points = data.find_low();

        let mut basin_map: Vec<Vec<usize>> = Vec::new();
        for x in &data.cells {
            basin_map.push(vec![0; x.len()]);
        }

        let mut basin_sizes: Vec<usize> = Vec::new();

        for (lpi, lp) in low_points.iter().enumerate() {
            basin_sizes.push(fill_basin(&data.cells, &mut basin_map, lp.0, lp.1, 0, lpi+1));
        }

        basin_sizes.sort();
        let basin_sizes_len = basin_sizes.len();
        let risk = basin_sizes[basin_sizes_len-1] * basin_sizes[basin_sizes_len-2] * basin_sizes[basin_sizes_len-3];

        Ok(risk.to_string())
    }
}