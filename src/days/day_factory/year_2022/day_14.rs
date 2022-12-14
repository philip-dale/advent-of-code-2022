use crate::days::day_factory::types::Point;
use crate::days::day_factory::Day;
use crate::input_reader;
use std::error::Error;

pub struct Day14 {}

struct Shape {
    v: Vec<Point>,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

impl Shape {
    fn new() -> Self {
        Self {
            v: Vec::new(),
            x_min: usize::MAX,
            x_max: 0,
            y_min: usize::MAX,
            y_max: 0,
        }
    }

    fn add_point(&mut self, p: Point) {
        if p.x < self.x_min {
            self.x_min = p.x;
        }
        if p.x > self.x_max {
            self.x_max = p.x;
        }
        if p.y < self.y_min {
            self.y_min = p.y;
        }
        if p.y > self.y_max {
            self.y_max = p.y;
        }
        self.v.push(p);
    }
}

impl std::str::FromStr for Shape {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut shape = Self {
            v: Vec::new(),
            x_min: usize::MAX,
            x_max: 0,
            y_min: usize::MAX,
            y_max: 0,
        };

        let points: Vec<&str> = s.split("->").collect();
        for p in points {
            shape.add_point(p.parse()?);
        }

        Ok(shape)
    }
}

struct Shapes {
    v: Vec<Shape>,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

impl Shapes {
    fn add_shape(&mut self, s: Shape) {
        if s.x_min < self.x_min {
            self.x_min = s.x_min;
        }
        if s.x_max > self.x_max {
            self.x_max = s.x_max;
        }
        if s.y_min < self.y_min {
            self.y_min = s.y_min;
        }
        if s.y_max > self.y_max {
            self.y_max = s.y_max;
        }
        self.v.push(s);
    }
}

impl std::str::FromStr for Shapes {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut shapes = Self {
            v: Vec::new(),
            x_min: usize::MAX,
            x_max: 0,
            y_min: usize::MAX,
            y_max: 0,
        };

        let sl: Vec<&str> = s.lines().collect();
        for l in sl {
            shapes.add_shape(l.parse()?);
        }

        Ok(shapes)
    }
}

struct Cave {
    m: Vec<Vec<char>>,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
    sand_entry: Point,
}

impl Cave {
    fn from_shapes(shapes: &Shapes) -> Self {
        let mut c = Self {
            m: vec![vec!['.'; shapes.y_max + 1]; shapes.x_max - shapes.x_min + 1],
            x_min: shapes.x_min,
            x_max: shapes.x_max,
            y_min: 0,
            y_max: shapes.y_max,
            sand_entry: Point { x: 500, y: 0 },
        };

        for s in &shapes.v {
            c.add_shape(s);
        }

        c
    }

    pub fn add_shape(&mut self, shape: &Shape) {
        let mut current: &Point = &shape.v[0];
        for next in &shape.v {
            if current == next {
                continue;
            }

            let x_range = if current.x < next.x {
                current.x..next.x + 1
            } else {
                next.x..current.x + 1
            };

            let y_range = if current.y < next.y {
                current.y..next.y + 1
            } else {
                next.y..current.y + 1
            };

            for x in x_range {
                for y in y_range.clone() {
                    self.set_point(x, y, '#');
                }
            }
            current = next;
        }
    }

    fn set_point(&mut self, x: usize, y: usize, val: char) {
        self.m[x - self.x_min][y - self.y_min] = val;
    }

    fn get_point(&self, x: usize, y: usize) -> char {
        self.m[x - self.x_min][y - self.y_min]
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.m[0].len() {
            for x in 0..self.m.len() {
                print!("{}", self.m[x][y]);
            }
            println!();
        }
    }

    pub fn run(&mut self) -> u64 {
        let mut sand_pos = self.sand_entry.as_point();
        let mut rest_count = 0;
        loop {
            if sand_pos.y + 1 > self.y_max {
                return rest_count;
            } else if self.get_point(sand_pos.x, sand_pos.y + 1) == '.' {
                sand_pos.y += 1;
            } else if sand_pos.x - 1 < self.x_min {
                return rest_count;
            } else if self.get_point(sand_pos.x - 1, sand_pos.y + 1) == '.' {
                sand_pos.y += 1;
                sand_pos.x -= 1;
            } else if sand_pos.x + 1 > self.x_max {
                return rest_count;
            } else if self.get_point(sand_pos.x + 1, sand_pos.y + 1) == '.' {
                sand_pos.y += 1;
                sand_pos.x += 1;
            } else if sand_pos.cmp(&self.sand_entry).is_eq() {
                // Jam
                return rest_count + 1;
            } else {
                // Comes to rest
                self.set_point(sand_pos.x, sand_pos.y, 'o');
                sand_pos = self.sand_entry.as_point();
                rest_count += 1;
            }
        }
    }
}

impl Day for Day14 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let shapes: Shapes = ipr.whole()?;
        let mut c = Cave::from_shapes(&shapes);
        let rest_count = c.run();
        Ok(rest_count.to_string())
    }

    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut shapes: Shapes = ipr.whole()?;

        let mut floor = Shape::new();
        floor.add_point(Point {
            x: shapes.x_min - shapes.y_max,
            y: shapes.y_max + 2,
        });
        floor.add_point(Point {
            x: shapes.x_max + shapes.y_max,
            y: shapes.y_max + 2,
        });
        shapes.add_shape(floor);

        let mut c = Cave::from_shapes(&shapes);
        let rest_count = c.run();
        c.print();
        Ok(rest_count.to_string())
    }
}
