use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::Lines;

pub struct Day05{}

fn lines_to_map(lines: &Lines, no_diag: bool) -> Vec<Vec<u64>> {

    let (x_max, y_max) = lines.get_max();

    let mut v: Vec<Vec<u64>> = vec![vec![0; x_max]; y_max];

    for l in &lines.vectors {
        if !l.is_diag() || !no_diag {
            let mut x = l.s.x;
            let mut y = l.s.y;

            for _s in 0..l.steps() {
                v[y][x] += 1;
                if x < l.e.x {
                    x += 1;
                } else if x > l.e.x{
                    x -= 1;
                }

                if y < l.e.y {
                    y += 1;
                } else if y > l.e.y{
                    y -= 1;
                }
            }

        }
    }

    return v;
}

impl Day for Day05 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data = Lines{
            vectors: ipr.vec_1d()?,
        };

        let vents = lines_to_map(&data, true);

        let mut count = 0;
        for i in vents {
            for j in i {
                if j > 1 {
                    count += 1;
                }
            }
        }
        return Ok(count.to_string());
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data = Lines{
            vectors: ipr.vec_1d()?,
        };

        let vents = lines_to_map(&data, false);

        let mut count = 0;
        for i in vents {
            for j in i {
                if j > 1 {
                    count += 1;
                }
            }
        }
        return Ok(count.to_string());
    }
}