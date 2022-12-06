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
 
fn get_start(s: &str, n:usize) -> Option<usize>{
    let ch: Vec<char> = s.chars().collect();
    for i in 0..ch.len()-n {
        if check_n(&ch[i..], n) {
            return Some(i+n);
        }
    }
    None
}

impl Day for Day06 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: String = ipr.whole()?;
        match get_start(&data, 4) {
            Some(n) => Ok(n.to_string()),
            None => Ok(String::from("Not found")),
        }
    }
    
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data: String = ipr.whole()?;
        match get_start(&data,14) {
            Some(n) => Ok(n.to_string()),
            None => Ok(String::from("Not found")),
        }
    }
}