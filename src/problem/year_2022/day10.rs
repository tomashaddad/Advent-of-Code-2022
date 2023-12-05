use crate::problem::Day;

pub struct Code;

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let mut cycle = 0;
        let mut value = 1;
        let mut strengths: Vec<i32> = Vec::new();
        for line in input.lines() {
            let mut dv = 0;
            let cycles = match line.split_once(' ') {
                Some((_, v)) => {
                    dv = v.parse::<i32>().unwrap();
                    2
                }
                None => 1,
            };

            for _ in 0..cycles {
                cycle += 1;
                match cycle {
                    20 | 60 | 100 | 140 | 180 | 220 => {
                        strengths.push(value * cycle);
                    }
                    _ => {}
                };
            }
            value += dv;
        }
        strengths.iter().sum::<i32>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut crt = vec![vec!['.'; 40]; 6];

        let mut position: i32 = 1;
        let mut pixel: i32 = 0;
        let mut row = 0;

        for line in input.lines() {
            let mut dv: i32 = 0;
            let cycles = match line.split_once(' ') {
                Some((_, v)) => {
                    dv = v.parse().unwrap();
                    2
                }
                None => 1,
            };

            for _ in 0..cycles {
                if pixel >= (position - 1) && pixel <= (position + 1) {
                    crt[row][pixel as usize] = '#';
                }
                pixel = (pixel + 1) % 40;
                if pixel == 0 {
                    row += 1;
                }
            }
            position += dv;
        }

        for row in crt {
            for pixel in row {
                print!("{}", pixel);
            }
            println!();
        }

        "Done!".to_string()
    }
}
