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
    fn part1(&self, input: &File) -> String {
        BufReader::new(input)
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let (first, second) = line.split_at(line.len() / 2);
                let first_half = first.chars().collect::<HashSet<_>>();
                second.chars().find(|c| first_half.contains(&c))
            })
            .map(|char| match char {
                Some(c) => char_to_priority(c),
                None => panic!("A duplicate character should have been found!"),
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &File) -> String {
        let reader = BufReader::new(input);
        String::from("Hello, World!")
    }
}
