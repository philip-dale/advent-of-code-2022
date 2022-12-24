use std::collections::{HashMap, VecDeque, HashSet};
use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::Point;
use num::integer::lcm;
#[derive(Clone, Copy)]
struct Blizard {
    d: char,
    p: Point,
    max_x: usize,
    max_y: usize,
    min_x: usize,
    min_y: usize,
}


impl Blizard {
    pub fn next(&self) -> Point {
        let mut new_point = self.p;
        match self.d {
            '>' => new_point.x += 1,
            '<' => new_point.x -= 1,
            '^' => new_point.y -= 1,
            _ => new_point.y += 1,
        }

        if new_point.x > self.max_x {
            new_point.x = self.min_x;
        }
        if new_point.x < self.min_x {
            new_point.x = self.max_x;
        }
        if new_point.y > self.max_y {
            new_point.y = self.min_y;
        }
        if new_point.y < self.min_y {
            new_point.y = self.max_y;
        }

        new_point
    }
}

struct WindMap {
    blizards: Vec<Blizard>,
    points: HashMap<Point, Blizard>,
    max_x: usize,
    max_y: usize,
    min_x: usize,
    min_y: usize,
}

impl WindMap {
    pub fn get_next(&self) -> Self {
        let mut new_map = Self {
            blizards: Vec::new(),
            points: HashMap::new(),
            max_x: self.max_x,
            max_y: self.max_y,
            min_x: self.min_x,
            min_y: self.min_y,
        };

        for b in &self.blizards {
            new_map.add_blizard(b.next(), b.d);
        }

        new_map
    }

    pub fn add_blizard(&mut self, p: Point, d: char) {
        self.blizards.push(Blizard { d, p, max_x: self.max_x, max_y: self.max_y, min_x: self.min_x, min_y: self.min_y });
        self.points.insert(p, Blizard { d, p, max_x: self.max_x, max_y: self.max_y, min_x: self.min_x, min_y: self.min_y });
    }

    pub fn is_safe(&self, p: &Point) -> bool {
        !self.points.contains_key(p)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.max_y + 2 {
            for x in 0..self.max_x + 2 {
                if self.points.contains_key(&Point{x,y}) {
                    print!("{}", self.points.get(&Point{x,y}).unwrap().d);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

impl std::str::FromStr for WindMap {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let sl: Vec<&str> = s.lines().collect();

        let mut wind_map = Self{
            blizards: Vec::new(),
            points: HashMap::new(),
            max_x: sl[0].len()-2,
            max_y: sl.len()-2,
            min_x: 1,
            min_y: 1,
        };

        for (y, l) in sl.iter().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '>' || c == '<' || c == '^' || c == 'v' {
                    wind_map.add_blizard(Point{x,y}, c);
                }
            }
        }

        Ok(wind_map)
    }
}

struct Crossing {
    wind_maps: Vec<WindMap>,
    max_x: usize,
    max_y: usize,
    min_x: usize,
    min_y: usize,
    start: Point,
    end: Point,
}

impl Crossing {
    pub fn from_wind_map(wind_map: WindMap) -> Self {
        let mut crossing = Crossing{
            wind_maps: Vec::new(),
            max_x: wind_map.max_x,
            max_y: wind_map.max_y,
            min_x: wind_map.min_x,
            min_y: wind_map.min_y,
            start: Point{x: 1, y: 0},
            end: Point{x: wind_map.max_x, y: wind_map.max_y + 1}
        };

        let map_count = lcm((wind_map.max_x-wind_map.min_x) + 1, (wind_map.max_y - wind_map.min_y) + 1);
        crossing.wind_maps.push(wind_map);
        for _i in 1..map_count {
            crossing.wind_maps.push(crossing.wind_maps[crossing.wind_maps.len()-1].get_next());
        }   

        crossing
    }

    pub fn run(&self, reverse: bool, wind_offset: usize) -> usize {

        let (start, end) = if reverse {
            (self.end, Point{x: self.start.x, y: self.start.y+1})
        } else {
            (self.start, Point{x: self.end.x, y: self.end.y-1})
        };

        let state = (start.as_point(), wind_offset);
        let mut state_queue: VecDeque<(Point, usize)> = VecDeque::new();
        state_queue.push_back(state);

        let mut seen: HashSet<(Point, usize)> = HashSet::new();
        seen.insert(state);

        let mut best = usize::MAX;

        while !state_queue.is_empty() {
            let (pos, time) = state_queue.pop_front().unwrap();

            if pos == end && time < best {
                best = time - wind_offset;
                break;
            }
            
            let mut neighbours = pos.get_adjacent_neighbours_min(self.max_x, self.max_y, self.min_x, self.min_y);

            neighbours.push(pos);
            for n in neighbours {
                let next_time = (time + 1) % self.wind_maps.len();
                if self.wind_maps[next_time].is_safe(&n) && !seen.contains(&(n.as_point(), time + 1)){
                    state_queue.push_back((n.as_point(), time + 1));
                    seen.insert((n, time + 1));
                }
            }

        }
        best + 1
    }
}



pub struct Day24{}

impl Day for Day24 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let wind_map: WindMap = ipr.whole()?;
        let crossing = Crossing::from_wind_map(wind_map);
        Ok(crossing.run(false, 0).to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let wind_map: WindMap = ipr.whole()?;
        let crossing = Crossing::from_wind_map(wind_map);
        let mut total = 0;
        total += crossing.run(false, total);
        total += crossing.run(true, total);
        total += crossing.run(false, total);
        Ok(total.to_string())
    }
}