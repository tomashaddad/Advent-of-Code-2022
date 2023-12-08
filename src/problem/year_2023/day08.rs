use std::collections::HashMap;

use crate::problem::Day;

pub struct Code;

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let (instructions, network) = input.split_once("\r\n\r\n").unwrap();

        let network_map = network
            .lines()
            .map(|line| {
                let filtered = line
                    .chars()
                    .filter(|c| ![' ', '(', ')'].contains(c))
                    .collect::<String>();
                let (from, to_either) = filtered.split_once('=').unwrap();
                let (left, right) = to_either.split_once(',').unwrap();
                (from.to_owned(), (left.to_owned(), right.to_owned()))
            })
            .collect::<HashMap<_, _>>();

        let mut current = String::from("AAA");
        let mut steps = 0;

        let mut instructions = instructions.chars().cycle();

        while current != "ZZZ" {
            match instructions.next().unwrap() {
                'L' => current = network_map[&current].0.clone(),
                'R' => current = network_map[&current].1.clone(),
                _ => panic!("Invalid instruction"),
            }
            steps += 1;
        }

        steps.to_string()
    }

    fn part2(&self, input: &str) -> String {
        todo!();
    }
}
