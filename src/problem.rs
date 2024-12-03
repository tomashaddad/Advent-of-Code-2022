use std::fs::read_to_string;
use std::path::Path;

macro_rules! include_modules {
    ($year:ident, $($day:ident),+) => {
        pub mod $year {
            $(
                pub mod $day;
            )+
        }
    }
}

include_modules!(year_2022, day01, day02, day03, day04, day05, day06, day07, day08, day09, day10);
include_modules!(year_2023, day01, day02, day05, day06, day07, day08, day09, day10, day11, day14);
include_modules!(year_2024, day01, day02, day03);

pub struct Problem {
    year: u32,
    day: u32,
    input: String,
}

pub trait Day {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

impl Problem {
    pub fn new(year: u32, day: u32) -> Self {
        let path =
            Path::new(format!("./src/data/{}", year).as_str()).join(format!("d{:02}.txt", day));
        let input = read_to_string(path).expect("A file with the input data must exist");
        Self { year, day, input }
    }

    pub fn solve(&self, part: u32) -> String {
        let code = self.get_code();
        match part {
            1 => code.part1(&self.input),
            2 => code.part2(&self.input),
            _ => panic!("Invalid part number"),
        }
    }

    fn get_code(&self) -> &dyn Day {
        match self.year {
            2022 => match self.day {
                1 => &year_2022::day01::Code,
                2 => &year_2022::day02::Code,
                3 => &year_2022::day03::Code,
                4 => &year_2022::day04::Code,
                5 => &year_2022::day05::Code,
                6 => &year_2022::day06::Code,
                7 => &year_2022::day07::Code,
                8 => &year_2022::day08::Code,
                9 => &year_2022::day09::Code,
                10 => &year_2022::day10::Code,
                _ => panic!("Invalid day given for year 2022!"),
            },
            2023 => match self.day {
                1 => &year_2023::day01::Code,
                2 => &year_2023::day02::Code,
                5 => &year_2023::day05::Code,
                6 => &year_2023::day06::Code,
                7 => &year_2023::day07::Code,
                8 => &year_2023::day08::Code,
                9 => &year_2023::day09::Code,
                10 => &year_2023::day10::Code,
                11 => &year_2023::day11::Code,
                14 => &year_2023::day14::Code,
                _ => panic!("Invalid day given for year 2023!"),
            },
            2024 => match self.day {
                1 => &year_2024::day01::Code,
                2 => &year_2024::day02::Code,
                3 => &year_2024::day03::Code,
                _ => panic!("Invalid day given for year 2024!"),
            },
            _ => panic!("The year {} is invalid!", self.year),
        }
    }
}
