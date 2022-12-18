use std::error::Error;
use std::cmp::max;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::Point;

use std::collections::HashSet;

struct WindFactory {
    c: Vec<char>,
    pos: usize,
}

impl std::str::FromStr for WindFactory {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { 
            c: s.chars().collect::<Vec<char>>(),
            pos: 0,
        })
    }
}

impl WindFactory {
    pub fn next(& mut self) -> char {
        let val = self.c[self.pos];
        self.pos += 1;
        if self.pos >= self.c.len() {
            self.pos = 0;
        }
        val
    }
}
#[derive(Clone)]
struct Shape {
    points: HashSet<Point>,
    pos: Point,
    width: usize,
    height: usize,
}

impl Shape {
    pub fn new() -> Self {
        Self { points: HashSet::new(), pos: Point::new(), width: 0, height: 0}
    }

    pub fn get_points(&self) -> HashSet<Point> {
        self.get_points_offset(0, 0)
    }

    pub fn get_points_offset(&self, x_off:i64, y_off:i64) -> HashSet<Point> {
        let mut out = HashSet::new();

        for p in &self.points {
            let x = (((p.x + self.pos.x) as i64) + x_off) as usize;
            let y = (((p.y + self.pos.y) as i64) + y_off) as usize;
            out.insert(Point{x,y});
        }

        out
    }

    pub fn left(&self) -> usize{
        self.pos.x
    }

    pub fn right(&self) -> usize{
        self.pos.x + self.width - 1
    }

    pub fn bottom(&self) -> usize{
        self.pos.y
    }

    pub fn top(&self) -> usize{
        self.pos.y + self.height - 1
    }
}


struct ShapeFactory {
    s: Vec<Shape>,
    pos: usize,
}

impl ShapeFactory {
    pub fn new() -> Self{
        let mut shapes = Vec::new();
    
        let s = Shape {
            points: vec![Point{x:0,y:0}, Point{x:1,y:0}, Point{x:2,y:0}, Point{x:3,y:0}].into_iter().collect(),
            pos: Point { x: 0, y: 0 },
            width: 4, height: 1,
        };
        shapes.push(s);

        let s = Shape {
            points: vec![Point{x:1,y:0}, Point{x:0,y:1}, Point{x:1,y:1}, Point{x:2,y:1}, Point{x:1,y:2}].into_iter().collect(),
            pos: Point { x: 0, y: 0 },
            width: 3, height: 3,
        };
        shapes.push(s);

        let s = Shape {
            points: vec![Point{x:0,y:0}, Point{x:1,y:0}, Point{x:2,y:0}, Point{x:2,y:1}, Point{x:2,y:2}].into_iter().collect(),
            pos: Point { x: 0, y: 0 },
            width: 3, height: 3,
        };
        shapes.push(s);

        let s = Shape {
            points: vec![Point{x:0,y:0}, Point{x:0,y:1}, Point{x:0,y:2}, Point{x:0,y:3}].into_iter().collect(),
            pos: Point { x: 0, y: 0 },
            width: 1, height: 4,
        };
        shapes.push(s);

        let s = Shape {
            points: vec![Point{x:0,y:0}, Point{x:1,y:0}, Point{x:0,y:1}, Point{x:1,y:1}].into_iter().collect(),
            pos: Point { x: 0, y: 0 },
            width: 2, height: 2,
        };
        shapes.push(s);

        Self {
            s: shapes,
            pos: 0,
        }
    }

    pub fn next(& mut self) -> Shape {
        let val = self.s[self.pos].clone();
        self.pos += 1;
        if self.pos >= self.s.len() {
            self.pos = 0;
        }
        val
    }
}

enum NextStage {
    New,
    Wind,
    Fall,
}

struct Board {
    width: usize,
    height: usize,
    points: HashSet<Point>,
    shape_list: Vec<Shape>,
}

impl Board {
    pub fn new() -> Self {
        Self { width: 7, height: 0, points: HashSet::new(), shape_list: Vec::new()}
    }

    fn clash(&self, shape: &Shape, x_off:i64, y_off:i64) -> bool{
        for p in shape.get_points_offset(x_off, y_off) {
            if self.points.contains(&p) {
                return true;
            }
        }
        false
    }

    pub fn add_shape(&mut self, shape: &Shape) {
        for p in shape.get_points() {
            self.points.insert(p.clone());
        }
        if shape.top() + 1 > self.height {
            self.height = shape.top() + 1;
        }
    }

    pub fn run(&mut self, wind: &mut WindFactory, shapes: &mut ShapeFactory, rocks: u64) -> usize{

        let mut next_stage = NextStage::New;

        let mut shape = Shape::new();
        let mut rock_count = 0;

        while  rock_count < rocks + 1 {
            match next_stage {
                NextStage::New => {
                    if rock_count == rocks {
                        break;
                    }
                    shape = shapes.next();
                    shape.pos = Point{x: 2, y: self.height+3};
                    rock_count += 1;
                    if rock_count % 100000 == 0 {
                        println!("{}% - {} of {}", (rock_count as f64 / rocks as f64) * 100_f64, rock_count, rocks);
                    }
                    next_stage = NextStage::Wind;
                },
                NextStage::Wind => {
                    let w = wind.next();
                    if w == '>' {
                        if shape.right() + 1 < self.width && !self.clash(&shape, 1, 0) {
                                shape.pos.x += 1;
                        }
                    } else if shape.left() > 0 && !self.clash(&shape, -1, 0){
                        shape.pos.x -= 1;
                    }
                    next_stage = NextStage::Fall;
                },
                NextStage::Fall => {
                    if shape.bottom() == 0 || self.clash(&shape, 0, -1){
                        self.shape_list.push(shape.clone());
                        self.add_shape(&shape);
                        next_stage = NextStage::New;
                        // self.print();
                        continue;
                    } else {
                        shape.pos.y -= 1;
                    }

                    next_stage = NextStage::Wind;
                },
            }
            // self.print_shape(&shape);
        }

        self.height
    }


    pub fn find_pattern(&self, num_shapes: usize, confirm_size: usize) -> (usize, usize) {
        let max_size = (self.shape_list.len() / confirm_size) / num_shapes;

        for block_size in 1..max_size-1 {
            let pattern_len = block_size*num_shapes;

            for test_start in 0..self.shape_list.len() - confirm_size*pattern_len {
                let mut found = true;
                'pattern_for: for i in test_start..test_start+pattern_len {
                    
                    for check in 1..confirm_size+1 {
                        if self.shape_list[i].pos.x != self.shape_list[i+(check * pattern_len)].pos.x {
                            found = false;
                            break 'pattern_for;
                        }
                    }
                }
                if found {
                    return (test_start, pattern_len);
                }
            }
        }

        (0,0)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let c = if self.points.contains(&Point{x,y}) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    #[allow(dead_code)]
    pub fn print_shape(&self, shape: &Shape) {
        for y in (0..max(self.height, shape.top()+1) ).rev() {
            for x in 0..self.width {
                let c = if self.points.contains(&Point{x,y}) {
                    '#'
                } else if shape.get_points().contains(&Point{x,y}) {
                    '@'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

}


pub struct Day17{}

impl Day for Day17 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut wind: WindFactory = ipr.whole()?;
        let mut shapes = ShapeFactory::new();
        let mut board = Board::new();
        let result = board.run(& mut wind, & mut shapes, 2022);
        Ok(result.to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut wind: WindFactory = ipr.whole()?;
        let mut shapes = ShapeFactory::new();
        let mut board = Board::new();
        board.run(& mut wind, & mut shapes, 10000);

        let (p_start, p_size) = board.find_pattern(shapes.s.len(), 4);

        println!("p_start = {}, p_size = {}", p_start, p_size);

        let target = 1000000000000;
        let multiples = (target - p_start) / p_size;
        let remainder = (target - p_start) % p_size;
        let start_height = board.shape_list[p_start].top();
        let pattern_height = board.shape_list[p_start + p_size].top() - board.shape_list[p_start].top();
        let remainder_height = board.shape_list[p_start + p_size + remainder].top() - board.shape_list[p_start + p_size].top();
        let result = start_height + (pattern_height*multiples) + (remainder_height);

        Ok(result.to_string())
    }
}