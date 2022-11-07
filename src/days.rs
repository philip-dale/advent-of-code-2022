use std::error::Error;

use super::input_reader;

mod day_factory;

pub fn run_day(day: String, ipr: input_reader::InputReader) -> Result<String, Box<dyn Error>> {
    
    return day_factory::get_day(day)?.run(ipr);
}
