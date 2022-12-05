use crate::Day;
use std::fs::read_to_string;
use std::path::Path;

pub struct Problem {
    day: i64,
    input: String,
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

impl Problem {
    pub fn new(day: i64) -> Self {
        let path = Path::new("./src/data").join(format!("d{:02}.txt", day));
        let input = read_to_string(path).expect("A file with the input data must exist");
        Self { day, input }
    }

    pub fn solve(&self, part: i64) -> String {
        let code = self.get_code();
        let solution = match part {
            1 => code.part1(&self.input),
            2 => code.part2(&self.input),
            _ => panic!("Invalid part number"),
        };

        solution
    }

    fn get_code(&self) -> &dyn Day {
        match self.day {
            1 => &day1::Code,
            2 => &day2::Code,
            3 => &day3::Code,
            4 => &day4::Code,
            5 => &day5::Code,
            _ => panic!("Invalid day number"),
        }
    }
}
