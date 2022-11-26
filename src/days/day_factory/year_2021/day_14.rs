use std::error::Error;
use std::collections::HashMap;

use crate::input_reader;
use crate::days::day_factory::Day;

struct Polymer {
    val: HashMap<String, u64>,
    instructions: HashMap<String, String>,
    last_char: String
}

fn hash_add_or_update(h:& mut HashMap<String, u64>, k:String, count:u64) {
    h.entry(k).and_modify(|v| {*v += count}).or_insert(count);
}

impl Polymer {
    pub fn step(& mut self) {
        let mut new:HashMap<String, u64> = HashMap::new();
        for (k, count) in self.val.iter() {

            let r = self.instructions.get(k).unwrap();
            let v1 = k[0..1].to_string() + r;
            let v2 = r.to_string() + &k[1..2].to_string();
            hash_add_or_update(& mut new, v1, *count);
            hash_add_or_update(& mut new, v2, *count);
        }

        self.val = new;
    }

    pub fn steps(& mut self, count: usize) {
        for _i in 0..count {
            self.step();
        }
        self.val.insert(self.last_char.to_string(), 1);
    }

    pub fn result(&self) -> u64{
        
        let mut keys: Vec<&String> = self.val.keys().collect();
        keys.sort();

        let mut min_val = u64::MAX;
        let mut max_val = 0;
        let mut current = &keys[0][0..1];
        let mut current_count = 0;

        for k in keys{
            let c = &k[0..1];
            if *c == *current {
                current_count += self.val.get(k).unwrap();
            } else {
                if current_count > max_val {
                    max_val = current_count;
                }
                if current_count < min_val {
                    min_val = current_count;
                }
                current = c;
                current_count = *self.val.get(k).unwrap();
            }
        }
        if current_count > max_val {
            max_val = current_count;
        }
        if current_count < min_val {
            min_val = current_count;
        }
        return max_val - min_val;
    }
}

impl std::str::FromStr for Polymer {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = Polymer{
            val: HashMap::new(),
            instructions: HashMap::new(),
            last_char: String::from(""),
        };
        for (i, l) in s.lines().enumerate(){
            if i == 0 {
                for i in 0..l.len()-1 {
                    hash_add_or_update(& mut p.val, l[i..i+2].to_string(),1);
                }
                p.last_char = l[l.len()-1..l.len()].to_string();
            } else if i > 1 {
                let s:Vec<&str> = l.split(" -> ").collect();
                p.instructions.insert(s[0].trim().to_string(), s[1].trim().to_string());
            }
        }
        return Ok(p);
    }
}

pub struct Day14{}

impl Day for Day14 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut poly: Polymer = ipr.whole()?;
        poly.steps(10);
        return Ok(poly.result().to_string());
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut poly: Polymer = ipr.whole()?;
        poly.steps(40);
        return Ok(poly.result().to_string());
    }
}