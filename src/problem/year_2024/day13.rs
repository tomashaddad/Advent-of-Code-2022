use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};

use crate::problem::Day;

pub struct Code;

#[derive(Debug)]
struct Equation {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Prize {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Machine {
    a: Equation,
    b: Equation,
    p: Prize,
}

fn parse_prize(i: &str) -> IResult<&str, Prize> {
    map(
        tuple((
            tag("X="),
            nom::character::complete::i64,
            tag(", Y="),
            nom::character::complete::i64,
            line_ending,
        )),
        |(_, x, _, y, _)| Prize { x, y },
    )(i)
}

fn parse_equation(i: &str) -> IResult<&str, Equation> {
    map(
        tuple((
            tag("X+"),
            nom::character::complete::i64,
            tag(", Y+"),
            nom::character::complete::i64,
            line_ending,
        )),
        |(_, x, _, y, _)| Equation { x, y },
    )(i)
}

fn parse_machine(i: &str) -> IResult<&str, Machine> {
    let (i, a) = preceded(tag("Button A: "), parse_equation)(i)?;
    let (i, b) = preceded(tag("Button B: "), parse_equation)(i)?;
    let (i, p) = preceded(tag("Prize: "), parse_prize)(i)?;
    Ok((i, Machine { a, b, p }))
}

fn parse_all_machines(i: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(line_ending, parse_machine)(i)
}

// (1) a * x1 + b * x2 = px
// (2) a * y1 + b * y2 = py
// a = (px - b * x2 ) / x1                                  from (1)
// ((px - b * x2) / x1) * y1 + b * y2 = py                  sub a into (2)
// (y1 * px - y1 * b * x2) / x1 + (x1 * b * y2) / x1 = py   bring in y1, equate denominators
// (y1 * px) - (b * y1 * x2) + (x1 * b * y2) = x1 * py      multiply x1 over
// (x1 * b * y2) - (b * y1 * x2) = x1 * py - y1 * px        rearrange free terms
// b = (x1 * py - y1 * px) / (x1 * y2 - y1 * x2)            group b terms, rearrange

fn solve_linear(x1: i64, y1: i64, x2: i64, y2: i64, px: i64, py: i64) -> i64 {
    let b = (x1 * py - y1 * px) / (x1 * y2 - y1 * x2);
    let a = (px - b * x2) / x1;
    // a and b might have been floats turned into integers by division
    // which doesn't make sense because a and b represent integers (# of button presses)
    // check if solutions are correct
    if (a * x1 + b * x2, a * y1 + b * y2) != (px, py) {
        return 0;
    }
    3 * a + b
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let (_, machines) = all_consuming(parse_all_machines)(input).finish().unwrap();
        let mut total = 0;
        for m in machines {
            total += solve_linear(m.a.x, m.a.y, m.b.x, m.b.y, m.p.x, m.p.y);
        }
        total.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (_, machines) = all_consuming(parse_all_machines)(input).finish().unwrap();
        let mut total = 0;
        let added: i64 = 10_000_000_000_000;
        for m in machines {
            total += solve_linear(m.a.x, m.a.y, m.b.x, m.b.y, m.p.x + added, m.p.y + added);
        }
        total.to_string()
    }
}
