use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum RpsMove{
    R,
    P,
    S,
}

impl RpsMove {
    pub fn from_char(c: char) -> Self {
        match c {
            'A' => Self::R,
            'B' => Self::P,
            'C' => Self::S,
            'X' => Self::R,
            'Y' => Self::P,
            'Z' => Self::S,
            _ => todo!(),
        }
    }

    pub fn from_wld(m: &RpsMove, c: char) -> Self {
        match c {
            'Y' => *m,
            'X' => m.get_lose(),
            'Z' => m.get_win(),
            _ => todo!()
        }
    }

    pub fn wld(&self, theirs: &Self) ->u32 {
        match theirs {
            theirs if theirs == self => 3,
            theirs if theirs.get_win() == *self => 6,
            _ => 0,
        }
    }

    pub fn get_lose(&self) -> Self{
        match self {
            RpsMove::R => RpsMove::S,
            RpsMove::S => RpsMove::P,
            RpsMove::P => RpsMove::R,
        }
    }

    pub fn get_win(&self) -> Self {
        match self {
            RpsMove::R => RpsMove::P,
            RpsMove::S => RpsMove::R,
            RpsMove::P => RpsMove::S,
        }
    }

    pub fn get_score(&self) -> u32 {
        match self {
            Self::R => 1,
            Self::P => 2,
            Self::S => 3,
        }
    }
}


pub struct  RpsGame {
    pub theirs: RpsMove,
    pub mine: RpsMove,
}

impl RpsGame {
    pub fn get_score(&self) -> u32{
        self.mine.wld(&self.theirs) + self.mine.get_score()
    }

    fn from_str(s: &str) -> Self {
        Self{
            theirs: RpsMove::from_char(s.chars().next().unwrap()),
            mine: RpsMove::from_char(s.chars().last().unwrap()),
        }
    }

    fn from_str_wld(s: &str) -> Self {
        let theirs = RpsMove::from_char(s.chars().next().unwrap());
        Self{
            theirs,
            mine: RpsMove::from_wld(&theirs, s.chars().last().unwrap()),
        }
    }
}

pub struct Day02{}

impl Day for Day02 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<String> = ipr.vec_1d_newln()?;
        let mut games: Vec<RpsGame> = Vec::new();
        for l in data {
            games.push(RpsGame::from_str(&l));
        }

        let mut total = 0;
        for m in games {
            total += m.get_score();
        }
        Ok(total.to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<String> = ipr.vec_1d_newln()?;
        let mut games: Vec<RpsGame> = Vec::new();
        for l in data {
            games.push(RpsGame::from_str_wld(&l));
        }

        let mut total = 0;
        for m in games {
            total += m.get_score();
        }
        Ok(total.to_string())
    }
}