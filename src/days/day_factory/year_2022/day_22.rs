use std::{error::Error};
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::DOUBLE_NEW_LINE;

use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct SPoint {
    pub x: i64,
    pub y: i64,
}


enum Instruction {
    Move(i64),
    Rotate(char),
}

struct Instructions {
    i: VecDeque<Instruction>
}

impl std::str::FromStr for Instructions {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = Self{
            i: VecDeque::new(),
        };
        
        let re = Regex::new(r"((?:\d+)|(?:[LR]))").unwrap();
        let caps = re.captures_iter(s);

        for c in caps {
            let v = c.get(1).unwrap().as_str();
            if v.chars().next().unwrap().is_ascii_digit() {
                i.i.push_back(Instruction::Move(v.parse()?))
            } else {
                i.i.push_back(Instruction::Rotate(v.chars().next().unwrap()))
            }
        }

        Ok(i)  
    }
}

#[derive(PartialEq, Eq)]
enum CellType {
    Space,
    Wall,
    Void,
}

impl CellType {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Space,
            '#' => Self::Wall,
            _ => Self::Void,
        }
    }
}

struct Map {
    points: HashMap<SPoint, CellType>,
    width: i64,
    height: i64,
}

impl Map {
    pub fn add_point(& mut self, x: i64, y: i64, v: CellType) {
        if x + 1 > self.width {
            self.width = x + 1;
        }
        if y + 1 > self.height {
            self.height = y + 1;
        }
        self.points.insert(SPoint{x,y}, v);
    }

    pub fn get(&self, x: i64, y: i64) -> &CellType {
        let t = self.points.get(&SPoint{x,y});
        match t {
            Some(_) => t.unwrap(),
            None => &CellType::Void,
        }
    }
}

impl std::str::FromStr for Map {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m = Map{
            points  : HashMap::new(),
            width: 0,
            height: 0,
        };
        
        let ls:Vec<&str> = s.lines().collect();
        for (y, line) in ls.iter().enumerate() {
            for (x, v) in line.chars().enumerate() {
                m.add_point(x as i64, y as i64, CellType::from_char(v));
            }
        }

        Ok(m)
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn rotate(&self, i:char) -> Self {
        match i {
            'L' => self.rotate_left(),
            _ => self.rotate_rigth(),
        }
    }

    pub fn rotate_left(&self) -> Self {
        match self {
            Self::Left => Self::Down,
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Down => Self::Right,
        }
    }

    pub fn rotate_rigth(&self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Up => Self::Right,
            Self::Down => Self::Left,
        }
    }

    pub fn score(&self) -> i64 {
        match self {
            Self::Left => 2,
            Self::Right => 0,
            Self::Up => 3,
            Self::Down => 1,
        }
    }
}


// 0
//213
// 5
// 4

//Sample
//  0
//421
//  53

//Actual
// 03
// 1
//25
//4

fn move_side_left(exit: & mut SPoint, origin: &SPoint, target: &SPoint, face_size: i64) {
    let xd = exit.x - origin.x;
    let yd = exit.y - origin.y;
    exit.x = target.x + yd;
    exit.y = target.y + ((face_size - xd) -1);        
}

fn move_side_right(exit: & mut SPoint, origin: &SPoint, target: &SPoint, face_size: i64) {
    let xd = exit.x - origin.x;
    let yd = exit.y - origin.y;
    exit.x = target.x + ((face_size - yd) -1);
    exit.y = target.y + xd;        
}

fn move_side_other(exit: & mut SPoint, origin: &SPoint, target: &SPoint, face_size: i64) {
    let xd = exit.x - origin.x;
    let yd = exit.y - origin.y;
    exit.x = target.x + ((face_size - yd) -1);
    exit.y = target.y + ((face_size - xd) -1);
}

struct Cube {
    current: i64,
    corners: Vec<SPoint>,
    face_size: i64,
}

impl Cube {
    pub fn from_map(m: &Map) -> Self{

        let mut c = Self {
            current: 0,
            corners: Vec::new(),
            face_size: std::cmp::max(m.width, m.height)/4,
        };

        if c.face_size == 4 {
            c.corners.push( SPoint { x: c.face_size*2, y: 0 });
            c.corners.push( SPoint { x: c.face_size*2, y: c.face_size });
            c.corners.push( SPoint { x: c.face_size, y: c.face_size });
            c.corners.push( SPoint { x: c.face_size*3, y: c.face_size*2 });
            c.corners.push( SPoint { x: 0, y: c.face_size});
            c.corners.push( SPoint { x: c.face_size*2, y: c.face_size*2 });
        } else {
            c.corners.push( SPoint { x: c.face_size, y: 0 });
            c.corners.push( SPoint { x: c.face_size, y: c.face_size });
            c.corners.push( SPoint { x: 0, y: c.face_size*2 });
            c.corners.push( SPoint { x: c.face_size*2, y: 0 });
            c.corners.push( SPoint { x: 0, y: c.face_size*3 });
            c.corners.push( SPoint { x: c.face_size, y: c.face_size*2 });
        }
        c

    }

    fn get_face(&self, p: SPoint) -> i64 {
        for i in 0..self.corners.len() {
            if p.x >= self.corners[i].x && p.x < self.corners[i].x + self.face_size &&
               p.y >= self.corners[i].y && p.y < self.corners[i].y + self.face_size {
                return i as i64;
               }
        }
        -1
    }

    fn jump_face(&self, p: SPoint, d: Direction) -> (i64, SPoint, Direction) {
        if self.face_size == 4 {
            match d {
                Direction::Left => {
                    match self.current {
                        0 => {
                            let mut np = p;
                            move_side_left(& mut np, &self.corners[0], &self.corners[2], self.face_size);
                            (2, np, d.rotate_left())
                        },
                        4 => {
                            let mut np = p;
                            move_side_right(& mut np, &self.corners[4], &self.corners[3], self.face_size);
                            (3, np, d.rotate_left())
                        },
                        _ => { // 5
                            let mut np = p;
                            move_side_right(& mut np, &self.corners[5], &self.corners[2], self.face_size);
                            (2, np, d.rotate_left())
                        },
                    }
                },
                Direction::Right => {
                    match self.current {
                        0 => {
                            let mut np = p;
                            move_side_other(& mut np, &self.corners[0], &self.corners[3], self.face_size);
                            (self.get_face(np), np, d.rotate_left())
                        },
                        1 => {
                            let mut np = p;
                            move_side_right(& mut np, &self.corners[1], &self.corners[3], self.face_size);
                            (self.get_face(np), np, d.rotate_left())
                        },
                        3 => {
                            let mut np = p;
                            move_side_other(& mut np, &self.corners[3], &self.corners[0], self.face_size);
                            (self.get_face(np), np, d.rotate_left())
                        },
                    }
                },
                Direction::Up => {
                    match self.current {
                        0 => {
                            let mut np = p;
                            move_side_other(np, &self.corners[0], &self.corners[4], self.face_size);
                            (self.get_face(np), np, d.rotate_left())
                        },
                        2 => {
                            let mut np = p;
                            move_side_right(np, &self.corners[2], &self.corners[0], self.face_size);
                            (self.get_face(np), np, d.rotate_left())
                        },
                        3 => {
                            let mut np = p;
                            move_side_left(np, &self.corners[3], &self.corners[1], self.face_size);
                            (self.get_face(np), np, d.rotate_left())
                        },
                        4 => {
                            let mut np = p;
                            move_side_other(np, &self.corners[4], &self.corners[0], self.face_size);
                            (self.get_face(np), np, d.rotate_left())
                        },
                    }
                },
                Direction::Down => {
                    match self.current {
                        4 => {
                            let mut np = p;
                            move_side_other(np, &self.corners[4], &self.corners[5], self.face_size);
                            (self.get_face(np), np, d.rotate_left())
                        },
                        2 => {
                            let mut np = p;
                            move_side_right(np, &self.corners[2], &self.corners[0], self.face_size);
                            (self.get_face(np), np, d.rotate_left())
                        },
                        5 => {
                            let mut np = p;
                            move_side_left(np, &self.corners[3], &self.corners[1], self.face_size);
                            (self.get_face(np), np, d.rotate_left())
                        },
                        3 => {
                            let mut np = p;
                            move_side_other(np, &self.corners[4], &self.corners[0], self.face_size);
                            (self.get_face(np), np, d.rotate_left())
                        },
                    }
                },
            }
        } else {
            match d {
                Direction::Left => todo!(),
                Direction::Right => todo!(),
                Direction::Up => todo!(),
                Direction::Down => todo!(),
            }
        }
    }
}

struct Passcode {
    map: Map,
    instructions: Instructions,
    position: SPoint,
    direction: Direction,
}

impl Passcode {
    fn set_start_point(& mut self) {
        for x in 0..self.map.width {
            if self.map.get(x, 0) == &CellType::Space {
                self.position = SPoint{x, y: 0};
                return;
            }
        }   
    }

    fn move_pos_hori(& mut self, distance: i64) {
        let mut d = 0;
        let mut temp_pos = self.position;
        let mut next_point = temp_pos;

        while d < distance.abs() {
            
            if distance > 0{
                next_point.x += 1;
            } else {
                next_point.x -= 1;
            }
            
            if next_point.x > self.map.width {
                next_point.x = 0;
            }

            if next_point.x < 0 {
                next_point.x = self.map.width - 1;
            }

            match self.map.get(next_point.x, next_point.y) {
                CellType::Space => temp_pos = next_point,
                CellType::Wall => break,
                CellType::Void => continue,
            }
            d += 1;
        }
        self.position = temp_pos;
    }

    fn move_pos_virt(& mut self, distance: i64) {
        let mut d = 0;
        let mut temp_pos = self.position;
        let mut next_point = temp_pos;
        
        while d < distance.abs() {
            if distance > 0{
                next_point.y += 1;
            } else {
                next_point.y -= 1;
            }
            
            if next_point.y > self.map.height {
                next_point.y = 0;
            }
            
            if next_point.y < 0 {
                next_point.y = self.map.height - 1;
            }

            match self.map.get(next_point.x, next_point.y) {
                CellType::Space => temp_pos = next_point,
                CellType::Wall => break,
                CellType::Void => continue,
            }
            d += 1;
        }
        self.position = temp_pos;
    }

    fn move_pos(& mut self, d: i64) {
        match self.direction {
            Direction::Left => self.move_pos_hori(-d),
            Direction::Right => self.move_pos_hori(d),
            Direction::Up => self.move_pos_virt(-d),
            Direction::Down => self.move_pos_virt(d),
        }
    }

    fn apply_instruction(& mut self, i:usize) {
        match self.instructions.i[i] {
            Instruction::Move(x) => self.move_pos(x),
            Instruction::Rotate(x) => self.direction = self.direction.rotate(x),
        }
    }

    pub fn apply_instructions(& mut self) {
        // println!("Start Point");
        // self.print();
        for i in 0..self.instructions.i.len() {
            self.apply_instruction(i);
            // println!("After {} of {}", i, self.instructions.i.len());
            // self.print();
        }
    }
    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if self.position == (SPoint{x,y}) {
                    print!("*");
                } else {
                    match self.map.get(x, y) {
                        CellType::Space => print!("."),
                        CellType::Wall => print!("#"),
                        CellType::Void => print!("_"),
                    }
                }
            }
            println!();
        }
        println!();
    }

    pub fn get_code(&self) -> i64{
        (1000 * (self.position.y + 1)) + (4 * (self.position.x + 1)) + self.direction.score()
    }
}

impl std::str::FromStr for Passcode {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(&DOUBLE_NEW_LINE).collect();
        let mut pc = Self{
            map: split[0].parse()?,
            instructions: split[1].parse()?,
            position: SPoint { x: 0, y: 0 },
            direction: Direction::Right,
        };
        pc.set_start_point();
        Ok(pc)
    }
}

pub struct Day22{}

impl Day for Day22 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut game:Passcode = ipr.whole()?;
        game.apply_instructions();
        let code = game.get_code();
        Ok(code.to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        Ok(ipr.fullname()?)
    }
}