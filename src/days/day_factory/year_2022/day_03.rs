use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

fn find_matches_2(v1: &Vec<char>, v2: &Vec<char>) -> Vec<char> {
    let mut matches: Vec<char> = Vec::new();
    for c1 in v1 {
        if v2.contains(c1) && !matches.contains(c1) {
            matches.push(*c1);
        }
    }
    matches
}

fn find_match_3(v1: &Vec<char>, v2: &Vec<char>, v3: &Vec<char>) -> char {
    find_matches_2(&find_matches_2(v1, v2),v3)[0]
}

struct BackPack {
    pub side1: Vec<char>,
    pub side2: Vec<char>,
}

impl BackPack {
    pub fn find_match(&self) -> Vec<char> {
        find_matches_2(&self.side1, &self.side2)
    }

    pub fn get_score(&self) -> u32 {
        let mut total: u32 = 0;
        let matches = self.find_match();
        for m in matches {
            total += Self::calc_score(m);
        }
        total
    }

    pub fn calc_score(m: char) -> u32 {
        match m {
            m if m >= 'a' => u32::from(m)-u32::from('a')+1,
            _ => u32::from(m) - u32::from('A') + 27,
        }
    }

    pub fn get_all(&self) -> Vec<char> {
        let mut r = self.side1.clone();
        r.extend(&self.side2);
        r
    }

    pub fn find_match_3_score(&self, v2: &Self, v3: &Self) -> u32 {
        Self::calc_score(find_match_3(&self.get_all(), &v2.get_all(), &v3.get_all()))
    }

}

impl std::str::FromStr for BackPack {
    type Err = std::char::ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self{
            side1: s[0..s.len()/2].chars().collect(),
            side2: s[s.len()/2..].chars().collect(),
        })
    }
}

pub struct Day03{}

impl Day for Day03 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let back_packs: Vec<BackPack> = ipr.vec_1d_newln()?;
        let mut total = 0;
        for b in back_packs {
            total += b.get_score();
        }
        Ok(total.to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let back_packs: Vec<BackPack> = ipr.vec_1d_newln()?;

        let groups = back_packs.len()/3;
        let mut total = 0;

        for g in 0..groups {
            total += back_packs[g*3].find_match_3_score(&back_packs[(g*3)+1], &back_packs[(g*3)+2]);
        }
        Ok(total.to_string())
    }
}