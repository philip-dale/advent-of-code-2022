use std::error::Error;
use crate::input_reader;
use crate::days::day_factory::Day;

fn from_snafu(s: &str) -> i64{
    let mut coeficent = 1;
    let mut total = 0;

    for c in (0..s.len()).rev() {
        let v = lookup_from_snafu(s.chars().nth(c).unwrap());
        total += v * coeficent;
        coeficent *= 5;
    }
    
    total
}

fn to_snafu(val_in: i64) -> String {
    let mut val = val_in;
    let mut total = String::new();

    while val > 0 {
        let rem = val % 5;
        val /= 5;
        if rem <= 2 {
            total = lookup_to_snafu(rem) + &total.to_string()[..];
        } else {
            total = lookup_to_snafu(rem-5) + &total.to_string()[..];
            val += 1;
        }
    }

    total
}

fn lookup_from_snafu(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        _ => -2 // '='
    }
}

fn lookup_to_snafu(i: i64) -> String {
    String::from(match i {
        2 => "2",
        1 => "1",
        0 => "0",
        -1 => "-",
        _ => "=" // -2
    })
}

pub struct Day25{}

impl Day for Day25 {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        let data:Vec<String> = ipr.vec_1d_newln()?;

        let mut total = 0;
        for snafu in &data {
            total += from_snafu(snafu);
        }

        Ok(to_snafu(total))
    }
    
    fn run2(&self, _ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        Ok(String::from("There is no part 2 just a delicious smoothie for the reindear"))
    }
}