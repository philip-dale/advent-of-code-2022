use std::{error::Error, cmp::max};
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

impl SPoint {
    pub fn new(x: i64, y: i64) -> Self {
        SPoint{x,y}
    }
}

impl std::ops::AddAssign for SPoint {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
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
    #[allow(dead_code)]
    pub fn to_char(&self) -> char {
        match self {
            Self::Left => 'L',
            Self::Right => 'R',
            Self::Up => 'U',
            Self::Down => 'D',
        }
    }

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

struct VoidMap {
    m: HashMap<SPoint, (SPoint, i64)>,
}

impl VoidMap {
    pub fn add_map(& mut self, a: SPoint, b:SPoint, r:i64) {
        self.m.insert(a, (b, r));
        self.m.insert(b, (a, -r));
    }

    pub fn add_range(& mut self, ain: &SPoint, ad: char, bin: &SPoint, bd: char, c: i64, r: i64) {
        let mut a = *ain;
        let mut b = *bin;

        let adelta = match ad {
            'L' => SPoint{x: -1, y: 0},
            'R' => SPoint{x: 1, y: 0},
            'U' => SPoint{x: 0, y: -1},
            _ => SPoint{x: 0, y: 1}, // D
        };

        let bdelta = match bd {
            'L' => SPoint{x: -1, y: 0},
            'R' => SPoint{x: 1, y: 0},
            'U' => SPoint{x: 0, y: -1},
            _ => SPoint{x: 0, y: 1}, // D
        };


        for _i in 0..c {
            self.add_map(a, b, r);
            a += adelta;
            b += bdelta;
        }
    }

    pub fn from_map(m: Map) -> Self {
        let mut vm = Self {
            m: HashMap::new(),
        };
        let face_size = max(m.width, m.height) / 4;
        if face_size == 4 {
            // sample
            //side 0
            vm.add_range(&SPoint::new(face_size*2, 0), 'D', &SPoint::new(face_size, face_size), 'R', face_size, -1);
            vm.add_range(&SPoint::new(face_size*2, 0), 'R', &SPoint::new(face_size-1, face_size), 'L', face_size, -2);
            vm.add_range(&SPoint::new((face_size*3) - 1, 0), 'D', &SPoint::new((face_size*4) - 1, (face_size*3) - 1), 'U', face_size, -2);
            // side 1
            vm.add_range(&SPoint::new((face_size*3) - 1, face_size), 'D', &SPoint::new((face_size*4) - 1, face_size*2), 'L', face_size, 1);
            //side 2
            vm.add_range(&SPoint::new((face_size*2) - 1, (face_size*2) - 1), 'L', &SPoint::new(face_size*2, face_size*2), 'D', face_size, -1);
            // side 3
            vm.add_range(&SPoint::new(face_size*3, (face_size*3) - 1), 'R', &SPoint::new(0, (face_size*2) - 1), 'U', face_size, -1);
            // side 4
            vm.add_range(&SPoint::new(0, (face_size*2) - 1), 'R', &SPoint::new((face_size*3) - 1, (face_size*3) - 1), 'L', face_size, -2);


        } else {
            //actual
        }


        vm
    }
}

struct Passcode {
    map: Map,
    instructions: Instructions,
    position: SPoint,
    direction: Direction,
    void_map: VoidMap,
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

    fn move_pos_hori(& mut self, distance: i64, use_void_map: bool) {
        let mut d = 0;
        let mut temp_pos = self.position;
        let mut next_point = temp_pos;

        while d < distance.abs() {
            
            if distance > 0{
                next_point.x += 1;
            } else {
                next_point.x -= 1;
            }
            
            if use_void_map {
                match self.map.get(next_point.x, next_point.y) {
                    CellType::Space => temp_pos = next_point,
                    CellType::Wall => break,
                    CellType::Void => {
                        let (jump_point, r) = self.void_map.m.get(&temp_pos).unwrap();
                        if *self.map.get(jump_point.x,  jump_point.y) == CellType::Wall {
                            break;
                        }
                        // println!("r {}, {}", r, self.direction.to_char());
                        self.position = *jump_point;
                        for _c in 0..r.abs() {
                            self.direction = if *r > 0 {
                                self.direction.rotate_rigth()
                            } else {
                                self.direction.rotate_left()
                            }
                        }
                        // println!("d {}, {}", d, self.direction.to_char());
                        let new_d = if distance > 0{
                            (distance - d) - 1
                        } else {
                            (distance + d) + 1
                        };
                        self.move_pos(new_d, use_void_map);
                        return;
                    },
                }
            } else {
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
            }
            
            d += 1;
        }
        self.position = temp_pos;
    }

    fn move_pos_virt(& mut self, distance: i64, use_void_map: bool) {
        let mut d = 0;
        let mut temp_pos = self.position;
        let mut next_point = temp_pos;
        
        while d < distance.abs() {
            if distance > 0{
                next_point.y += 1;
            } else {
                next_point.y -= 1;
            }
            
            if use_void_map {
                match self.map.get(next_point.x, next_point.y) {
                    CellType::Space => temp_pos = next_point,
                    CellType::Wall => break,
                    CellType::Void => {
                        let (jump_point, r) = self.void_map.m.get(&temp_pos).unwrap();
                        if *self.map.get(jump_point.x,  jump_point.y) == CellType::Wall {
                            break;
                        }
                        self.position = *jump_point;
                        for _c in 0..r.abs() {
                            self.direction = if *r > 0 {
                                self.direction.rotate_rigth()
                            } else {
                                self.direction.rotate_left()
                            }
                        }
                        let new_d = if distance > 0{
                            (distance - d) - 1
                        } else {
                            (distance + d) + 1
                        };
                        self.move_pos(new_d, use_void_map);
                        return;
                    },
                }
            } else {

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
            }
            d += 1;
        }
        self.position = temp_pos;
    }

    fn move_pos(& mut self, d: i64, use_void_map: bool) {
        // println!("{}, {}", d, self.direction.to_char());
        match self.direction {
            Direction::Left => self.move_pos_hori(-d, use_void_map),
            Direction::Right => self.move_pos_hori(d, use_void_map),
            Direction::Up => self.move_pos_virt(-d, use_void_map),
            Direction::Down => self.move_pos_virt(d, use_void_map),
        }
    }

    fn apply_instruction(& mut self, i:usize, use_void_map: bool) {
        
        match self.instructions.i[i] {
            Instruction::Move(x) => self.move_pos(x, use_void_map),
            Instruction::Rotate(x) => self.direction = self.direction.rotate(x),
        }
    }

    pub fn apply_instructions(& mut self, use_void_map: bool) {
        // println!("Start Point");
        // self.print();
        for i in 0..self.instructions.i.len() {
            self.apply_instruction(i, use_void_map);
            // println!("After {} of {}", i, self.instructions.i.len());
            // self.print();
        }
    }
    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if self.position == (SPoint{x,y}) {
                    match self.direction {
                        Direction::Left => print!("<"),
                        Direction::Right => print!(">"),
                        Direction::Up => print!("^"),
                        Direction::Down => print!("V"),
                    };
                    
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
            void_map: VoidMap::from_map(split[0].parse()?),
        };
        pc.set_start_point();
        Ok(pc)
    }
}

pub struct Day22{}

impl Day for Day22 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut game:Passcode = ipr.whole()?;
        game.apply_instructions(false);
        let code = game.get_code();
        Ok(code.to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut game:Passcode = ipr.whole()?;
        game.apply_instructions(true);
        let code = game.get_code();
        Ok(code.to_string())
    }
}