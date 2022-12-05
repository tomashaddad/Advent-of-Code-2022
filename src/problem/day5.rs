use std::collections::VecDeque;

use crate::Day;

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

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let state = transpose(
            input
                .lines()
                .take_while(|line| !line.is_empty())
                .map(|line| line.chars().collect())
                .collect(),
        );

        let mut state = state
            .into_iter()
            .map(|row| VecDeque::from(row))
            .filter(|row| row.back().unwrap().is_numeric())
            .map(|mut row| {
                row.pop_back();
                row.retain(|c| !c.is_whitespace());
                VecDeque::from(row)
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

        instructions.iter().for_each(|i| {
            for _ in 0..i.amount {
                let value = state[i.from - 1].pop_front().unwrap();
                state[i.to - 1].push_front(value);
            }
        });

        state.iter().map(|s| s[0]).collect()
    }

    fn part2(&self, input: &str) -> String {
        todo!();
    }
}
