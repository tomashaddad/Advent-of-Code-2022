use crate::Day;
use std::fs::File;
use std::path::Path;

pub struct Problem {
    day: i64,
    input: File,
}

mod day1;
mod day2;
mod day3;
mod day4;

impl Problem {
    pub fn new(day: i64) -> Self {
        let path = Path::new("./src/data").join(format!("d{:02}.txt", day));
        let input = File::open(path).expect("file wasn't found.");
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
            _ => panic!("Invalid day number"),
        }
    }
}
