mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use clap::{Parser, ValueEnum};
use std::fs;
use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display, Parser, ValueEnum)]
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

fn main() {
    let args = Args::parse();
    let input = fs::read_to_string("input.txt").unwrap();

    match args.day {
        1 => run_day(args.part, day01::part_a, day01::part_b, &input),
        2 => run_day(args.part, day02::part_a, day02::part_b, &input),
        3 => run_day(args.part, day03::part_a, day03::part_b, &input),
        4 => run_day(args.part, day04::part_a, day04::part_b, &input),
        5 => run_day(args.part, day05::part_a, day05::part_b, &input),
        6 => run_day(args.part, day06::part_a, day06::part_b, &input),
        7 => run_day(args.part, day07::part_a, day07::part_b, &input),
        8 => run_day(args.part, day08::part_a, day08::part_b, &input),
        _ => panic!(),
    }
}

fn run_day(part: Part, part_a: fn(input: &String), part_b: fn(input: &String), input: &String) {
    match part {
        Part::A => part_a(input),
        Part::B => part_b(input),
        Part::Both => {
            part_a(input);
            part_b(input);
        }
    }
}
