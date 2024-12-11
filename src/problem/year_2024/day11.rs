use std::collections::HashMap;

use crate::problem::Day;

pub struct Code;

fn blink(times: u8, mut stones: HashMap<isize, usize>) -> usize {
    for _ in 0..times {
        let mut new_stones = HashMap::new();
        for (k, v) in &stones {
            let length = k.checked_ilog10().unwrap_or(0) + 1;
            if *k == 0 {
                *new_stones.entry(1).or_default() += v;
            } else if length % 2 == 0 {
                let h = 10isize.pow(length / 2);
                let (left, right) = (k / h, k % h);
                *new_stones.entry(left).or_default() += v;
                *new_stones.entry(right).or_default() += v;
            } else {
                *new_stones.entry(k * 2024).or_default() += v;
            }
        }
        stones = new_stones;
    }
    stones.values().sum()
}

fn parse(input: &str) -> HashMap<isize, usize> {
    input
        .split_whitespace()
        .flat_map(|c| c.parse())
        .map(|n| (n, 1))
        .collect()
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        blink(25, parse(input)).to_string()
    }

    fn part2(&self, input: &str) -> String {
        blink(75, parse(input)).to_string()
    }
}
