pub struct StrU64{
    pub dir: String,
    pub val: u64,
}

impl std::str::FromStr for StrU64 {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a:Vec<&str> = s.split_whitespace().collect();
        return Ok( StrU64{
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
            val = val << 1;
        }
        val = val >> 1;
        return val;
    }
}

impl std::str::FromStr for Bits {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b: Vec<char> = Vec::new();
        for c in s.chars() {
            b.push(c);
        }
        return Ok(Bits{bits: b});
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
        return Ok(Self { board: bv, marked: mv });
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
        return false;
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
        return sum;
    }
}

impl std::str::FromStr for BingoBoard {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(BingoBoard::new(s)?);
    }
}

pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl std::str::FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals:Vec<&str> = s.split(",").collect();
        Ok(Point{
            x: vals[0].trim().parse()?,
            y: vals[1].trim().parse()?,
        })
    }
}

pub struct LineVector {
    pub s: Point,
    pub e: Point,
}

impl LineVector {
    pub fn is_diag(&self) -> bool {
        return !(self.e.x == self.s.x || self.e.y == self.s.y);
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
        return y_steps+1;
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

        return (x_max+1, y_max+1);
    }
}