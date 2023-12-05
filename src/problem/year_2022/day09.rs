use std::collections::HashSet;

use crate::problem::Day;

struct Instruction {
    dir: Direction,
    steps: i32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Knot {
    pos: Position,
}

impl Knot {
    fn new() -> Self {
        Knot {
            pos: Position { x: 0, y: 0 },
        }
    }

    fn move_in_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.pos.y += 1,
            Direction::Down => self.pos.y -= 1,
            Direction::Left => self.pos.x -= 1,
            Direction::Right => self.pos.x += 1,
        }
    }

    fn follow(&mut self, other: &Knot) {
        fn to_unit(p: i32) -> i32 {
            match p {
                p if p < 0 => -1,
                0 => 0,
                _ => 1,
            }
        }
        let dx = to_unit(other.pos.x - self.pos.x);
        let dy = to_unit(other.pos.y - self.pos.y);

        self.pos.x += dx;
        self.pos.y += dy;
    }
}

struct Rope {
    knots: Vec<Knot>,
    tail_visited: HashSet<Position>,
}

impl Rope {
    fn new(knots: u32) -> Self {
        let mut rope = Rope {
            knots: Vec::new(),
            tail_visited: HashSet::new(),
        };
        for _ in 0..knots {
            rope.knots.push(Knot::new());
        }
        rope.tail_visited.insert(Position { x: 0, y: 0 });
        rope
    }

    fn move_head(&mut self, dir: &Direction) {
        self.knots.first_mut().unwrap().move_in_dir(dir);

        for index in 1..self.knots.len() {
            let previous = self.knots[index - 1];
            let current = self.knots.get_mut(index).unwrap();
            if (previous.pos.x - current.pos.x).abs() > 1
                || (previous.pos.y - current.pos.y).abs() > 1
            {
                current.follow(&previous);
            }
        }

        self.tail_visited.insert(self.knots.last().unwrap().pos);
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(dir, steps)| {
                    let dir = match dir {
                        "U" => Direction::Up,
                        "D" => Direction::Down,
                        "L" => Direction::Left,
                        "R" => Direction::Right,
                        _ => panic!("Invalid direction"),
                    };
                    let steps = steps.parse::<i32>().unwrap();
                    Instruction { dir, steps }
                })
                .expect(
                    "An instruction should be a whitespace separated pair of direction and steps",
                )
        })
        .collect::<Vec<_>>()
}

pub struct Code;

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let instructions = parse(input);

        let mut rope = Rope::new(2);

        instructions.iter().for_each(|instruction| {
            for _ in 0..instruction.steps {
                rope.move_head(&instruction.dir);
            }
        });

        rope.tail_visited.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let instructions = parse(input);

        let mut rope = Rope::new(10);

        instructions.iter().for_each(|instruction| {
            for _ in 0..instruction.steps {
                rope.move_head(&instruction.dir);
            }
        });

        rope.tail_visited.len().to_string()
    }
}
