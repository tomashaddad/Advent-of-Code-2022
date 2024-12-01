use crate::problem::Day;

pub struct Code;

fn to_columns(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .filter_map(|line| line.split_once("   "))
        .map(|(left, right)| (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()))
        .unzip()
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let (mut left_col, mut right_col) = to_columns(input);

        left_col.sort();
        right_col.sort();

        left_col
            .iter()
            .zip(right_col.iter())
            .fold(0, |acc, (left, right)| acc + (left - right).abs())
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (left_col, right_col) = to_columns(input);

        left_col
            .iter()
            .map(|left| left * right_col.iter().filter(|&right| left == right).count() as i32)
            .sum::<i32>()
            .to_string()
    }
}
