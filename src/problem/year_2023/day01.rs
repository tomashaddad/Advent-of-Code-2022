use crate::problem::Day;

fn to_calibration_value(line: &str) -> u32 {
    println!("{}", line);
    let a = line
        .chars()
        .find(|c| c.is_ascii_digit())
        .expect("First digit found as char")
        .to_digit(10)
        .expect("First char converted into u32");
    let b = line
        .chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .expect("Second digit found as char")
        .to_digit(10)
        .expect("Second char converted into u32");
    10 * a + b
}

pub struct Code;
impl Day for Code {
    fn part1(&self, input: &str) -> String {
        input
            .lines()
            .map(to_calibration_value)
            .sum::<u32>() // why can't it tell they're u32 already? :(
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                to_calibration_value(
                    line.replace("one", "o1e")
                        .replace("two", "t2o")
                        .replace("three", "t3e")
                        .replace("four", "f4r")
                        .replace("five", "f5e")
                        .replace("six", "s6x")
                        .replace("seven", "s7n")
                        .replace("eight", "e8t")
                        .replace("nine", "n9e")
                        .as_str(),
                )
            })
            .sum::<u32>()
            .to_string()
    }
}
