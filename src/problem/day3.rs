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
                let first_as_set = first.chars().collect::<HashSet<_>>();
                let second_as_set = second.chars().collect::<HashSet<_>>();
                char_to_priority(*first_as_set.intersection(&second_as_set).next().unwrap())
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &File) -> String {
        let mut sum = 0;
        for rucksacks in BufReader::new(input)
            .lines()
            .map(|line| line.unwrap().chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .chunks(3)
        {
            let mut intersection = rucksacks[0].clone();
            for rucksack in rucksacks {
                intersection = intersection.intersection(rucksack).cloned().collect();
            }
            sum += char_to_priority(*intersection.iter().next().unwrap());
        }
        sum.to_string()
    }
}
