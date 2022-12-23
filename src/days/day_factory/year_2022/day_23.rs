use std::collections::{HashSet, HashMap};
use std::error::Error;
use std::ops::Range;
use crate::input_reader;
use crate::days::day_factory::Day;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct SPoint {
    pub x: i64,
    pub y: i64,
}

impl SPoint {
    fn get_range(&self, x_range: Range<i64>, y_range: Range<i64>) -> Vec<Self> {
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

    pub fn get_all_neighbours(&self) -> Vec<Self> {
        let x_range = self.x - 1 .. self.x + 2;
        let y_range = self.y - 1 .. self.y + 2;
        self.get_range(x_range, y_range)

    }
    
    pub fn get_north(&self) -> Vec<Self> {
        let x_range = self.x - 1 .. self.x + 2;
        let y_range = self.y - 1 .. self.y;
        self.get_range(x_range, y_range)
    }

    pub fn get_south(&self) -> Vec<Self> {
        let x_range = self.x - 1 .. self.x + 2;
        let y_range = self.y + 1 .. self.y + 2;
        self.get_range(x_range, y_range)
    }

    pub fn get_east(&self) -> Vec<Self> {
        let x_range = self.x + 1 .. self.x + 2;
        let y_range = self.y - 1 .. self.y + 2;
        self.get_range(x_range, y_range)
    }

    pub fn get_west(&self) -> Vec<Self> {
        let x_range = self.x - 1 .. self.x;
        let y_range = self.y - 1 .. self.y + 2;
        self.get_range(x_range, y_range)
    }
}

struct Phase {
    current: i64,
}

impl Phase {
    pub fn new() -> Self {
        Self{current: 0}
    }

    pub fn next_phase(& mut self) {
        self.current += 1;
        if self.current >= 4 {
            self.current = 0;
        }
    }

    pub fn get_check_points_inc(&self, p: &SPoint, phase_inc: i64) -> Vec<SPoint> {
        let check = (self.current + phase_inc) % 4;
        
        match check {
            0 => p.get_north(),
            1 => p.get_south(),
            2 => p.get_west(),
            _ => p.get_east()
        }
    }

    pub fn get_delta(&self, phase_inc: i64) -> SPoint {
        let check = (self.current + phase_inc) % 4;
        
        match check {
            0 => SPoint{x: 0, y: -1},
            1 => SPoint{x: 0, y: 1},
            2 => SPoint{x: -1, y: 0},
            _ => SPoint{x: 1, y: 0}
        }
    }
}

struct Planting {
    elves: HashSet<SPoint>,
    min: SPoint,
    max: SPoint,
    phase: Phase,
}

impl Planting {
    pub fn add_elf(& mut self, x: i64, y: i64) {
        if self.max.x < x {
            self.max.x = x;
        }
        if self.min.x > x {
            self.min.x = x;
        }
        if self.max.y < y {
            self.max.y = y;
        }
        if self.min.y > y {
            self.min.y = y;
        }

        self.elves.insert(SPoint{x, y});
    }

    pub fn take_turn(& mut self) -> bool {
        let mut elf_moved = false;
        let mut proposed: HashMap<SPoint, i64> = HashMap::new();
        for e in &self.elves{
            // Check for any neighbours
            let mut has_neighbour = false;
            for p in e.get_all_neighbours() {
                if self.elves.contains(&p) {
                    has_neighbour = true;
                    break;
                }
            }

            if !has_neighbour {
                continue;
            }

            for test in 0..4 {
                let mut has_neighbour = false;
                for points in self.phase.get_check_points_inc(e, test) {
                    if self.elves.contains(&points) {
                        has_neighbour = true;
                        break;
                    }
                }
                
                if !has_neighbour {
                    
                    let delta = self.phase.get_delta(test);
                    proposed.entry(SPoint{x: e.x + delta.x, y: e.y + delta.y}).and_modify(|c| *c += 1).or_insert(1);
                    break;
                }
            }
            
        }
        // need to do move only if there are no conflicts
        let mut to_remove: Vec<SPoint> = Vec::new();
        let mut to_add: Vec<SPoint> = Vec::new();
        for e in &self.elves{
            // Check for any neighbours
            let mut has_neighbour = false;
            for p in e.get_all_neighbours() {
                if self.elves.contains(&p) {
                    has_neighbour = true;
                    break;
                }
            }

            if !has_neighbour {
                continue;
            }

            for test in 0..4 {
                let mut has_neighbour = false;
                for points in self.phase.get_check_points_inc(e, test) {
                    if self.elves.contains(&points) {
                        has_neighbour = true;
                        break;
                    }
                }
                if !has_neighbour {
                    let delta = self.phase.get_delta(test);
                    if *proposed.get(&SPoint{x: e.x + delta.x, y: e.y + delta.y}).unwrap() < 2 {
                        to_remove.push(*e);
                        to_add.push(SPoint{x: e.x + delta.x, y: e.y + delta.y});
                        elf_moved = true;
                    }
                    break;
                }
            }
        }
        for p in to_remove {
            self.elves.remove(&p);
        }
        for p in to_add {
            self.add_elf(p.x, p.y);
        }
        self.update_min_max();
        elf_moved
        // may need to reduce min and max
    }

    fn update_min_max(& mut self) {
        self.min.x = i64::MAX;
        self.min.y = i64::MAX;
        self.max.x = i64::MIN;
        self.max.y = i64::MIN;

        for e in &self.elves {
            let x = e.x;
            let y = e.y;
            if self.max.x < x {
                self.max.x = x;
            }
            if self.min.x > x {
                self.min.x = x;
            }
            if self.max.y < y {
                self.max.y = y;
            }
            if self.min.y > y {
                self.min.y = y;
            }
        }


    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for y in self.min.y..self.max.y + 1 {
            print!("{}", y);
            for x in self.min.x..self.max.x + 1 {
                if self.elves.contains(&SPoint{x,y}) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    pub fn get_score(&self) -> i64{
        ((self.max.x - self.min.x + 1) * (self.max.y - self.min.y + 1)) - self.elves.len() as i64
    }
}

impl std::str::FromStr for Planting {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sl:Vec<&str> = s.lines().collect();
        let mut p = Self {
            elves: HashSet::new(),
            min: SPoint{x: i64::MAX, y: i64::MAX},
            max: SPoint{x: i64::MIN, y: i64::MIN},
            phase: Phase::new(),
        };
        for (y, line) in sl.iter().enumerate() {
            for (x, val) in line.chars().enumerate() {
                if val == '#' {
                    p.add_elf(x as i64, y as i64);
                }
            }
        }

        Ok(p)
    }
}

pub struct Day23{}

impl Day for Day23 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut planting: Planting = ipr.whole()?;
        for _i in 0..10 {
            planting.take_turn();
            planting.phase.next_phase();
        }

        Ok(planting.get_score().to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut planting: Planting = ipr.whole()?;
        let mut count:i64 = 0;
        let mut run = true;
        while run {
            run = planting.take_turn();
            planting.phase.next_phase();
            count += 1;
        }

        Ok(count.to_string())
    }
}