use std::cmp::Ordering;
use std::collections::HashSet;

#[cfg(windows)]
pub const DOUBLE_NEW_LINE: & str = "\r\n\r\n";
#[cfg(not(windows))]
pub const DOUBLE_NEW_LINE: & str = "\n\n";

pub struct StrU64{
    pub dir: String,
    pub val: u64,
}

impl std::str::FromStr for StrU64 {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a:Vec<&str> = s.split_whitespace().collect();
        Ok( StrU64{
            dir: a[0].to_string(),
            val: a[1].trim().parse()?,    
        })
    }
}
#[derive(Clone)]
pub struct Bits {
    pub bits: Vec<char>,
}

impl Bits {
    pub fn to_uint(&self) -> u64 {
        let mut val: u64 = 0; 
        for c in &self.bits {
            if *c == '1' {
                val += 1;
            }
            val <<= 1;
        }
        val >>= 1;
        val
    }
}

impl std::str::FromStr for Bits {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b: Vec<char> = Vec::new();
        for c in s.chars() {
            b.push(c);
        }
        Ok(Bits{bits: b})
    }
}

#[derive(Clone)]
pub struct HighLowCounts{
    pub low: u64,
    pub high: u64,
}

pub struct BingoBoard {
    board: Vec<Vec<u64>>,
    marked: Vec<Vec<bool>>,
}

impl BingoBoard {
    pub fn new(s: &str) -> Result<Self, std::num::ParseIntError> {
        let mut bv: Vec<Vec<u64>> = Vec::new();
        let mut mv: Vec<Vec<bool>> = Vec::new();
        for line in s.lines() {
            let mut lv: Vec<u64> = Vec::new();
            for n in line.split_whitespace() {
                lv.push(n.parse()?);
            }
            mv.push(vec![false; lv.len()]);
            bv.push(lv);
        }
        Ok(Self { board: bv, marked: mv })
    }

    pub fn play(&mut self, n: u64) {
        'outer: for i in 0..self.board.len()  {
            for j in 0..self.board[i].len()  {
                if self.board[i][j] == n {
                    self.marked[i][j] = true;
                    break 'outer;
                }
            }
        }
    }

    pub fn line_win(&self) -> bool {
        for i in 0..self.marked.len() {
            let mut row = true;
            for j in 0..self.marked[i].len() {
                if !self.marked[i][j] {
                    row = false;
                    break;
                }
            }
            if row {
                return true;
            }
        }

        for j in 0..self.marked[0].len() {
            let mut col = true;
            for i in 0..self.marked.len() {
                if !self.marked[i][j] {
                    col = false;
                    break;
                }
            }
            if col {
                return true;
            }
        }
        false
    }

    pub fn get_sum(&self, marked: bool) -> u64 {
        let mut sum: u64 = 0;
        for i in 0..self.marked.len() {
            for j in 0..self.marked[i].len() {
                if self.marked[i][j] == marked {
                    sum += self.board[i][j];
                }
            }
        }
        sum
    }
}

impl std::str::FromStr for BingoBoard {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BingoBoard::new(s)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}


impl Point {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Point{x:0, y:0}
    }

    pub fn as_point(&self) -> Self{
        Point { x: self.x, y: self.y }
    }

    pub fn cmp(&self, c:&Self) -> Ordering {
        if self.x == c.x {
            return self.y.cmp(&c.y);
        }
        self.x.cmp(&c.x)
    }

    #[allow(dead_code)]
    pub fn get_all_neighbours(&self, x_max_value: usize, y_max_value: usize) -> Vec<Self> {
        let x_range = match self.x {
            x if x > 0 && x < x_max_value => self.x-1..self.x+2,
            0 => 0..self.x+2,
            x if x == x_max_value => self.x-1..self.x+1,
            _ => 0..0
        };

        let y_range = match self.y {
            y if y > 0 && y < y_max_value => self.y-1..self.y+2,
            0 => 0..self.y+2,
            y if y == y_max_value => self.y-1..self.y+1,
            _ => 0..0
        };

        let mut n:Vec<Self> = Vec::new();
        for x in x_range {
            for y in y_range.clone() {
                if x == self.x && y == self.y {
                    continue;
                }
                n.push(Self{x,y});
            }
        }
        n

    }
    #[allow(dead_code)]
    pub fn get_adjacent_neighbours(&self, x_max_value: usize, y_max_value: usize) -> Vec<Self> {
        let x_range = match self.x {
            x if x > 0 && x < x_max_value => self.x-1..self.x+2,
            0 => 0..self.x+2,
            x if x == x_max_value => self.x-1..self.x+1,
            _ => 0..0
        };

        let y_range = match self.y {
            y if y > 0 && y < y_max_value => self.y-1..self.y+2,
            0 => 0..self.y+2,
            y if y == y_max_value => self.y-1..self.y+1,
            _ => 0..0
        };

        let mut n:Vec<Self> = Vec::new();
        for x in x_range {
            if x == self.x {
                continue;
            }
            n.push(Self{x, y: self.y});
        }
        for y in y_range {
            if y == self.y {
                continue;
            }
            n.push(Self{x:self.x, y});
        }
        n

    }

    pub fn get_adjacent_neighbours_min(&self, x_max_value: usize, y_max_value: usize, x_min_value: usize, y_min_value: usize) -> Vec<Self> {
        let x_range = match self.x {
            x if x > x_min_value && x < x_max_value => self.x-1..self.x+2,
            x if x == x_min_value => self.x..self.x+2,
            x if x+1 == x_min_value => self.x+1..self.x+2,
            x if x == x_max_value => self.x-1..self.x+1,
            x if x-1 == x_max_value => self.x-1..self.x,
            _ => 0..0
        };

        let y_range = match self.y {
            y if y > y_min_value && y < y_max_value => self.y-1..self.y+2,
            y if y == y_min_value => self.y..self.y+2,
            y if y+1 == y_min_value => self.y+1..self.y+2,
            y if y == y_max_value => self.y-1..self.y+1,
            y if y-1 == y_max_value => self.y-1..self.y,
            _ => 0..0
        };

        let mut n:Vec<Self> = Vec::new();
        if self.y >= y_min_value && self.y <= y_max_value{
            for x in x_range {
                if x == self.x {
                    continue;
                }
                n.push(Self{x, y: self.y});
            }
        }

        if self.x >= x_min_value && self.x <= x_max_value {
            for y in y_range {
                if y == self.y {
                    continue;
                }
                n.push(Self{x:self.x, y});
            }
        }

        n

    }
}

impl std::str::FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals:Vec<&str> = s.split(',').collect();
        Ok(Point{
            x: vals[0].trim().parse()?,
            y: vals[1].trim().parse()?,
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Point3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3D {
    pub const SIDE_DELTAS: &[Point3D] = &[Point3D{x: -1, y: 0, z: 0}, Point3D{x: 1, y: 0, z: 0}, Point3D{x: 0, y: -1, z: 0}, Point3D{x: 0, y: 1, z: 0}, Point3D{x: 0, y: 0, z: -1}, Point3D{x: 0, y: 0, z: 1}];

    pub fn get_neighbours(&self) -> HashSet<Point3D> {
        
        let mut n = HashSet::new();
        for d in Self::SIDE_DELTAS {
            n.insert(Self{x: self.x + d.x, y: self.y + d.y, z: self.z + d.z});
        }
        n
    }

    pub fn max() -> Self {
        Self{x: i64::MAX, y: i64::MAX, z: i64::MAX}
    }

    pub fn min() -> Self {
        Self{x: i64::MIN, y: i64::MIN, z: i64::MIN}
    }

    pub fn update_min_max(&self, min: & mut Self, max: &mut Self) {
        if self.x < min.x {
            min.x = self.x;
        }
        if self.x > max.x {
            max.x = self.x;
        }

        if self.y < min.y {
            min.y = self.y;
        }
        if self.y > max.y {
            max.y = self.y;
        }

        if self.z < min.z {
            min.z = self.z;
        }
        if self.z > max.z {
            max.z = self.z;
        }

    }
}

impl std::str::FromStr for Point3D {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals:Vec<&str> = s.split(',').collect();
        Ok(Self{
            x: vals[0].trim().parse()?,
            y: vals[1].trim().parse()?,
            z: vals[2].trim().parse()?,
        })
    }
}

pub struct LineVector {
    pub s: Point,
    pub e: Point,
}

impl LineVector {
    pub fn is_diag(&self) -> bool {
        !(self.e.x == self.s.x || self.e.y == self.s.y)
    }

    pub fn steps(&self) -> usize {
        let x_steps = if self.s.x > self.e.x {
            self.s.x - self.e.x
        } else {
            self.e.x - self.s.x
        };

        let y_steps = if self.s.y > self.e.y {
            self.s.y - self.e.y
        } else {
            self.e.y - self.s.y
        };

        if x_steps > y_steps {
            return x_steps+1;
        }
        y_steps+1
    }
}

impl std::str::FromStr for LineVector {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points:Vec<&str> = s.split("->").collect();
        Ok(LineVector {
            s: points[0].parse()?,
            e: points[1].parse()?,
        })
    }
}
pub struct Lines {
    pub vectors: Vec<LineVector>,
}

impl Lines {
    pub fn get_max(&self) -> (usize, usize) {
        let mut x_max = 0;
        let mut y_max = 0;

        for v in &self.vectors {
            if v.s.x > x_max {
                x_max = v.s.x;
            }
            if v.e.x > x_max {
                x_max = v.e.x;
            }
            if v.s.y > y_max {
                y_max = v.s.y;
            }
            if v.e.y > x_max {
                y_max = v.e.y;
            }
        }

        (x_max+1, y_max+1)
    }
}

pub struct CharNumGrid {
    pub cells: Vec<Vec<u32>>,
}

impl std::str::FromStr for CharNumGrid {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CharNumGrid {
            cells: {
                let mut v: Vec<Vec<u32>> = Vec::new();
                for l in s.lines() {
                    v.push(Vec::new());
                    let last = v.len()-1;
                    for c in l.chars() {
                        v[last].push(c.to_digit(10).unwrap());
                    }
                }
                v
            },
        })
    }
}

pub struct CharList {
    pub items: Vec<char>,
}

impl std::str::FromStr for CharList {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok({
            CharList {
                items: s.chars().collect(),
            }
        })
    }
}


//////////////////////////////
/// Functions
/// 

pub fn get_range(val: usize, max: usize, range: usize) -> (usize, usize){
    if val < range{
        (val, val+range)
    } else if val > max-range{
        (val-range, val)
    } else {
        (val-range, val+range)
    }
}