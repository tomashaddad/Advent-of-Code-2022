use crate::problem::Day;

pub struct Code;

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        input
            .lines()
            .map(Pyramid::from_str)
            .collect::<Vec<_>>()
            .iter()
            .map(|pyramid| {
                pyramid.levels.first().unwrap().last().unwrap()
                    + pyramid
                        .levels
                        .windows(2)
                        .rev()
                        .fold(0, |acc, pair| acc + pair[1].last().unwrap())
            })
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .lines()
            .map(Pyramid::from_str)
            .collect::<Vec<_>>()
            .iter()
            .map(|pyramid| {
                pyramid.levels.first().unwrap().first().unwrap()
                    - pyramid
                        .levels
                        .windows(2)
                        .rev()
                        .fold(0, |acc, pair| pair[1].first().unwrap() - acc)
            })
            .sum::<i64>()
            .to_string()
    }
}

struct Pyramid {
    levels: Vec<Vec<i64>>,
}

impl Pyramid {
    fn from_str(str: &str) -> Self {
        let mut pyramid: Vec<Vec<i64>> = vec![str
            .split_whitespace()
            .map(|num| {
                num.parse::<i64>()
                    .expect("Input should only contain numbers")
            })
            .collect::<Vec<_>>()];

        while pyramid.last().unwrap().iter().any(|&num| num != 0) {
            let next_level = pyramid
                .last()
                .unwrap()
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<_>>();
            pyramid.push(next_level);
        }

        Pyramid { levels: pyramid }
    }
}
