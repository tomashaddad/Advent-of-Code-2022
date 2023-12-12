use crate::problem::Day;

#[derive(Debug)]
struct Mapping {
    destination_start: u64,
    source_start: u64,
    range: u64,
}

impl Mapping {
    fn new(destination_start: u64, source_start: u64, range: u64) -> Mapping {
        Mapping {
            destination_start,
            source_start,
            range,
        }
    }
}

pub struct Code;
impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let mut blocks = input.split("\r\n\r\n");

        let seeds = blocks
            .next()
            .expect("File should be non-empty")
            .split_once(':')
            .expect("First line should be seeds: <numbers>")
            .1
            .split_whitespace()
            .map(|s| s.parse::<u64>().expect("Seeds should be numbers"))
            .collect::<Vec<_>>();

        let maps = blocks
            .map(|block| {
                block
                    .lines()
                    .skip(1)
                    .map(|line| {
                        line.split_whitespace()
                            .map_while(|value| value.parse::<u64>().ok())
                    })
                    .map(|mut values| {
                        let destination = values.next().expect("Destination should be 1st value");
                        let source = values.next().expect("Source should be 2nd value");
                        let range = values.next().expect("Range should be 3rd value");
                        Mapping::new(destination, source, range)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // Note to self:
        // You failed at this until you cheated and looked at Ando's code,realising that you were supposed
        // to reuse the seed variable name as the accumulator, otherwise unwrap_or(seed) would always
        // use the original value. Idiot!
        seeds
            .iter()
            .map(|seed| {
                maps.iter().fold(*seed, |seed, maps| {
                    maps.iter()
                        .find_map(|map| {
                            if seed >= map.source_start && seed < map.source_start + map.range {
                                Some(seed - map.source_start + map.destination_start)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(seed)
                })
            })
            .min()
            .unwrap()
            .to_string()
    }

    fn part2(&self, _input: &str) -> String {
        todo!();
    }
}
