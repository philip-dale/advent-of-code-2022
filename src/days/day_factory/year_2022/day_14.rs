use crate::days::day_factory::types::Point;
use crate::days::day_factory::Day;
use crate::input_reader;
use std::error::Error;

use std::collections::HashSet;

pub struct Day14 {}

struct Cave {
    m: HashSet<Point>,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
    sand_entry: Point,
}

impl std::str::FromStr for Cave {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = Cave {
            m: HashSet::new(),
            x_min: usize::MAX,
            x_max: 0,
            y_min: usize::MAX,
            y_max: 0,
            sand_entry: Point { x: 500, y: 0 },
        };
        let sl: Vec<&str> = s.lines().collect();
        for l in sl {
            let points: Vec<&str> = l.split("->").collect();
            let mut current: Point = points[0].parse()?;
            for n in points {
                let next: Point = n.parse()?;
                if current == next {
                    continue;
                }
                c.add_line(&current, &next);
                current = next;
            }
        }
        Ok(c)
    }
}

impl Cave {
    pub fn add_line(&mut self, s: &Point, e: &Point) {
        let x_range = if s.x < e.x {
            s.x..e.x + 1
        } else {
            e.x..s.x + 1
        };

        let y_range = if s.y < e.y {
            s.y..e.y + 1
        } else {
            e.y..s.y + 1
        };

        for x in x_range {
            for y in y_range.clone() {
                self.m.insert(Point { x, y });
                if x < self.x_min {
                    self.x_min = x;
                }
                if x > self.x_max {
                    self.x_max = x;
                }
                if y < self.y_min {
                    self.y_min = y;
                }
                if y > self.y_max {
                    self.y_max = y;
                }
            }
        }
    }

    pub fn run(&mut self) -> u64 {
        let mut sand_pos = self.sand_entry.as_point();
        let mut rest_count = 0;
        loop {
            if sand_pos.y + 1 > self.y_max {
                return rest_count;
            } else if !self.m.contains(&Point {
                x: sand_pos.x,
                y: sand_pos.y + 1,
            }) {
                sand_pos.y += 1;
            } else if sand_pos.x - 1 < self.x_min {
                return rest_count;
            } else if !self.m.contains(&Point {
                x: sand_pos.x - 1,
                y: sand_pos.y + 1,
            }) {
                sand_pos.y += 1;
                sand_pos.x -= 1;
            } else if sand_pos.x + 1 > self.x_max {
                return rest_count;
            } else if !self.m.contains(&Point {
                x: sand_pos.x + 1,
                y: sand_pos.y + 1,
            }) {
                sand_pos.y += 1;
                sand_pos.x += 1;
            } else if sand_pos.cmp(&self.sand_entry).is_eq() {
                // Jam
                return rest_count + 1;
            } else {
                // Comes to rest
                self.m.insert(sand_pos);
                sand_pos = self.sand_entry.as_point();
                rest_count += 1;
            }
        }
    }
}

impl Day for Day14 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut c: Cave = ipr.whole()?;
        let rest_count = c.run();
        Ok(rest_count.to_string())
    }

    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut c: Cave = ipr.whole()?;
        c.add_line(
            &Point {
                x: c.x_min - c.y_max,
                y: c.y_max + 2,
            },
            &Point {
                x: c.x_max + c.y_max,
                y: c.y_max + 2,
            },
        );
        let rest_count = c.run();
        Ok(rest_count.to_string())
    }
}
