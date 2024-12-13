use std::collections::{HashMap, HashSet, VecDeque};

use crate::problem::Day;

pub struct Code;

type Position = (usize, usize);

#[derive(Debug, Eq, Hash, PartialEq)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    fn dr_dc(&self) -> (isize, isize) {
        match self {
            Direction::NORTH => (-1, 0),
            Direction::SOUTH => (1, 0),
            Direction::EAST => (0, 1),
            Direction::WEST => (0, -1),
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::NORTH,
    Direction::SOUTH,
    Direction::EAST,
    Direction::WEST,
];

fn get_adjacents(
    pos: &(usize, usize),
    garden: &Vec<Vec<char>>,
) -> Vec<(Direction, Option<Position>)> {
    DIRECTIONS
        .into_iter()
        .map(|direction| {
            let (dr, dc) = direction.dr_dc();
            let row = pos.0 as isize + dr;
            let col = pos.1 as isize + dc;
            let adjacent = if row >= 0
                && col >= 0
                && (row as usize) < garden.len()
                && (col as usize) < garden[0].len()
            {
                Some((row as usize, col as usize))
            } else {
                None
            };
            (direction, adjacent)
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct Plot {
    _char: char,
    positions: HashSet<Position>,
    fences: HashMap<Position, HashSet<Direction>>,
}

fn flood_fill(
    start_pos: Position,
    garden: &Vec<Vec<char>>,
    visited: &mut HashSet<Position>,
) -> Plot {
    let (row, col) = start_pos;
    let char = garden[row][col];
    let mut queue = VecDeque::from([start_pos]);
    let mut positions = HashSet::from([start_pos]);
    let mut fences = HashMap::<Position, HashSet<Direction>>::new();

    while let Some(position) = queue.pop_front() {
        if visited.contains(&position) {
            continue;
        }

        visited.insert(position);

        for (direction, adjacent) in get_adjacents(&position, garden) {
            if let Some(adjacent) = adjacent {
                let (adj_row, adj_col) = adjacent;
                if garden[adj_row][adj_col] == char {
                    positions.insert(adjacent);
                    queue.push_back(adjacent);
                } else {
                    fences.entry(position).or_default().insert(direction);
                }
            } else {
                fences.entry(position).or_default().insert(direction);
            }
        }
    }
    Plot {
        _char: char,
        positions,
        fences,
    }
}

fn create_garden(parsed_input: Vec<Vec<char>>) -> Vec<Plot> {
    let mut visited = HashSet::<Position>::new();
    let mut garden = Vec::new();
    for row in 0..parsed_input.len() {
        for col in 0..parsed_input[row].len() {
            if visited.contains(&(row, col)) {
                continue;
            }
            let plot = flood_fill((row, col), &parsed_input, &mut visited);
            garden.push(plot);
        }
    }
    garden
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let parsed = parse(input);
        let garden = create_garden(parsed);
        garden
            .iter()
            .fold(0, |acc, plot| {
                acc + (plot.positions.len()
                    * plot
                        .fences
                        .values()
                        .map(|fences| fences.len())
                        .sum::<usize>())
            })
            .to_string()
    }

    fn part2(&self, _input: &str) -> String {
        todo!();
    }
}
