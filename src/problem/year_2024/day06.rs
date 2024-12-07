use crate::problem::Day;
use std::collections::HashSet;

pub struct Code;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position(i32, i32);

impl Position {
    fn move_in_direction(&self, d: Direction) -> Position {
        let delta = match d {
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
            Direction::RIGHT => (0, 1),
        };
        Position(self.0 + delta.0, self.1 + delta.1)
    }
}

#[derive(Debug, Copy, Clone)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn rotate_right(&mut self) {
        self.direction = self.direction.rotate_right();
    }

    fn next(&self) -> Position {
        self.position.move_in_direction(self.direction)
    }
}

#[derive(PartialEq)]
enum CellType {
    PATH,
    WALL,
    EXIT,
}

struct Maze {
    maze: Vec<Vec<char>>,
}

impl From<&str> for Maze {
    fn from(input: &str) -> Self {
        let maze = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { maze }
    }
}

impl Maze {
    fn get(&self, p: &Position) -> Option<&char> {
        if p.0 >= 0 && p.1 >= 0 {
            self.maze.get(p.0 as usize)?.get(p.1 as usize)
        } else {
            None
        }
    }

    fn find_guard(&self) -> Position {
        for (ri, _) in self.maze.iter().enumerate() {
            if let Some((ci, _)) = self.maze[ri].iter().enumerate().find(|(_, &v)| v == '^') {
                return Position(ri as i32, ci as i32);
            }
        }
        unreachable!();
    }

    fn cell_type_at(&self, p: &Position) -> CellType {
        if let Some(cell) = self.get(p) {
            match cell {
                '.' | '^' => CellType::PATH,
                '#' => CellType::WALL,
                _ => unreachable!(),
            }
        } else {
            CellType::EXIT
        }
    }
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let maze = Maze::from(input);
        let mut guard = Guard {
            position: maze.find_guard(),
            direction: Direction::UP,
        };

        let mut visited: HashSet<Position> = HashSet::from([guard.position]);

        loop {
            let next = guard.next();
            match maze.cell_type_at(&next) {
                CellType::PATH => {
                    guard.position = next;
                    visited.insert(next);
                }
                CellType::WALL => {
                    guard.rotate_right();
                }
                CellType::EXIT => {
                    break;
                }
            }
        }
        visited.len().to_string()
    }

    fn part2(&self, _input: &str) -> String {
        todo!("Brute force? :(");
    }
}
