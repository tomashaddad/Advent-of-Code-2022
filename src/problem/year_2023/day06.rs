use crate::problem::Day;

pub struct Code;

fn time_to_dist(race_time: u64, hold_time: u64) -> u64 {
    race_time * hold_time - hold_time.pow(2)
}

fn total_better_times(max_time: u64, current_record: u64) -> u64 {
    (0..=max_time)
        .filter(|&candidate_time| time_to_dist(max_time, candidate_time) > current_record)
        .count() as u64
}

fn parse_vec_seperate(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .collect::<Vec<_>>()
}

fn parse_vec_combined(input: &str) -> u64 {
    input
        .split_once(':')
        .expect("Should be a colon")
        .1
        .replace(' ', "")
        .parse::<u64>()
        .expect("Should be a u64")
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let mut iter = input.lines();
        let race_times = parse_vec_seperate(iter.next().expect("First line should be times"));
        let record_distances =
            parse_vec_seperate(iter.next().expect("Second line should be distances"));

        race_times
            .into_iter()
            .zip(record_distances)
            .map(|(max_time, current_record)| total_better_times(max_time, current_record))
            .collect::<Vec<_>>()
            .into_iter()
            .product::<u64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut iter = input.lines();
        let race_time = parse_vec_combined(iter.next().expect("First line should be times"));
        let record_distance =
            parse_vec_combined(iter.next().expect("Second line should be distances"));

        total_better_times(race_time, record_distance).to_string()
    }
}
