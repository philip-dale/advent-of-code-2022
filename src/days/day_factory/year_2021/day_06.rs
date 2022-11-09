use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

pub struct Day06{}

struct Lanterns {
    times: Vec<u64>,
}

impl Lanterns {
    pub fn new() -> Self {
        return Lanterns {
            times: vec![0; 9],
        };
    }

    pub fn from_vec(init: Vec<usize>) -> Self {
        let mut l = Self::new();
        for v in init {
            l.times[v] += 1;
        }
        return l;
    }

    pub fn step(&mut self) {
        let mut l_next = Self::new();
        l_next.times[0] = self.times[1];
        l_next.times[1] = self.times[2];
        l_next.times[2] = self.times[3];
        l_next.times[3] = self.times[4];
        l_next.times[4] = self.times[5];
        l_next.times[5] = self.times[6];
        l_next.times[6] = self.times[7] + self.times[0];
        l_next.times[7] = self.times[8];
        l_next.times[8] = self.times[0];

        self.times = l_next.times;
    }

    pub fn sum(&self) -> u64 {
        return self.times[0] +
               self.times[1] +
               self.times[2] +
               self.times[3] +
               self.times[4] +
               self.times[5] +
               self.times[6] +
               self.times[7] +
               self.times[8];
    }

    pub fn run(&mut self, steps: usize) -> u64 {
        for _s in 0..steps {
            self.step();
        }

        return self.sum();
    }
}

impl Day for Day06 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut data = Lanterns::from_vec(ipr.vec_1d_sep(&String::from(","))?);
        let total = data.run(80);
        return Ok(total.to_string());
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut data = Lanterns::from_vec(ipr.vec_1d_sep(&String::from(","))?);
        let total = data.run(256);
        return Ok(total.to_string());
    }
}