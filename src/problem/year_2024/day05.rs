use std::{cmp::Ordering, collections::HashMap};

use crate::problem::Day;

pub struct Code;

struct ParsedInput {
    rules: HashMap<usize, Vec<usize>>,
    updates: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> ParsedInput {
    let rules = input
        .lines()
        .take_while(|l| !l.is_empty())
        .flat_map(|l| {
            l.split_once('|')
                .map(|v| (v.0.parse::<usize>().unwrap(), v.1.parse::<usize>().unwrap()))
        })
        .fold(HashMap::new(), |mut acc, (left, right)| {
            acc.entry(left)
                .and_modify(|e: &mut Vec<usize>| e.push(right))
                .or_insert(vec![right]);
            acc
        });

    let updates = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| {
            l.split(',')
                .filter_map(|v| v.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    ParsedInput { rules, updates }
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let ParsedInput { rules, updates } = parse_input(input);
        updates
            .iter()
            .filter(|update| {
                update.is_sorted_by(|a, b| rules.get(a).is_some_and(|k| k.contains(b)))
            })
            .map(|update| update[update.len() / 2])
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let ParsedInput { rules, mut updates } = parse_input(input);
        updates
            .iter_mut()
            .filter(|update| {
                !update.is_sorted_by(|a, b| rules.get(a).is_some_and(|k| k.contains(b)))
            })
            .map(|update| {
                update.sort_by(|a, b| {
                    if rules.get(a).is_some_and(|k| k.contains(b)) {
                        Ordering::Less
                    } else if rules.get(b).is_some_and(|k| k.contains(a)) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                update
            })
            .map(|update| update[update.len() / 2])
            .sum::<usize>()
            .to_string()
    }
}
