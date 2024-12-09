use itertools::Itertools;

use crate::problem::Day;

pub struct Code;

type Block = Option<usize>;

fn to_checksum(parsed: &Vec<Block>) -> Vec<usize> {
    let mut i = 0;
    let mut j = parsed.len() - 1;
    let mut res: Vec<usize> = vec![];
    loop {
        if let Some(left) = parsed[i] {
            res.push(left);
            i += 1;
        } else if let Some(right) = parsed[j] {
            res.push(right);
            i += 1;
            j -= 1;
        } else {
            j -= 1;
        }

        if i > j {
            break;
        }
    }
    res
}

fn parse_input(input: &str) -> Vec<Block> {
    let mut digits = input
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    if digits.len() % 2 != 0 {
        digits.push(0);
    }

    digits
        .into_iter()
        .tuples()
        .enumerate()
        .flat_map(|(id, (blocks, spaces))| {
            let mut blocks: Vec<Block> = vec![Some(id); blocks as usize];
            let spaces = vec![None; spaces as usize];
            blocks.extend(spaces);
            blocks
        })
        .collect::<Vec<_>>()
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let parsed = parse_input(input);

        let checksum = to_checksum(&parsed);

        checksum
            .iter()
            .enumerate()
            .map(|(i, v)| i * v)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut parsed = parse_input(input)
            .into_iter()
            .chunk_by(|v| *v)
            .into_iter()
            .map(|(_, group)| group.collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut j = parsed.len() - 1;

        loop {
            let mut i = 0;
            // from the right, find a block of numbers ...
            while !parsed[j].iter().all(|v| v.is_some()) {
                j -= 1;
            }

            // ... but if we've reach the first block of numbers we're done
            if j == 1 {
                break;
            }

            // from the left, find the first block with enough remaining space
            while i < j && parsed[i].iter().filter(|v| **v == None).count() < parsed[j].len() {
                i += 1;
            }

            // there was no where to put this block, give up on it and reset
            if i >= j {
                j -= 1;
                continue;
            }

            // override the Nones with the values
            let idx = parsed[i].iter().position(|v| *v == None).unwrap();
            let amount = parsed[j].len();
            let value = parsed[j][0];

            for k in idx..(idx + amount) {
                parsed[i][k] = value;
            }

            for k in 0..parsed[j].len() {
                parsed[j][k] = None;
            }
        }

        let none_tail_idx = parsed
            .iter()
            .flatten()
            .rev()
            .position(|v| *v != None)
            .unwrap();

        parsed.truncate(none_tail_idx);

        parsed
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(i, v)| if let Some(v) = v { i * v } else { 0 })
            .sum::<usize>()
            .to_string()
    }
}
