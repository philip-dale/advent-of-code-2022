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
        if self == theirs {
            return 3;
        }

        if *self == RpsMove::R && *theirs == RpsMove::S {
            return 6;
        }

        if *self == RpsMove::P && *theirs == RpsMove::R {
            return 6;
        }

        if *self == RpsMove::S && *theirs == RpsMove::P {
            return 6;
        }

        0
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
}

impl std::str::FromStr for RpsGame {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self{
            theirs: RpsMove::from_char(s.chars().nth(0).unwrap()),
            mine: RpsMove::from_char(s.chars().last().unwrap()),
        })
    }
}

pub struct  RpsGame2 {
    pub theirs: RpsMove,
    pub mine: RpsMove,
}

impl RpsGame2 {
    pub fn get_score(&self) -> u32{
        self.mine.wld(&self.theirs) + self.mine.get_score()
    }
}

impl std::str::FromStr for RpsGame2 {
    type Err = std::char::ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let theirs = RpsMove::from_char(s.chars().nth(0).unwrap());
        Ok(Self{
            theirs,
            mine: RpsMove::from_wld(&theirs, s.chars().last().unwrap()),
        })
    }
}

pub struct Day02{}

impl Day for Day02 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let games: Vec<RpsGame> = ipr.vec_1d_newln()?;
        let mut total = 0;
        for m in games {
            total += m.get_score();
        }
        Ok(total.to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let games: Vec<RpsGame2> = ipr.vec_1d_newln()?;
        let mut total = 0;
        for m in games {
            total += m.get_score();
        }
        Ok(total.to_string())
    }
}