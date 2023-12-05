use crate::problem::Day;

pub struct Code;

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let mut max = 0;
        let mut current = 0;

        for line in input.lines() {
            if line.is_empty() {
                if current > max {
                    max = current;
                }
                current = 0;
                continue;
            }
            let calorie = line.parse::<i32>();
            current += calorie.unwrap();
        }

        max.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut top_three = [0, 0, 0];
        let mut current = 0;

        for line in input.lines() {
            if line.is_empty() {
                let min = top_three.iter_mut().min().unwrap();
                if current > *min {
                    *min = current;
                }
                current = 0;
                continue;
            }
            let calorie = line.parse::<i32>();
            current += calorie.unwrap();
        }

        let sum: i32 = top_three.iter().sum();

        sum.to_string()
    }
}
