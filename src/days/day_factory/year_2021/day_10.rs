use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;
use crate::days::day_factory::types::CharList;

pub struct Day10{}

struct SyntaxSection {
    pub opening: char,
    pub closing: char,
    pub corrupt_score: u64,
    pub auto_score: u64,
}

const SYNTAX_CURVED: SyntaxSection = SyntaxSection{
    opening: '(',
    closing: ')',
    corrupt_score: 3,
    auto_score: 1,
};

const SYNTAX_SQUARE: SyntaxSection = SyntaxSection{
    opening: '[',
    closing: ']',
    corrupt_score: 57,
    auto_score: 2,
};

const SYNTAX_CURLY: SyntaxSection = SyntaxSection{
    opening: '{',
    closing: '}',
    corrupt_score: 1197,
    auto_score: 3,
};

const SYNTAX_ANGLED: SyntaxSection = SyntaxSection{
    opening: '<',
    closing: '>',
    corrupt_score: 25137,
    auto_score: 4,
};

impl SyntaxSection {
    pub fn from_char(c: char) -> std::result::Result<SyntaxSection, char>{
        if c == SYNTAX_CURVED.opening || c == SYNTAX_CURVED.closing {
            return Ok(SYNTAX_CURVED);
        }
        if c == SYNTAX_SQUARE.opening || c == SYNTAX_SQUARE.closing {
            return Ok(SYNTAX_SQUARE);
        }
        if c == SYNTAX_CURLY.opening || c == SYNTAX_CURLY.closing {
            return Ok(SYNTAX_CURLY);
        }
        if c == SYNTAX_ANGLED.opening || c == SYNTAX_ANGLED.closing {
            return Ok(SYNTAX_ANGLED);
        }
        return Err(c);
    }

    pub fn is_open(c: char) -> bool {
        return c == SYNTAX_CURVED.opening || c == SYNTAX_SQUARE.opening || c == SYNTAX_CURLY.opening || c == SYNTAX_ANGLED.opening;
    }
}

enum SyntaxScan {
    Ok(),
    Corrupt(SyntaxSection),
    Inclomplete(Vec<SyntaxSection>),
}

fn scan_syntax(syntax: &Vec<char>, index: &mut usize) -> std::result::Result<SyntaxScan, char> {
    let mut expected = SyntaxSection::from_char(syntax[*index])?;
    *index += 1;

    while *index < syntax.len() {
        if syntax[*index] == expected.closing {
            *index += 1;
            
            if *index == syntax.len() || !SyntaxSection::is_open(syntax[*index]) {
                return Ok(SyntaxScan::Ok());
            }
            expected = SyntaxSection::from_char(syntax[*index])?;
            *index += 1;
            continue;
        }

        if !SyntaxSection::is_open(syntax[*index]) {
            return Ok(SyntaxScan::Corrupt(SyntaxSection::from_char(syntax[*index])?));
        }

        let r = scan_syntax(syntax, index)?;
        match r {
            SyntaxScan::Corrupt(c) => return Ok(SyntaxScan::Corrupt(c)),
            SyntaxScan::Inclomplete(mut v) => {
                v.push(expected);
                return Ok(SyntaxScan::Inclomplete(v));
            }
            SyntaxScan::Ok() => (),
        }
    }

    return Ok(SyntaxScan::Inclomplete(vec![expected]));

}

impl Day for Day10 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<CharList> = ipr.vec_1d_newln()?;
        let mut score = 0;
        for d in data {
            let mut index:usize = 0;
            let r = scan_syntax(&d.items, &mut index).unwrap();
            match r {
                SyntaxScan::Corrupt(c) => score += c.corrupt_score,
                _ => (),
            }
        }
        return Ok(score.to_string());
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: Vec<CharList> = ipr.vec_1d_newln()?;
        let mut scores: Vec<u64> = Vec::new();
        for d in data {
            let mut score = 0;
            let mut index:usize = 0;
            let r = scan_syntax(&d.items, &mut index).unwrap();
            match r {
                SyntaxScan::Inclomplete(v) => {
                    for s in v {
                        score *= 5;
                        score += s.auto_score;
                    }
                    scores.push(score);
                }
                _ => (),
            }
        }
        scores.sort();
        return Ok(scores[(scores.len()/2)].to_string());
    }
}