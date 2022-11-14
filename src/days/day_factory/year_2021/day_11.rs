use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::{CharNumGrid, get_range};

pub struct Day11{}

struct OctoEnergy {
    pub energy: Vec<Vec<u32>>,
    pub fired: Vec<Vec<u32>>,
}

fn parse_cells(octo: & mut OctoEnergy) {
    for x in 0..octo.energy.len() {
        for y in 0..octo.energy[x].len() {
            octo.energy[x][y] += 1;
            octo.fired[x][y] = 0;
        }
    }
}

fn fire_cell(octo: & mut OctoEnergy, x: usize, y: usize) -> u32{
    let mut count: u32 = 0;
    if octo.fired[x][y] == 0 {
        octo.fired[x][y] = 1;
        count += 1;

        let (x_min, x_max) = get_range (x, octo.energy.len()-1, 1);
        let (y_min, y_max) = get_range (y, octo.energy[x].len()-1, 1);

        for i in x_min..x_max+1 {
            for j in y_min..y_max+1 {
                if i == x && j == y {
                    continue;
                }
                if octo.fired[i][j] == 0 {
                    octo.energy[i][j] += 1;
                    if octo.energy[i][j] > 9 {
                        count += fire_cell(octo, i, j);
                    }
                }
                
            }
        }
        octo.energy[x][y] = 0;
    }
    return count;
}

fn fire_cells(octo: & mut OctoEnergy) -> u32{
    let mut count: u32 = 0;
    for x in 0..octo.energy.len() {
        for y in 0..octo.energy[x].len() {
            if octo.energy[x][y] > 9 {
                count += fire_cell(octo, x, y);
            }
        }
    }
    return count
}

impl Day for Day11 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: CharNumGrid = ipr.whole()?;
        let mut octo = OctoEnergy {
            fired: vec![vec![0; data.cells[0].len()]; data.cells.len()],
            energy: data.cells,
        };

        let mut fired = 0;
        for _i in 0..100 {
            parse_cells(&mut octo);
            fired += fire_cells(&mut octo);
        }
        
        return Ok(fired.to_string());
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: CharNumGrid = ipr.whole()?;
        let mut octo = OctoEnergy {
            fired: vec![vec![0; data.cells[0].len()]; data.cells.len()],
            energy: data.cells,
        };
        
        for step in 1..500 {
            parse_cells(&mut octo);
            if fire_cells(&mut octo) == 100 {
                return Ok((step).to_string());
            }
        }
        
        return Ok(String::from("Fail"));
    }
}