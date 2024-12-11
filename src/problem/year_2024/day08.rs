use std::collections::HashMap;

use itertools::Itertools;
use num::ToPrimitive;

use crate::problem::Day;

pub struct Code;

fn parse_input(input: &str) -> HashMap<char, Vec<(usize, usize)>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| match c {
                    '.' => None,
                    _ => Some((c, (row, col))),
                })
        })
        .fold(HashMap::new(), |mut acc, (c, (row, col))| {
            let p = (row, col);
            acc.entry(c).and_modify(|v| v.push(p)).or_insert(vec![p]);
            acc
        })
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let row_max = input.lines().count();
        let col_max = input.lines().next().unwrap().len();

        parse_input(input)
            .values()
            .flat_map(|a| a.iter().tuple_combinations::<(_, _)>())
            .flat_map(|(a1, a2)| {
                let (a1i, a2i) = (
                    (a1.0 as isize, a1.1 as isize),
                    (a2.0 as isize, a2.1 as isize),
                );
                let (dr, dc) = (a2i.0 - a1i.0, a2i.1 - a1i.1);
                [(a1i.0 - dr, a1i.1 - dc), (a2i.0 + dr, a2i.1 + dc)]
                    .into_iter()
                    .filter(|ant| {
                        let (row, col) = ant;
                        if let (Some(row), Some(col)) = (row.to_usize(), col.to_usize()) {
                            return row < row_max && col < col_max;
                        }
                        false
                    })
            })
            .unique()
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let row_max = input.lines().count();
        let col_max = input.lines().next().unwrap().len();

        parse_input(input)
            .values()
            .flat_map(|a| a.iter().tuple_combinations::<(_, _)>())
            .flat_map(|(a1, a2)| {
                let (a1i, a2i) = (
                    (a1.0 as isize, a1.1 as isize),
                    (a2.0 as isize, a2.1 as isize),
                );
                (0..)
                    .map(move |s| {
                        let (dr, dc) = (s * (a2i.0 - a1i.0), s * (a2i.1 - a1i.1));
                        [(a1i.0 - dr, a1i.1 - dc), (a2i.0 + dr, a2i.1 + dc)]
                            .into_iter()
                            .filter(|ant| {
                                let (row, col) = ant;
                                if let (Some(row), Some(col)) = (row.to_usize(), col.to_usize()) {
                                    return row < row_max && col < col_max;
                                }
                                false
                            })
                            .collect::<Vec<_>>()
                    })
                    .take_while(|res| !res.is_empty())
            })
            .flatten()
            .unique()
            .count()
            .to_string()
    }
}
