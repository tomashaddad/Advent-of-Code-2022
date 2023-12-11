use crate::problem::Day;

pub struct Code;

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let pipe_grid = PipeGrid::from_str(input);
        let pipe_loop = pipe_grid.get_pipe_loop();
        pipe_loop.len().to_string() // need to divide by 2 for answer
    }

    fn part2(&self, input: &str) -> String {
        let pipe_grid = PipeGrid::from_str(input);
        let pipe_loop = pipe_grid.get_pipe_loop();
        pipe_grid.get_enclosed_tile_sum(&pipe_loop).to_string()
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Node {
    position: Position,
    tile_type: char,
}

struct PipeGrid {
    nodes: Vec<Vec<Node>>,
    start_node: Node,
}

impl PipeGrid {
    fn from_str(str: &str) -> Self {
        let mut nodes: Vec<Vec<Node>> = Vec::new();
        for (y, line) in str.lines().enumerate() {
            nodes.push(Vec::new());
            for (x, c) in line.chars().enumerate() {
                nodes[y].push(Node {
                    position: Position { x, y },
                    tile_type: c,
                });
            }
        }

        let start_node = *nodes
            .iter()
            .flatten()
            .find(|node| node.tile_type == 'S')
            .expect("At least one start position must exist in the grid");

        PipeGrid { nodes, start_node }
    }

    fn get_pipe_loop(&self) -> Vec<&Node> {
        let mut pipe_loop = vec![];
        let mut curr_node = &self.start_node;
        let mut prev_node = Option::<&Node>::None;

        loop {
            for direction in &[
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                if !can_travel_in_dir(curr_node.tile_type, direction) {
                    continue;
                }

                if let Some(next_node) = self.find_next(direction, curr_node) {
                    if let Some(prev_pos) = prev_node {
                        if next_node.position == prev_pos.position {
                            continue;
                        }
                    }
                    pipe_loop.push(next_node);
                    prev_node = Some(curr_node);
                    curr_node = next_node;
                    break;
                }
            }
            if curr_node.position == self.start_node.position {
                break;
            }
        }
        pipe_loop
    }

    fn get_enclosed_tile_sum(&self, pipe_loop: &[&Node]) -> u32 {
        let mut wiped_grid = self.get_wiped_grid(pipe_loop);

        for candidate_type in ['|', '-', 'J', 'L', 'F', '7'] {
            let mut count = 0;
            for direction in [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                if can_travel_in_dir(candidate_type, &direction)
                    && self.find_next(&direction, &self.start_node).is_some()
                {
                    count += 1;
                }
            }
            if count == 2 {
                let Position { x, y } = self.start_node.position;
                wiped_grid[y][x].tile_type = candidate_type;
                break;
            }
        }

        let mut count = 0;
        let mut prev_char = '.'; // dummy value
        for row in &wiped_grid {
            let mut inside = false;
            for node in row {
                // hack because pipe_loop start is still 'S' while wiped_grid is replaced
                if pipe_loop.contains(&node) || node.position == self.start_node.position {
                    match node.tile_type {
                        '-' => continue,
                        '|' => inside = !inside,
                        _ => {
                            if prev_char == 'L' && node.tile_type == '7'
                                || prev_char == 'F' && node.tile_type == 'J'
                            {
                                inside = !inside;
                            }
                        }
                    }
                    prev_char = node.tile_type;
                } else if inside {
                    count += 1;
                }
            }
        }
        count
    }

    // wipes all nodes not in the pipe loop
    fn get_wiped_grid(&self, pipe_loop: &[&Node]) -> Vec<Vec<Node>> {
        self.nodes
            .iter()
            .map(|row| {
                row.iter()
                    .map(|node| {
                        if pipe_loop.contains(&node) {
                            *node
                        } else {
                            Node {
                                position: node.position,
                                tile_type: '.',
                            }
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn find_next(&self, direction: &Direction, curr_node: &Node) -> Option<&Node> {
        let Position { x, y } = curr_node.position;
        match direction {
            Direction::North => {
                if y > 0 && matches!(self.nodes[y - 1][x].tile_type, 'S' | '|' | 'F' | '7') {
                    Some(&self.nodes[y - 1][x])
                } else {
                    None
                }
            }
            Direction::South => {
                if y < self.nodes.len() - 1
                    && matches!(self.nodes[y + 1][x].tile_type, 'S' | '|' | 'L' | 'J')
                {
                    Some(&self.nodes[y + 1][x])
                } else {
                    None
                }
            }
            Direction::East => {
                if x < self.nodes[0].len() - 1
                    && matches!(self.nodes[y][x + 1].tile_type, 'S' | '-' | 'J' | '7')
                {
                    Some(&self.nodes[y][x + 1])
                } else {
                    None
                }
            }
            Direction::West => {
                if x > 0 && matches!(self.nodes[y][x - 1].tile_type, 'S' | '-' | 'L' | 'F') {
                    Some(&self.nodes[y][x - 1])
                } else {
                    None
                }
            }
        }
    }
}

fn can_travel_in_dir(current_type: char, direction: &Direction) -> bool {
    match current_type {
        'S' => true,
        '|' => matches!(direction, Direction::North | Direction::South),
        '-' => matches!(direction, Direction::East | Direction::West),
        'J' => matches!(direction, Direction::North | Direction::West),
        'L' => matches!(direction, Direction::North | Direction::East),
        'F' => matches!(direction, Direction::South | Direction::East),
        '7' => matches!(direction, Direction::South | Direction::West),
        _ => unreachable!(),
    }
}
