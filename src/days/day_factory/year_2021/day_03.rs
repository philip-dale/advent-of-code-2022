use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::{Bits, HighLowCounts};

pub struct Day03{}

fn count_bits(data: &Vec<Bits>, start: usize, len: usize) -> Vec<HighLowCounts> {
    let mut counts: Vec<HighLowCounts> = vec![HighLowCounts{low: 0, high: 0}; len];
    for d in data {
        for p in start..(start + len) {
            if d.bits[p] == '0' {
                counts[p-start].low += 1;
            } else {
                counts[p-start].high += 1;
            }
        }
    }
    counts
}

fn strip_unwanted(data: &mut Vec<Bits>, is_high: bool) {
    let mut phase: usize = 0;
    while data.len() > 1 {
        let counts = count_bits(data, phase, 1);

        let mut target: char = '0';
        if counts[0].high >= counts[0].low {
            target = '1';
        }

        if !is_high {
            if target == '1' {
                target = '0'
            } else {
                target = '1'
            }
        }

        data.retain(|b| b.bits[phase] == target);
        phase += 1;
    }
}

impl Day for Day03 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<Bits> = ipr.vec_1d_newln()?;
        let counts = count_bits(&data, 0, data[0].bits.len());

        let mut gamma: u64 = 0;
        let mut epsilon: u64 = 0;

        for c in counts {
            if c.high > c.low {
                gamma += 1;
            } else {
                epsilon += 1;
            }
            gamma <<= 1;
            epsilon <<= 1;
        }
        gamma >>= 1;
        epsilon >>= 1;

        Ok((gamma * epsilon).to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut data: Vec<Bits> = ipr.vec_1d_newln()?;
        let mut data2 = data.clone();

        strip_unwanted(&mut data, true);
        let oxy = data[0].to_uint();

        strip_unwanted(&mut data2, false);
        let c02 = data2[0].to_uint();

        Ok((c02 * oxy).to_string())
    }
}



