use crate::problem::Day;

pub struct Code;

#[derive(Debug)]
struct Equation {
    target: usize,
    operands: Vec<usize>,
}

fn add(left: Option<usize>, right: usize) -> usize {
    left.unwrap_or(0) + right
}

fn multiply(left: Option<usize>, right: usize) -> usize {
    left.unwrap_or(1) * right
}

fn concatenate(left: Option<usize>, right: usize) -> usize {
    if let Some(left) = left {
        format!("{}{}", left, right).parse().unwrap()
    } else {
        right
    }
}

fn do_the_thing(
    target: usize,
    values: &[usize],
    current: Option<usize>,
    operations: &[fn(Option<usize>, usize) -> usize],
) -> bool {
    if values.is_empty() {
        return current.unwrap() == target;
    }

    operations.iter().any(|op| {
        do_the_thing(
            target,
            &values[1..],
            Some(op(current, values[0])),
            operations,
        )
    })
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .flat_map(|l| {
            l.split_once(':').map(|(target, numbers)| Equation {
                target: target.parse().unwrap(),
                operands: numbers
                    .trim()
                    .split(" ")
                    .map(|v| v.parse().unwrap())
                    .collect::<Vec<_>>(),
            })
        })
        .collect::<Vec<_>>()
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let parsed = parse_input(input);

        parsed
            .iter()
            .filter(|eq| do_the_thing(eq.target, &eq.operands, None, &[add, multiply]))
            .map(|valid| valid.target)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let parsed = parse_input(input);

        parsed
            .iter()
            .filter(|eq| do_the_thing(eq.target, &eq.operands, None, &[add, multiply, concatenate]))
            .map(|valid| valid.target)
            .sum::<usize>()
            .to_string()
    }
}
