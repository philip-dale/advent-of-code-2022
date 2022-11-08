use clap::Parser;
use std::error::Error;

pub mod input_reader;
pub mod days;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Year to run e.g. 2022
   #[arg(short, long)]
   year: String,

    /// Day to run e.g. 01
   #[arg(short, long)]
   day: String,

   /// Input type to run (S for sample and A for actual)
   #[arg(short, long)]
   input: String,

   /// stage number (1,2)
   #[arg(short, long)]
   stage: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let ipr = input_reader::InputReader {
        filename: args.year.to_string() + &args.day[..] + "-" + &args.input[..],
        directory: String::from("input/") + &args.year[..],
        stage: args.stage.to_string(),
    };

    println!("Testing File {0}", ipr.fullname()?);
    println!("----");
    println!("{}",days::run_day(args.year.to_string() + &args.day[..], ipr)?);

    Ok(())
}


