use crate::Day;
use std::fs::read_to_string;
use std::path::Path;

pub struct Problem {
    day: i64,
    input: String,
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

impl Problem {
    pub fn new(day: i64) -> Self {
        let path = Path::new("./src/data").join(format!("d{:02}.txt", day));
        let input = read_to_string(path).expect("A file with the input data must exist");
        Self { day, input }
    }

    pub fn solve(&self, part: i64) -> String {
        let code = self.get_code();
        match part {
            1 => code.part1(&self.input),
            2 => code.part2(&self.input),
            _ => panic!("Invalid part number"),
        }
    }

    fn get_code(&self) -> &dyn Day {
        match self.day {
            1 => &day01::Code,
            2 => &day02::Code,
            3 => &day03::Code,
            4 => &day04::Code,
            5 => &day05::Code,
            6 => &day06::Code,
            7 => &day07::Code,
            8 => &day08::Code,
            9 => &day09::Code,
            10 => &day10::Code,
            _ => panic!("Invalid day number"),
        }
    }
}
