use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use std::collections::HashSet;
use std::collections::HashMap;

static DELTAS: &[(i64, i64, i64)] = &[(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];

fn get_neighbours(pos:&(i64, i64, i64)) -> HashSet<(i64, i64, i64)> {
    let mut n = HashSet::new();
    for d in DELTAS {
        n.insert((pos.0 + d.0, pos.1 + d.1, pos.2 + d.2 ));
    }
    n
}

struct Rock {
    pos: HashSet<(i64, i64, i64)>,
    neighbours: HashSet<(i64, i64, i64)>,
}

impl Rock {
    pub fn new(pos: (i64, i64, i64)) -> Self {
        Self {
            pos : {
                let mut p = HashSet::new();
                p.insert(pos);
                p
            },
            neighbours: get_neighbours(&pos),
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
            self.pos.insert(p);
        }
        
        for n in m.neighbours {
            if !self.pos.contains(&n) {
                self.neighbours.insert(n);
            }
        }
    }

    pub fn is_edge(&self, p: &(i64, i64, i64)) -> bool {
        if self.neighbours.contains(p) {
            return true;
        }
        false
    }

    pub fn get_edges(&self) -> HashMap<(i64, i64, i64), usize> {
        let mut edges = HashMap::new();

        for p in &self.pos {
            let neighbours = get_neighbours(p);
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

    pub fn get_min_max(&self) -> ((i64, i64, i64), (i64, i64, i64)) {
        let mut val_min = (i64::MAX, i64::MAX, i64::MAX);
        let mut val_max = (i64::MIN, i64::MIN, i64::MIN);

        for p in &self.pos {
            if p.0 < val_min.0 {
                val_min.0 = p.0;
            }
            if p.0 > val_max.0 {
                val_max.0 = p.0;
            }

            if p.1 < val_min.1 {
                val_min.1 = p.1;
            }
            if p.1 > val_max.1 {
                val_max.1 = p.1;
            }

            if p.2 < val_min.2 {
                val_min.2 = p.2;
            }
            if p.2 > val_max.2 {
                val_max.2 = p.2;
            }
        }

        (val_min, val_max)

    }

}

impl std::str::FromStr for Rock {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<&str> = s.split(',').collect();
        Ok(Self::new((points[0].parse()?, points[1].parse()?, points[2].parse()?)))
    }
}

struct Rocks {
    r: Vec<Rock>,
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

    fn is_rock(&self, p: &(i64, i64, i64)) -> bool {
        for r2 in &self.r {
            if r2.pos.contains(p) {
                return true;
            }
        }
        false
    }

    fn is_external(&self, p: &(i64, i64, i64), visited: & mut HashSet<(i64, i64, i64)>) -> bool{

        let (min, max) = self.get_min_max();

        visited.insert(*p);
        for d in DELTAS {
            let current = (p.0 + d.0, p.1 + d.1, p.2 + d.2);

            if current.0 < min.0 || current.0 > max.0 || 
                current.1 < min.1 || current.1 > max.1 ||
                current.2 < min.2 || current.2 > max.2 {
    
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

    fn get_min_max(&self) -> ((i64, i64, i64), (i64, i64, i64)) {
        let mut val_min = (i64::MAX, i64::MAX, i64::MAX);
        let mut val_max = (i64::MIN, i64::MIN, i64::MIN);
    
        for r in &self.r {
            let (r_min, r_max) = r.get_min_max();
            if r_min.0 < val_min.0 {
                val_min.0 = r_min.0;
            }
            if r_max.0 > val_max.0 {
                val_max.0 = r_max.0;
            }
    
            if r_min.1 < val_min.1 {
                val_min.1 = r_min.1;
            }
            if r_max.1 > val_max.1 {
                val_max.1 = r_max.1;
            }
    
            if r_min.2 < val_min.2 {
                val_min.2 = r_min.2;
            }
            if r_max.2 > val_max.2 {
                val_max.2 = r_max.2;
            }
        }
    
        (val_min, val_max)
    }
}

impl std::str::FromStr for Rocks {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<&str> = s.lines().collect();
        Ok(Self {
            r: {
                let mut rs = Vec::new(); 
                for p in points {
                    rs.push(p.parse()?);
                }
                rs
            },
        })
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