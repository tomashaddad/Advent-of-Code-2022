use clap::Parser;
use std::fs::File;

pub trait Day {
    fn part1(&self, input: &File) -> String;
    fn part2(&self, input: &File) -> String;
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
