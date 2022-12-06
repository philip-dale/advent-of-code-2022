use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

pub struct Day06{}

fn check_n(ch: &[char], n:usize) -> bool {
    for i in 1..n {
        if ch[0] == ch[i] {
            return false;
        }
    }

    if n > 2 {
        return check_n(&ch[1..], n-1);
    }
    true
}
 
fn get_start(s: &str, n:usize) -> usize{
    let ch: Vec<char> = s.chars().collect();
    for i in 0..ch.len()-n {
        if check_n(&ch[i..], n) {
            return i+n;
        }
    }

    0
}

impl Day for Day06 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: String = ipr.whole()?;

        Ok(get_start(&data, 4).to_string())
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: String = ipr.whole()?;
        Ok(get_start(&data,14).to_string())
    }
}