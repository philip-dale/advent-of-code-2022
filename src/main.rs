use clap::Parser;
use std::error::Error;

pub mod input_reader;
pub mod days;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Day to run
   #[arg(short, long, default_value_t = String::new())]
   day: String,

   // Input type to run
   #[arg(short, long, default_value_t = String::new())]
   input: String,

   // stage number (1,2)
   #[arg(short, long, default_value_t = String::new())]
   stage: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let ipr = input_reader::InputReader {
        filename: args.day.to_string() + "-" + &args.input[..],
        directory: String::from("input"),
        stage: args.stage.to_string(),
    };

    println!("Testing File {0}", ipr.fullname()?);
    println!("----");
    println!("{}",days::run_day(args.day, ipr)?);

    Ok(())
}


