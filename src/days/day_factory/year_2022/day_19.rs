use crate::days::day_factory::Day;
use crate::input_reader;
use regex::Regex;
use std::cmp;
use std::error::Error;
use std::collections::{VecDeque, HashSet};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Materials {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}

impl Materials {
    pub fn new(ore: i64, clay: i64, obsidian: i64, geode: i64) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    #[allow(dead_code)]
    pub fn print(&self, tag: &str) {
        println!("{} - ore = {}, clay = {}, obs = {}, geode = {}", tag, self.ore, self.clay, self.obsidian, self.geode);
    } 
    
}

impl std::ops::AddAssign for Materials {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl std::ops::SubAssign for Materials {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

impl std::ops::Add<Materials> for Materials {
    type Output = Self;

    fn add(self, rhs: Materials) -> Self::Output {
        Self{
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl std::ops::Sub<Materials> for Materials {
    type Output = Self;

    fn sub(self, rhs: Materials) -> Self::Output {
        Self{
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl std::ops::Mul<i64> for Materials {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self{
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}

#[derive(Clone, Copy)]
struct Robot {
    requires: Materials,
    produces: Materials,
}

impl Robot {
    pub fn new(requires: Materials, produces: Materials) -> Self {
        Self { requires, produces,}
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        self.requires.print("Requires");
        self.produces.print("Produces");
    }
}

#[derive(Clone, Copy)]
struct BluePrint {
    ore: Robot,
    clay: Robot,
    obsidian: Robot,
    geode: Robot,
}

impl BluePrint {
    pub fn new(
        ore: Robot,
        clay: Robot,
        obsidian: Robot,
        geode: Robot,
    ) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct ProdState {
    robots: Materials,
    store: Materials,
    time_remaining: i64,
    from: usize,
}
struct Production {
    blueprints: Vec<BluePrint>,
}

fn find_time(requires: i64, store: i64, robots: i64) -> i64 {
    if requires > store {
        (((requires - store) + robots-1) / robots) + 1

    } else {
        1_i64
    }
}

impl Production {
    pub fn run_blueprint(&self, bp: &BluePrint, steps: usize) -> i64{

        let mut best = 0;

        let initial_state = ProdState{
            robots: Materials::new(1,0,0,0),
            store: Materials::new(0,0,0,0),
            time_remaining: steps as i64,
            from: 0,
        };

        let mut queue: VecDeque<ProdState> = VecDeque::new();
        // let mut seen: HashSet<ProdState> = HashSet::new();
        queue.push_back(initial_state);


        // limit the number of ore bots we have
        let max_ore_bots = cmp::max(bp.ore.requires.ore, cmp::max(bp.clay.requires.ore, cmp::max(bp.obsidian.requires.ore, bp.geode.requires.ore)));

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();

            if current.time_remaining < 0 {
                continue;
            }
            // if we have seen this state before move on.
            // if seen.contains(&current) {
            //     continue;
            // }
            // seen.insert(current);

            // calculate the best value if we dont build any more machines
            let no_build_best = (current.time_remaining * current.robots.geode) + current.store.geode;
            if no_build_best > best {
                best = no_build_best;
            }
            if current.store.geode > best {
                best = current.store.geode;
            }
            
            if current.robots.ore < max_ore_bots {
                // with current robots how long to purchase a ore bot
                let ore_time_to_build = find_time(bp.ore.requires.ore, current.store.ore, current.robots.ore);

                let mut next_state = current;
                // Mine
                next_state.store += next_state.robots * ore_time_to_build;

                // Produce Robots
                next_state.store -= bp.ore.requires;
                next_state.robots.ore += 1;

                next_state.time_remaining -= ore_time_to_build;
                // next_state.from = seen.len();
                queue.push_back(next_state);
            }

            // limit clay bots?

            // with current robots how long to purchase a clay bot
            let clay_time_to_build = find_time(bp.clay.requires.ore, current.store.ore, current.robots.ore);

            let mut next_state = current;
            // Mine
            next_state.store += next_state.robots * clay_time_to_build;

            // Produce Robots
            next_state.store -= bp.clay.requires;
            next_state.robots.clay += 1;

            next_state.time_remaining -= clay_time_to_build;
            // next_state.from = seen.len();
            queue.push_back(next_state);       


            // limit obs bots?
            if current.robots.clay > 0 { 
                // with current robots how long to purchase a obs bot
                let obs_time_to_build_ore = find_time(bp.obsidian.requires.ore, current.store.ore, current.robots.ore);
                let obs_time_to_build_clay = find_time(bp.obsidian.requires.clay, current.store.clay, current.robots.clay);
                let obs_time_to_build = cmp::max(obs_time_to_build_ore, obs_time_to_build_clay);
                let mut next_state = current;
                // Mine
                next_state.store += next_state.robots * obs_time_to_build;

                // Produce Robots
                next_state.store -= bp.obsidian.requires;
                next_state.robots.obsidian += 1;

                next_state.time_remaining -= obs_time_to_build;
                // next_state.from = seen.len();
                queue.push_back(next_state); 
            }

            // limit geods bots?
            if current.robots.obsidian > 0 {
                // with current robots how long to purchase a geo bot
                let geo_time_to_build_ore = find_time(bp.geode.requires.ore, current.store.ore, current.robots.ore);
                let geo_time_to_build_obs = find_time(bp.geode.requires.obsidian, current.store.obsidian, current.robots.obsidian);
                let geo_time_to_build = cmp::max(geo_time_to_build_ore, geo_time_to_build_obs);

                let mut next_state = current;
                // Mine
                next_state.store += next_state.robots * geo_time_to_build;

                // Produce Robots
                next_state.store -= bp.geode.requires;
                next_state.robots.geode += 1;

                next_state.time_remaining -= geo_time_to_build;
                // next_state.from = seen.len();
                queue.push_back(next_state); 
            }


        }
        println!("Best = {}", best);
        best

    }

    pub fn run(&self, steps: usize) -> i64 {
        let mut best = 0;
        for (i, bp) in self.blueprints.to_vec().iter().enumerate() {

            let result = self.run_blueprint(bp, steps) * (i as i64 + 1);
            best += result;
        }
        best
    }

    pub fn run2(&self, steps: usize) -> i64 {
        let mut best = 1;
        for (i, bp) in self.blueprints.to_vec().iter().enumerate() {
            println!("i = {}, best = {}", i, best);
            let result = self.run_blueprint(bp, steps) * (i as i64 + 1);
            best *= result;
            if i == 2{
                break;
            }
        }
        best
    }
}

impl std::str::FromStr for Production {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Blueprint (\d\d?): Each ore robot costs (\d\d?) ore. Each clay robot costs (\d\d?) ore. Each obsidian robot costs (\d\d?) ore and (\d\d?) clay. Each geode robot costs (\d\d?) ore and (\d\d?) obsidian.").unwrap();
        //
        Ok(Self {
            blueprints: {
                let mut b = Vec::new();
                for l in s.lines().collect::<Vec<&str>>() {
                    let caps = re.captures(l).unwrap();
                    b.push(BluePrint::new(
                        Robot::new(Materials::new(caps[2].parse()?, 0, 0, 0), Materials::new(1,0,0,0)),
                        Robot::new(Materials::new(caps[3].parse()?, 0, 0, 0), Materials::new(0,1,0,0)),
                        Robot::new(Materials::new(caps[4].parse()?, caps[5].parse()?, 0, 0), Materials::new(0,0,1,0)),
                        Robot::new(Materials::new(caps[6].parse()?, 0, caps[7].parse()?, 0), Materials::new(0,0,0,1)),
                    ))
                }
                b
            },
        })
    }
}
pub struct Day19 {}

impl Day for Day19 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let prod:Production = ipr.whole()?;
        let result = prod.run(24);
        Ok(result.to_string())
    }

    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let prod:Production = ipr.whole()?;
        let result = prod.run2(34);
        Ok(result.to_string())
    }
}
