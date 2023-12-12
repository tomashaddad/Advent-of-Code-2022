use crate::problem::Day;

pub struct Code;

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        do_it(input, 2).to_string()
    }

    fn part2(&self, input: &str) -> String {
        do_it(input, 1000000).to_string()
    }
}

fn do_it(input: &str, multiplier: usize) -> usize {
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, node_type)| Node { node_type, x, y })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let galaxies = grid
        .iter()
        .flatten()
        .filter(|n| n.node_type == '#')
        .collect::<Vec<_>>();

    let empty_row_idxs = grid
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            if row.iter().all(|t| t.node_type == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let empty_col_idxs = (0..grid[0].len())
        .filter(|&col_index| grid.iter().all(|row| row[col_index].node_type == '.'))
        .collect::<Vec<_>>();

    let mut distance = 0;
    let mut empty_rows_between = 0;
    let mut empty_cols_between = 0;
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(i) {
            distance += g1.x.abs_diff(g2.x) + g1.y.abs_diff(g2.y);
            empty_rows_between += empty_row_idxs
                .iter()
                .filter(|&&row_index| row_index > g1.y.min(g2.y) && row_index < g1.y.max(g2.y))
                .count();
            empty_cols_between += empty_col_idxs
                .iter()
                .filter(|&&col_index| col_index > g1.x.min(g2.x) && col_index < g1.x.max(g2.x))
                .count();
        }
    }

    distance + (multiplier - 1) * empty_cols_between + (multiplier - 1) * empty_rows_between
}

struct Node {
    node_type: char,
    x: usize,
    y: usize,
}
