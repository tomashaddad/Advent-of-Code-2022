use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::Day;

pub struct Code;

impl Day for Code {
    fn part1(&self, input: &File) -> String {
        let reader = BufReader::new(input);

        #[derive(PartialEq)]
        enum Shape {
            Rock,
            Paper,
            Scissors,
        }

        impl Shape {
            fn from_char(c: char) -> Self {
                match c {
                    'A' | 'X' => Shape::Rock,
                    'B' | 'Y' => Shape::Paper,
                    'C' | 'Z' => Shape::Scissors,
                    _ => panic!(),
                }
            }

            fn beats(&self, other: &Shape) -> bool {
                match self {
                    Shape::Rock => match other {
                        Shape::Scissors => true,
                        _ => false,
                    },
                    Shape::Paper => match other {
                        Shape::Rock => true,
                        _ => false,
                    },
                    Shape::Scissors => match other {
                        Shape::Paper => true,
                        _ => false,
                    },
                }
            }

            fn score_against(&self, other: &Shape) -> i32 {
                if self.beats(other) {
                    6
                } else if self == other {
                    3
                } else {
                    0
                }
            }

            fn value(&self) -> i32 {
                match self {
                    Shape::Rock => 1,
                    Shape::Paper => 2,
                    Shape::Scissors => 3,
                }
            }
        }

        let mut score = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let mut chars = line.chars();
            let opponent = Shape::from_char(chars.next().unwrap());
            let player = Shape::from_char(chars.last().unwrap());
            score += player.value() + player.score_against(&opponent);
        }
        score.to_string()
    }

    fn part2(&self, input: &File) -> String {
        return String::from("Not implemented yet.");
    }
}
