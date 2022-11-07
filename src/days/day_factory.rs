use std::error::Error;
use std::fmt;

use super::input_reader;

mod day_202101;
mod day_202201;
mod day_202202;
mod day_202203;
mod day_202204;
mod day_202205;
mod day_202206;
mod day_202207;
mod day_202208;
mod day_202209;
mod day_202210;
mod day_202211;
mod day_202212;
mod day_202213;
mod day_202214;
mod day_202215;
mod day_202216;
mod day_202217;
mod day_202218;
mod day_202219;
mod day_202220;
mod day_202221;
mod day_202222;
mod day_202223;
mod day_202224;

#[derive(Debug)]
struct DayError(String);

impl fmt::Display for DayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for DayError {}

pub trait Day {
    fn run1(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>>;
    fn run2(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>>;
    fn run(&self, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
        match ipr.stage.as_str() {
            "1" => return self.run1(ipr),
            "2" => return self.run2(ipr),
            _ => return Ok(String::from("Unknown Stage")),
        };
    }
}

pub fn get_day(day: String) -> Result<Box<dyn Day>, Box<dyn Error>> {
    match day.as_str() {
        "202101" => return Ok(Box::new(day_202101::Day202101{})),
        "202201" => return Ok(Box::new(day_202201::Day202201{})),
        "202202" => return Ok(Box::new(day_202202::Day202202{})),
        "202203" => return Ok(Box::new(day_202203::Day202203{})),
        "202204" => return Ok(Box::new(day_202204::Day202204{})),
        "202205" => return Ok(Box::new(day_202205::Day202205{})),
        "202206" => return Ok(Box::new(day_202206::Day202206{})),
        "202207" => return Ok(Box::new(day_202207::Day202207{})),
        "202208" => return Ok(Box::new(day_202208::Day202208{})),
        "202209" => return Ok(Box::new(day_202209::Day202209{})),
        "202210" => return Ok(Box::new(day_202210::Day202210{})),
        "202211" => return Ok(Box::new(day_202211::Day202211{})),
        "202212" => return Ok(Box::new(day_202212::Day202212{})),
        "202213" => return Ok(Box::new(day_202213::Day202213{})),
        "202214" => return Ok(Box::new(day_202214::Day202214{})),
        "202215" => return Ok(Box::new(day_202215::Day202215{})),
        "202216" => return Ok(Box::new(day_202216::Day202216{})),
        "202217" => return Ok(Box::new(day_202217::Day202217{})),
        "202218" => return Ok(Box::new(day_202218::Day202218{})),
        "202219" => return Ok(Box::new(day_202219::Day202219{})),
        "202220" => return Ok(Box::new(day_202220::Day202220{})),
        "202221" => return Ok(Box::new(day_202221::Day202221{})),
        "202222" => return Ok(Box::new(day_202222::Day202222{})),
        "202223" => return Ok(Box::new(day_202223::Day202223{})),
        "202224" => return Ok(Box::new(day_202224::Day202224{})),
        _ => return Err(Box::new(DayError("Unknown Day".into()))),
    }
}