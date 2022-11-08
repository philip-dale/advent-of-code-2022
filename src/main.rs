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

   /// base directory e.g input
   #[arg(short, long, default_value_t = String::from("input"))]
   base_dir: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let ipr = input_reader::InputReader {
        filename: args.day.to_string() + "-" + &args.input[..],
        directory: args.base_dir.to_string() + "/" + &args.year[..],
        stage: args.stage.to_string(),
    };

    println!("Running Year {0} day {1} stage {2} input {3}", args.year, args.day, args.stage, args.input);
    println!("{}",days::run_day(args.year.to_string() + &args.day[..], ipr)?);

    Ok(())
}


