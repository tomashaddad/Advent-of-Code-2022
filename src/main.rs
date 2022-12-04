use advent_of_code::problem::Problem;
use advent_of_code::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let problem = Problem::new(args.day);
    let solution = problem.solve(args.part);

    println!("{}", solution);
}
