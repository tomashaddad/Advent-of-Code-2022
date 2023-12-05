use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub year: u32,

    #[arg(short, long)]
    pub day: u32,

    #[arg(short, long)]
    pub part: u32,
}

mod problem;
use problem::Problem;

fn main() {
    let args = Args::parse();
    let problem = Problem::new(args.year, args.day);
    let solution = problem.solve(args.part);

    println!("{}", solution);
}
