use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::{map, value},
    multi::{many0, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::problem::Day;

pub struct Code;

type Factor = i32;

#[derive(Debug, Clone)]
struct Product(Factor, Factor);

#[derive(Debug, Clone)]

enum Instruction {
    Mult(Product),
    Do,
    Dont,
}

fn parse_factors(i: &str) -> IResult<&str, Product> {
    map(
        separated_pair(
            nom::character::complete::i32,
            char(','),
            nom::character::complete::i32,
        ),
        |(a, b)| Product(a, b),
    )(i)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        many_till(
            map(anychar, drop),
            alt((
                map(delimited(tag("mul("), parse_factors, tag(")")), |p| {
                    Instruction::Mult(p)
                }),
                value(Instruction::Do, tag("do()")),
                value(Instruction::Dont, tag("don't()")),
            )),
        ),
        |(_, instruction)| instruction,
    )(i)
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let (_, v) = many0(parse_instruction)(input).unwrap();
        v.iter()
            .filter_map(|p| match p {
                Instruction::Mult(p) => Some(p.0 * p.1),
                _ => None,
            })
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (_, v) = many0(parse_instruction)(input).unwrap();
        let mut enabled = true;
        v.iter()
            .filter_map(|p| match p {
                Instruction::Mult(p) if enabled => Some(p.0 * p.1),
                Instruction::Do => {
                    enabled = true;
                    None
                }
                Instruction::Dont => {
                    enabled = false;
                    None
                }
                _ => None,
            })
            .sum::<i32>()
            .to_string()
    }
}
