use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::Day;

pub struct Code;

#[derive(PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Shape {
    fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => unreachable!(),
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

impl Outcome {
    fn from_char(c: char) -> Self {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!(),
        }
    }

    fn shape_needed_against(&self, shape: &Shape) -> Shape {
        match self {
            Outcome::Win => match shape {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            Outcome::Lose => match shape {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Outcome::Draw => shape.clone(),
        }
    }
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let mut score = 0;
        for line in input.lines() {
            let mut chars = line.chars();
            let opponent_shape = Shape::from_char(chars.next().unwrap());
            let player_shape = Shape::from_char(chars.last().unwrap());
            score += player_shape.value() + player_shape.score_against(&opponent_shape);
        }
        score.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut score = 0;
        for line in input.lines() {
            let mut chars = line.chars();
            let opponent_shape = Shape::from_char(chars.next().unwrap());
            let target_outcome = Outcome::from_char(chars.last().unwrap());
            let player_shape = target_outcome.shape_needed_against(&opponent_shape);
            score += player_shape.value() + player_shape.score_against(&opponent_shape);
        }
        score.to_string()
    }
}
