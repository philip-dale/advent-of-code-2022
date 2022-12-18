use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::Point3D;

use std::collections::HashSet;
use std::collections::HashMap;

struct Rock {
    pos: HashSet<Point3D>,
    neighbours: HashSet<Point3D>,
    min: Point3D,
    max: Point3D,
}

impl Rock {
    pub fn new(pos: Point3D) -> Self {
        Self {
            pos : {
                let mut p = HashSet::new();
                p.insert(pos);
                p
            },
            neighbours: pos.get_neighbours(),
            min: pos,
            max: pos,
        }
    }

    pub fn is_neighbour(&self, n: &Self) -> bool {
        for p in &n.pos {
            if self.neighbours.contains(p) {
                return true;
            }
        }
        false
    }

    pub fn merge(& mut self, m:Self){
        for p in m.pos {
            self.neighbours.remove(&p);
            p.update_min_max(& mut self.min, & mut self.max);
            self.pos.insert(p);
        }
        
        for n in m.neighbours {
            if !self.pos.contains(&n) {
                self.neighbours.insert(n);
            }
        }
    }

    pub fn is_edge(&self, p: &Point3D) -> bool {
        if self.neighbours.contains(p) {
            return true;
        }
        false
    }

    pub fn get_edges(&self) -> HashMap<Point3D, usize> {
        let mut edges = HashMap::new();

        for p in &self.pos {
            let neighbours = p.get_neighbours();
            for n in &neighbours {
                if self.is_edge(n) {
                    edges.entry(*n).and_modify(|p| *p += 1).or_insert(1);
                }
            }
        }
        edges
    }

    pub fn count_sides(&self) -> usize {
        let mut count = 0;
        for e in self.get_edges() {
            count += e.1;
        }
        count
    }

}

impl std::str::FromStr for Rock {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse()?))
    }
}

struct Rocks {
    r: Vec<Rock>,
    min: Point3D,
    max: Point3D,
}

impl Rocks {
    pub fn merge(& mut self) {
        let mut has_merged = true;

        'while_loop: while !self.r.is_empty() && has_merged{
            has_merged = false;
            for rs in 0..self.r.len() {
                for rm in 0..self.r.len() {
                    if self.r[rs].is_neighbour(&self.r[rm]) {
                        let rem = self.r.remove(rm);
                        self.r[rs].merge(rem);
                        has_merged = true;
                        continue 'while_loop;
                    }
                }
            }
        }
    }

    fn is_rock(&self, p: &Point3D) -> bool {
        for r2 in &self.r {
            if r2.pos.contains(p) {
                return true;
            }
        }
        false
    }

    fn is_external(&self, p: &Point3D, visited: & mut HashSet<Point3D>) -> bool{
        visited.insert(*p);
        for d in Point3D::SIDE_DELTAS {
            let current = Point3D{x: p.x + d.x, y: p.y + d.y, z: p.z + d.z};

            if current.x < self.min.x || current.x > self.max.x || 
                current.y < self.min.y || current.y > self.max.y ||
                current.z < self.min.z || current.z > self.max.z {
    
                return true;
            } else if self.is_rock(&current) {
                continue;
            } else if !visited.contains(&current) && self.is_external(&current, visited) {
                return true;
            }
        }
        false
    }

    pub fn cout_all_sides(&self) -> usize {
        let mut count = 0;
        for r in &self.r {
            count += r.count_sides();
        }
        count
    }

    pub fn count_external_sides(&self) -> usize {
        let mut count = 0;
        for r in &self.r {
            for e in r.get_edges() {
                if self.is_external(&e.0, & mut HashSet::new()) {
                    count += e.1;
                }
            }
        }
        count
    }

    fn update_min_max(& mut self) {
        for r in &self.r {
            r.min.update_min_max(& mut self.min, & mut self.max);
            r.max.update_min_max(& mut self.min, & mut self.max);
        }
    }
}

impl std::str::FromStr for Rocks {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<&str> = s.lines().collect();
        let mut out = Self {
            r: {
                let mut rs = Vec::new(); 
                for p in points {
                    rs.push(p.parse()?);
                }
                rs
            },
            min: Point3D::max(),
            max: Point3D::min(),
        };

        out.update_min_max();

        Ok(out)
    }
}

pub struct Day18{}

impl Day for Day18 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut rocks: Rocks = ipr.whole()?;
        rocks.merge();
        Ok(rocks.cout_all_sides().to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut rocks: Rocks = ipr.whole()?;
        rocks.merge();
        Ok(rocks.count_external_sides().to_string())
    }
}