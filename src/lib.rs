use clap::Parser;

pub trait Day {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub day: i64,

    #[arg(short, long)]
    pub part: i64,
}

pub mod problem;
