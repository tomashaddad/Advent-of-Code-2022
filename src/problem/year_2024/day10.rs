use std::collections::HashSet;

use crate::problem::Day;

pub struct Code;

type Position = (usize, usize);

fn get_adjacents(pos: &Position, row_max: usize, col_max: usize) -> Vec<Position> {
    let (row, col) = (pos.0 as isize, pos.1 as isize);
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .filter_map(|(dr, dc)| {
            let (adj_row, adj_col) = (row + dr, col + dc);
            if adj_row >= 0
                && adj_col >= 0
                && adj_row < row_max as isize
                && adj_col < col_max as isize
            {
                Some((adj_row as usize, adj_col as usize))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn dfs(
    start: Position,
    graph: &Vec<Vec<u32>>,
    row_max: usize,
    col_max: usize,
    count_once: bool,
) -> usize {
    let mut score = 0;
    let mut explored = HashSet::<Position>::new();
    let mut stack = Vec::<Position>::from([start]);

    while let Some(current) = stack.pop() {
        if explored.contains(&current) {
            continue;
        }

        if count_once {
            explored.insert(current);
        }

        let (cur_row, cur_col) = current;

        if graph[cur_row][cur_col] == 9 {
            score += 1;
            continue;
        }

        for adjacent in get_adjacents(&current, row_max, col_max) {
            let (adj_row, adj_col) = adjacent;
            if graph[adj_row][adj_col] == graph[cur_row][cur_col] + 1 {
                stack.push(adjacent);
            }
        }
    }
    score
}

fn dfs_count_once(start: Position, graph: &Vec<Vec<u32>>, row_max: usize, col_max: usize) -> usize {
    dfs(start, graph, row_max, col_max, true)
}

fn dfs_count_many(start: Position, graph: &Vec<Vec<u32>>, row_max: usize, col_max: usize) -> usize {
    dfs(start, graph, row_max, col_max, false)
}

fn parse_input(input: &str) -> ((usize, usize), Vec<Vec<u32>>) {
    let row_max = input.lines().count();
    let col_max = input.lines().next().unwrap().len();
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|height| height.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    ((row_max, col_max), grid)
}

fn solve(input: &str, dfs_variant: fn(Position, &Vec<Vec<u32>>, usize, usize) -> usize) -> usize {
    let ((row_max, col_max), map) = parse_input(input);
    let mut score = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] != 0 {
                continue;
            }
            score += dfs_variant((row, col), &map, row_max, col_max);
        }
    }
    score
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        solve(input, dfs_count_once).to_string()
    }

    fn part2(&self, input: &str) -> String {
        solve(input, dfs_count_many).to_string()
    }
}
