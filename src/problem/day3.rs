use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::collections::HashSet;

use crate::Day;

pub struct Code;

fn char_to_priority(mut c: char) -> u32 {
    let offset: u32 = 'A' as u32;
    match c.is_lowercase() {
        true => {
            c.make_ascii_uppercase();
            c as u32 - offset + 1
        }
        false => c as u32 - offset + 26 + 1,
    }
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let (first, second) = line.split_at(line.len() / 2);
                let first_as_set = first.chars().collect::<HashSet<_>>();
                let second_as_set = second.chars().collect::<HashSet<_>>();
                char_to_priority(*first_as_set.intersection(&second_as_set).next().unwrap())
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| line.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|group| {
                char_to_priority(
                    *group
                        .iter()
                        .cloned()
                        .reduce(|a, b| a.intersection(&b).cloned().collect())
                        .unwrap()
                        .iter()
                        .next()
                        .unwrap(),
                )
            })
            .sum::<u32>()
            .to_string()
    }
}
