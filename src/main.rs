mod day1;
mod day2a;

use anyhow::{Result, anyhow};
use clap::Parser;
use clap::ValueEnum;
use std::fs;
use strum_macros::Display;

use crate::day1::run_day_1;

#[derive(Clone, Debug, Display, Parser, ValueEnum)]
#[clap(rename_all = "kebab_case")]
#[strum(serialize_all = "kebab_case")]
enum Part {
    A,
    B,
    Both,
}

/// A compilation of solutions for Advent of Code 2016.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(short, long)]
    day: usize,

    /// Part of the day to run
    #[arg(short, long, default_value_t = Part::Both)]
    part: Part,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = fs::read_to_string("input.txt")?;

    match args.day {
        1 => run_day_1(args.part, &input)?,
        _ => return Err(anyhow!("Invalid argument for day")),
    }

    Ok(())
}
