use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

struct Cave {
    name: String,
    flow: u64,
    connected: Vec<String>,
    working_distance: HashMap<String, u64>,
}

impl std::str::FromStr for Cave {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Valve (..) has flow rate=(\d+); tunnel(?:s)? lead(?:s)? to valve(?:s)? (.+)").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Self{
            name: caps[1].to_string(),
            flow: caps[2].parse()?,
            connected: {
                let mut v = Vec::new();
                let ns: Vec<&str> = caps[3].split(',').collect();
                for n in ns {
                    v.push(n.trim().to_string());
                }
                v
            },
            working_distance: HashMap::new(),
        })
    }
}

struct Worker {
    name: String,
    time_remaining: u64,
    preasure: u64,
}

struct CaveSystem {
    m: HashMap<String, Cave>,
    working: HashSet<String>,
}

impl std::str::FromStr for CaveSystem {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ls: Vec<&str> = s.lines().collect();
        let mut cs = Self{
            m: {
                let mut m = HashMap::new();
                for l in ls {
                    let c: Cave = l.parse()?;
                    m.insert(c.name.to_string(), c);
                }
                m
            },
            working: HashSet::new(),
        };

        for c in &cs.m {
            if c.1.flow > 0 {
                cs.working.insert(c.0.to_string());
            }
        }

        Ok(cs)
    }
}

impl CaveSystem {
    pub fn get_max_preasure(&self) -> u64 {
        let mut best = 0;
        let time_remaining = 30;
        let mut visited: HashSet<String> = HashSet::new();
        for n in &self.m.get(&String::from("AA")).unwrap().working_distance {
            let worker = Worker {
                name: n.0.to_string(),
                time_remaining: time_remaining - n.1,
                preasure: 0,
            };
            let val = self.calc_prasure(&worker, & mut visited);
            if val > best {
                best = val;
            }
        }

        best

    }

    fn calc_prasure(&self, worker_in: &Worker, visited: & mut HashSet<String>) -> u64 {

        let worker = Worker {
            name: worker_in.name.to_string(),
            time_remaining: worker_in.time_remaining -1,
            preasure: worker_in.preasure + ((worker_in.time_remaining - 1) * self.m.get(&worker_in.name).unwrap().flow)
        };

        visited.insert(worker.name.to_string());

        let mut best = worker.preasure;
        for n in &self.m.get(&worker.name).unwrap().working_distance {
            if !visited.contains(n.0) && worker.time_remaining > *n.1 {
                let worker = Worker {
                    name: n.0.to_string(),
                    time_remaining: worker.time_remaining - n.1,
                    preasure: worker.preasure,
                };
                let val = self.calc_prasure(&worker, visited);
                if val > best {
                    best = val;
                }
            }
        }
        visited.remove(&worker.name);

        best
    }

    pub fn get_max_preasure_2(&self) -> u64 {
        let mut best = 0;
        let time_remaining = 26;
        let mut visited: HashSet<String> = HashSet::new();
        for n in &self.m.get(&String::from("AA")).unwrap().working_distance {

            for e in &self.m.get(&String::from("AA")).unwrap().working_distance {
                if e.0 != n.0 {
                    
                    let worker = Worker {
                        name: n.0.to_string(),
                        time_remaining: time_remaining - n.1,
                        preasure: 0,
                    };
                    let elephant = Worker {
                        name: e.0.to_string(),
                        time_remaining: time_remaining - e.1,
                        preasure: 0,
                    };

                    let val = self.calc_prasure_2(&worker, &elephant, & mut visited);
                    if val > best {
                        best = val;
                    }
                }
            }
        }

        best

    }

    fn calc_prasure_2(&self, worker_in: &Worker, elephant_in: &Worker, visited: & mut HashSet<String>) -> u64 {
        let worker = Worker {
            name: worker_in.name.to_string(),
            time_remaining: worker_in.time_remaining -1,
            preasure: worker_in.preasure + ((worker_in.time_remaining - 1) * self.m.get(&worker_in.name).unwrap().flow)
        };
        visited.insert(worker.name.to_string());
       
        let elephant = Worker {
            name: elephant_in.name.to_string(),
            time_remaining: elephant_in.time_remaining -1,
            preasure: elephant_in.preasure + ((elephant_in.time_remaining - 1) * self.m.get(&elephant_in.name).unwrap().flow)
        };
        visited.insert(elephant.name.to_string());


        let mut best = worker.preasure + elephant.preasure;
        for n in &self.m.get(&worker.name).unwrap().working_distance {
            if !visited.contains(n.0) && worker.time_remaining > *n.1 {
                for e in &self.m.get(&elephant.name).unwrap().working_distance {
                    if !visited.contains(e.0) && elephant.time_remaining > *e.1 && e.0 != n.0{
                        let worker = Worker {
                            name: n.0.to_string(),
                            time_remaining: worker.time_remaining - n.1,
                            preasure: worker.preasure,
                        };

                        let elephant = Worker {
                            name: e.0.to_string(),
                            time_remaining: elephant.time_remaining - e.1,
                            preasure: elephant.preasure,
                        };

                        let val = self.calc_prasure_2(&worker, &elephant, visited);
                        if val > best {
                            best = val;
                        }
                    }
                }
            }
        }
        visited.remove(&worker.name);
        visited.remove(&elephant.name);

        best
    }

    fn calc_distances(& mut self) {
        let mut visited: HashSet<String> = HashSet::new();
        for ns in &self.working {
            for ne in &self.working {
                let val = self.calc_distance(ns, ne, 0, & mut visited);
                self.m.get_mut(ns).unwrap().working_distance.insert(ne.to_string(), val);
            }
        }

        // Also do AA
        let ns = &String::from("AA");
        for ne in &self.working {
            let val = self.calc_distance(ns, ne, 0, & mut visited);
            self.m.get_mut(ns).unwrap().working_distance.insert(ne.to_string(), val);
        }
    }

    fn calc_distance(&self, start: &String, end: &String, steps: u64, visited: & mut HashSet<String>) -> u64{

        if start == end {
            return steps;
        }
        visited.insert(start.to_string());
        let steps = steps + 1;
        let mut best = u64::MAX;
        for n in &self.m.get(start).unwrap().connected {
            if !visited.contains(n) {
                let dist = self.calc_distance(n, end, steps, visited);
                if dist < best {
                    best = dist;
                }
            }
            
        }
        visited.remove(start);

        best
    }

}
pub struct Day16{}

impl Day for Day16 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut cave_system: CaveSystem = ipr.whole()?;
        cave_system.calc_distances();
        Ok(cave_system.get_max_preasure().to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut cave_system: CaveSystem = ipr.whole()?;
        cave_system.calc_distances();
        Ok(cave_system.get_max_preasure_2().to_string())
    }
}