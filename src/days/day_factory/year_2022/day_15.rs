use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

use std::collections::HashSet;
use regex::Regex;

const TARGET_LINE:i64 = 2000000;
// const TARGET_LINE:i64 = 10;

const TARGET_AREA:i64 = 4000000;
// const TARGET_AREA:i64 = 20;

static DELTAS: &[(i64, i64)] = &[(-1, 1), (1, 1), (1, -1), (-1, -1)];

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct SPoint {
    pub x: i64,
    pub y: i64,
}

impl SPoint {
    pub fn mhd(&self, d:&Self) -> i64 {
        (self.x - d.x).abs() + (self.y - d.y).abs()
    }
}

impl std::str::FromStr for SPoint {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Self{
            x: caps[1].trim().parse()?,
            y: caps[2].trim().parse()?,
        })
    }
}

struct PointSet {
    beacons: HashSet<SPoint>,
    sensors: HashSet<(SPoint, i64)>,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl PointSet {
    pub fn in_range(x:i64, y:i64, sensor:&(SPoint, i64)) -> bool {
        let d = sensor.0.mhd(&SPoint{x,y});
        d <= sensor.1
    }

    pub fn count_y_line(&self, y:i64) -> u64{
        let mut total = 0;
        'outer: for x in self.x_min..self.x_max+1 {
            for s in &self.sensors {
                if Self::in_range(x, y, s) {
                    if !self.beacons.contains(&SPoint{x,y}) {
                        total += 1;
                    }
                    continue 'outer;
                }
            }
        }
        total
    }

    pub fn empty_spot(&self, a:i64) -> i64{
        // looking for a point that is d+1 away from all shapes and in our target area
        for (p, r) in &self.sensors {

            for outside_x in 0..r+2{
                let outside_y = r-outside_x+1;
                
                for (sx, sy) in DELTAS {
                    let x = p.x+(outside_x*sx);
                    let y = p.y+(outside_y*sy);
                    if !((0..=a).contains(&x) && (0..=a).contains(&y)){
                        continue
                    }
                    let mut inside = false;
                    for s in &self.sensors {
                        if Self::in_range(x, y, s) {
                            inside = true;
                        }
                    }
                    if !inside {
                        println!("{}, {}",x,y);
                        return (x * 4000000) + y;
                    }
                }
            }
        }
        0
    }
}

impl std::str::FromStr for PointSet {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ls: Vec<&str> = s.lines().collect();
        let mut ps = Self{
            beacons: HashSet::new(),
            sensors: HashSet::new(),
            x_min: i64::MAX,
            x_max: i64::MIN,
            y_min: i64::MAX,
            y_max: i64::MIN,
        };
        for l in ls {
            let re = Regex::new(r"Sensor at (x=-?\d+, y=-?\d+): closest beacon is at (x=-?\d+, y=-?\d+)").unwrap();
            let caps = re.captures(l).unwrap();
            let sensor: SPoint = caps[1].parse()?;
            let beacon: SPoint = caps[2].parse()?;

            let d = sensor.mhd(&beacon);
            ps.beacons.insert(beacon.clone());
            ps.sensors.insert((sensor.clone(),d));
            
            let y_range = sensor.y-d..sensor.y+d;
            let x_range = sensor.x-d..sensor.x+d;

            if y_range.contains(&TARGET_LINE) {
                if x_range.start < ps.x_min {
                    ps.x_min = x_range.start;
                }
                if x_range.end > ps.x_max {
                    ps.x_max = x_range.end;
                }
                if y_range.start < ps.y_min {
                    ps.y_min = y_range.start;
                }
                if y_range.end > ps.y_max {
                    ps.y_max = y_range.end;
                }
            }
            
        }
        Ok(ps)
    }
}

pub struct Day15{}

impl Day for Day15 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data:PointSet = ipr.whole()?;
        Ok(data.count_y_line(TARGET_LINE).to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data:PointSet = ipr.whole()?;
        Ok(data.empty_spot(TARGET_AREA).to_string())
    }
}