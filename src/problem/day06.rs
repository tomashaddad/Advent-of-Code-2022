use std::collections::HashSet;

use crate::Day;

pub struct Code;

fn unique_substr_end_ofsize(size: u32, s: &str) -> u32 {
    s.chars()
        .collect::<Vec<_>>()
        .windows(size as usize)
        .enumerate()
        .find(|(_, c)| c.iter().collect::<HashSet<_>>().len() as u32 == size)
        .expect("String should contain a string of unique characters")
        .0 as u32
        + size
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        unique_substr_end_ofsize(4, input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        unique_substr_end_ofsize(14, input).to_string()
    }
}
