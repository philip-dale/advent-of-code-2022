use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::BingoBoard;

struct BingoGame {
    moves: Vec<u64>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    pub fn play(&mut self) -> (usize, usize) {
        for m in 0..self.moves.len() {
            for b in 0..self.boards.len() {
                self.boards[b].play(self.moves[m]);
                if self.boards[b].line_win() {
                    return (m, b);
                }
            }
        }
        return (0,0);
    }

    pub fn play_till_end(&mut self) -> (usize, usize) {

        for m in 0..self.moves.len() {
            let mut all_won = true;
            let mut last_won = 0;
            for b in 0..self.boards.len() {
                if !self.boards[b].line_win() {
                    self.boards[b].play(self.moves[m]);
                    if !self.boards[b].line_win() {
                        all_won = false;
                    } else {
                        last_won = b;
                    }
                }
            }

            if all_won {
                return (m, last_won);
            }
        }
        return (0,0);
    }
}

impl std::str::FromStr for BingoGame {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = s.split("\r\n\r\n").collect();
        let first_line: Vec<&str> =  sections[0].split(',').collect();

        return Ok(BingoGame {
            moves: {
                let mut v: Vec<u64> = Vec::new();
                
                for i in first_line {
                    v.push(i.parse()?);
                }
                v
            },
            boards: {
                let mut v: Vec<BingoBoard> = Vec::new();
                for i in 1..sections.len() {
                    v.push(sections[i].parse()?)
                };
                v
            }
        })
    }
}
pub struct Day04{}

impl Day for Day04 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut data: BingoGame = ipr.whole()?;
        let (m, b) = data.play();
        return Ok((data.moves[m] * data.boards[b].get_sum(false)).to_string());
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let mut data: BingoGame = ipr.whole()?;
        let (m, b) = data.play_till_end();
        return Ok((data.moves[m] * data.boards[b].get_sum(false)).to_string());
    }
}