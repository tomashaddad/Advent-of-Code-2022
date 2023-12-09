use std::collections::HashMap;

use crate::problem::Day;

pub struct Code;

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let (instructions, network) = input.split_once("\r\n\r\n").unwrap();
        let network_map = create_network_map(network);

        total_steps_until(test::all_z, "AAA", instructions, &network_map).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (instructions, network) = input.split_once("\r\n\r\n").unwrap();
        let network_map = create_network_map(network);

        let starting_nodes = network_map
            .keys()
            .filter(|node| node.ends_with('A'))
            .cloned()
            .collect::<Vec<_>>();

        starting_nodes
            .iter()
            .map(|node| total_steps_until(test::ends_with_z, node, instructions, &network_map))
            .fold(1, num::integer::lcm)
            .to_string()
    }
}

mod test {
    pub fn all_z(node: &str) -> bool {
        node == "ZZZ"
    }

    pub fn ends_with_z(node: &str) -> bool {
        node.ends_with('Z')
    }
}

fn total_steps_until(
    test_node: fn(&str) -> bool,
    node: &str,
    instructions: &str,
    network_map: &HashMap<String, (String, String)>,
) -> u64 {
    let mut instructions = instructions.chars().cycle();
    let mut steps: u64 = 0;
    let mut current = node.to_string();
    while !test_node(&current) {
        match instructions.next().unwrap() {
            'L' => current = network_map[&current].0.clone(),
            'R' => current = network_map[&current].1.clone(),
            _ => panic!("Invalid instruction"),
        }
        steps += 1;
    }
    steps
}

fn create_network_map(network: &str) -> HashMap<String, (String, String)> {
    network
        .lines()
        .map(|line| {
            let filtered = line
                .chars()
                .filter(|c| ![' ', '(', ')'].contains(c))
                .collect::<String>();
            let (from, to_either) = filtered.split_once('=').unwrap();
            let (left, right) = to_either.split_once(',').unwrap();
            (from.to_owned(), (left.to_owned(), right.to_owned()))
        })
        .collect::<HashMap<_, _>>()
}
