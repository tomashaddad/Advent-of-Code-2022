use std::collections::VecDeque;

use crate::problem::Day;

pub struct Code;

// Straight up copied this from below SO question no shame whatsoever
// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> (Vec<VecDeque<char>>, Vec<Instruction>) {
    let state = transpose(
        input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect(),
    );

    let state = state
        .into_iter()
        .map(VecDeque::from)
        .filter(|row| row.back().unwrap().is_numeric())
        .map(|mut row| {
            row.pop_back();
            row.retain(|c| !c.is_whitespace());
            row
        })
        .collect::<Vec<_>>();

    let instructions = input
        .lines()
        .skip_while(|line| !line.starts_with("move"))
        .map(|line| {
            let mut parts = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok());

            Instruction {
                amount: parts.next().unwrap(),
                from: parts.next().unwrap(),
                to: parts.next().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    (state, instructions)
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let (mut state, instructions) = parse(input);
        instructions.iter().for_each(|i| {
            for _ in 0..i.amount {
                let value = state[i.from - 1].pop_front().unwrap();
                state[i.to - 1].push_front(value);
            }
        });

        state.iter().map(|s| s[0]).collect()
    }

    fn part2(&self, input: &str) -> String {
        let (mut state, instructions) = parse(input);
        instructions.iter().for_each(|i| {
            let mut values: VecDeque<char> = VecDeque::new();
            for _ in 0..i.amount {
                values.push_front(state[i.from - 1].pop_front().unwrap());
            }

            for value in values {
                state[i.to - 1].push_front(value);
            }
        });

        state.iter().map(|s| s[0]).collect()
    }
}
