use std::collections::HashSet;

use crate::Day;

pub struct Code;

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        (input
            .chars()
            .collect::<Vec<_>>()
            .windows(4)
            .enumerate()
            .find(|(_, c)| c.iter().collect::<HashSet<_>>().len() == 4)
            .expect("String should contain a string of 4 unique characters")
            .0
            + 4)
        .to_string()
    }

    fn part2(&self, input: &str) -> String {
        todo!();
    }
}
