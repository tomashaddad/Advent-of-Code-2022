use clap::Parser;

mod problem;
use problem::Problem;

use std::fs::File;

pub trait Day {
    fn part1(&self, input: &File) -> String;
    fn part2(&self, input: &File) -> String;
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: i64,

    #[arg(short, long)]
    part: i64,
}

fn main() {
    let args = Args::parse();
    let problem = Problem::new(args.day);
    let solution = problem.solve(args.part);

    println!("{}", solution);
}
