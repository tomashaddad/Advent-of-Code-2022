use itertools::Itertools;

use crate::problem::Day;

pub struct Code;

const ROWS: usize = 103;
const COLS: usize = 101;

#[derive(Debug)]
struct Position {
    r: usize,
    c: usize,
}

#[derive(Debug)]
struct Velocity {
    r: i64,
    c: i64,
}

#[derive(Debug)]
struct Robot {
    p: Position,
    v: Velocity,
}

impl Robot {
    fn new(row: usize, col: usize, vrow: i64, vcol: i64) -> Self {
        Self {
            p: Position { r: row, c: col },
            v: Velocity { r: vrow, c: vcol },
        }
    }
}

fn print_robots(lobby: &[[u32; 101]; 103]) {
    for row in lobby {
        println!(
            "{:?}",
            row.iter()
                .flat_map(|v| {
                    if *v > 0 {
                        char::from_digit(*v, 10)
                    } else {
                        Some('.')
                    }
                })
                .collect::<String>()
        );
    }
}

fn lined_up(robots: &[[u32; 101]; 103], num: u8) -> bool {
    let mut count = 0;

    for r in 0..ROWS {
        for c in 0..COLS {
            if robots[r][c] > 0 {
                count += 1;
            } else {
                count = 0;
            }

            if count >= num {
                return true;
            }
        }
        count = 0;
    }
    false
}

fn safety_factor(robots: &Vec<Robot>) -> usize {
    let mut quadrants = [0; 4]; // top left, top right, bottom left, bottom right
    for robot in robots {
        if robot.p.r == ROWS / 2 || robot.p.c == COLS / 2 {
            continue;
        }

        let row = (robot.p.r < ROWS / 2) as usize;
        let col = (robot.p.c < COLS / 2) as usize;
        quadrants[2 * row + col] += 1;
    }
    quadrants.iter().product()
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let mut robots = input
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter(|s| !s.is_empty())
            .flat_map(|s| s.parse::<i64>())
            .tuples()
            .map(|(c, r, vc, vr)| Robot::new(r as usize, c as usize, vr, vc))
            .collect::<Vec<_>>();

        for _ in 0..100 {
            for r in &mut robots {
                r.p.r = (r.p.r as i64 + r.v.r).rem_euclid(ROWS as i64) as usize;
                r.p.c = (r.p.c as i64 + r.v.c).rem_euclid(COLS as i64) as usize;
            }
        }

        safety_factor(&robots).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut robots = input
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter(|s| !s.is_empty())
            .flat_map(|s| s.parse::<i64>())
            .tuples()
            .map(|(c, r, vc, vr)| Robot::new(r as usize, c as usize, vr, vc))
            .collect::<Vec<_>>();

        let mut lobby = [[0u32; COLS]; ROWS];

        for robot in &robots {
            lobby[robot.p.r][robot.p.c] += 1;
        }

        for i in 0..10000 {
            for r in &mut robots {
                lobby[r.p.r][r.p.c] -= 1;
                r.p.r = (r.p.r as i64 + r.v.r).rem_euclid(ROWS as i64) as usize;
                r.p.c = (r.p.c as i64 + r.v.c).rem_euclid(COLS as i64) as usize;
                lobby[r.p.r][r.p.c] += 1;
            }

            if lined_up(&lobby, 8) {
                print_robots(&lobby);
                return i.to_string();
            }
        }
        unreachable!();
    }
}
