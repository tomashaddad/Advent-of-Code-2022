use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

    let file = File::open("./src/data/p01-input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    if args.part == 1 {
        part1(reader);
    } else {
        part2(reader);
    }
}

fn part1(reader: BufReader<File>) {
    let mut max = 0;
    let mut current = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
            continue;
        }
        let calorie = line.parse::<i32>();
        if calorie.is_ok() {
            current += calorie.unwrap();
        }
    }

    println!("max: {}", max);
}

fn part2(reader: BufReader<File>) {
    let mut top_three = [0, 0, 0];
    let mut current = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            let min = top_three.iter_mut().min().unwrap();
            if current > *min {
                *min = current;
            }
            current = 0;
            continue;
        }
        let calorie = line.parse::<i32>();
        if calorie.is_ok() {
            current += calorie.unwrap();
        }
    }

    println!("top three: {:?}", top_three);

    let sum: i32 = top_three.iter().sum();

    println!("sum: {}", sum);
}
