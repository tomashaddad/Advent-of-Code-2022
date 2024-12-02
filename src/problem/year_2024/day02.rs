use crate::problem::Day;

pub struct Code;

fn parse_ints(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|char| char.parse().ok())
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    let ascending = report.windows(2).all(|pair| pair[0] < pair[1]);
    let descending = report.windows(2).all(|pair| pair[0] > pair[1]);
    let within_range = report
        .windows(2)
        .all(|pair| (1..=3).contains(&pair[0].abs_diff(pair[1])));
    (ascending || descending) && within_range
}

fn is_safe_dampened(report: &[i32]) -> bool {
    for level in 0..report.len() {
        let skipped = report
            .iter()
            .enumerate()
            .filter_map(|(i, &l)| (level != i).then_some(l))
            .collect::<Vec<_>>();
        if is_safe(&skipped) {
            return true;
        }
    }
    false
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| parse_ints(line))
            .filter(|report| is_safe(report))
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| parse_ints(line))
            .filter(|report| is_safe_dampened(report))
            .count()
            .to_string()
    }
}
