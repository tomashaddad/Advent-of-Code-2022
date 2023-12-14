use crate::problem::Day;

pub struct Code;
impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let grid = input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut total = 0;
        let max_points = grid.len();
        for (y, _) in grid[0].iter().enumerate() {
            let mut column_total = 0;
            let mut bonus = 0;
            for (x, _) in grid.iter().enumerate() {
                let value = max_points - x;
                match grid[x][y] {
                    Tile::Rock => {
                        column_total += value + bonus;
                    }
                    Tile::Cube => {
                        bonus = 0;
                    }
                    Tile::Empty => {
                        bonus += 1;
                    }
                }
            }
            total += column_total;
        }

        total.to_string()
    }

    fn part2(&self, _input: &str) -> String {
        todo!();
    }
}

enum Tile {
    Rock,
    Cube,
    Empty,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'O' => Tile::Rock,
            '#' => Tile::Cube,
            '.' => Tile::Empty,
            _ => panic!("Invalid tile"),
        }
    }
}
