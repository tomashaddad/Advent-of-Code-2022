use std::collections::HashSet;

use crate::Day;

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    steps: i32,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct RopeHead {
    x: i32,
    y: i32,
    previous: (i32, i32),
}

struct RopeTail {
    x: i32,
    y: i32,
    history: Vec<(i32, i32)>,
}

struct Rope {
    head: RopeHead,
    tail: RopeTail,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: RopeHead {
                x: 0,
                y: 0,
                previous: (0, 0),
            },
            tail: RopeTail {
                x: 0,
                y: 0,
                history: Vec::from([(0, 0)]),
            },
        }
    }

    fn move_head(&mut self, dir: &Direction) {
        self.head.previous = (self.head.x, self.head.y);
        match dir {
            Direction::Up => self.head.y += 1,
            Direction::Down => self.head.y -= 1,
            Direction::Left => self.head.x -= 1,
            Direction::Right => self.head.x += 1,
        }

        if (self.head.x - self.tail.x).abs() > 1 || (self.head.y - self.tail.y).abs() > 1 {
            let (x, y) = self.head.previous;
            self.tail.x = x;
            self.tail.y = y;
            self.tail.history.push((x, y));
        }
    }
}

pub struct Code;

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let instructions = input
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
                    .expect("An instruction should be a whitespace separated pair of direction and steps")
            })
            .collect::<Vec<_>>();

        println!("{:?}", instructions.len());

        // for instruction in &instructions {
        //     println!("{:?}", instruction);
        // }

        let mut rope = Rope::new();

        instructions.iter().for_each(|instruction| {
            for _ in 0..instruction.steps {
                rope.move_head(&instruction.dir);
            }
        });

        rope.tail
            .history
            .iter()
            .collect::<HashSet<_>>()
            .len()
            .to_string()
    }

    fn part2(&self, _input: &str) -> String {
        todo!();
    }
}
