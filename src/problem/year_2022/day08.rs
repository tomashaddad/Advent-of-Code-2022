use crate::problem::Day;

pub struct Code;

#[derive(Clone, Copy)]
struct Visibility {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl Visibility {
    fn new() -> Self {
        Self {
            top: true,
            bottom: true,
            left: true,
            right: true,
        }
    }

    fn is_visible(&self) -> bool {
        self.top || self.bottom || self.left || self.right
    }
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let tree_height = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut visibility_grid = [[Visibility::new(); 100]; 100];

        for y in 0..tree_height.len() {
            for x in 0..tree_height[y].len() {
                if x == 0 || y == 0 || x == tree_height[0].len() - 1 || y == tree_height.len() - 1 {
                    continue;
                }

                let mut tree_vis = &mut visibility_grid[y][x];
                let height = tree_height[y][x];

                for dx in 0..x {
                    if tree_height[y][dx] >= height {
                        tree_vis.left = false;
                        break;
                    }
                }

                for dx in (x + 1)..tree_height.len() {
                    if tree_height[y][dx] >= height {
                        tree_vis.right = false;
                        break;
                    }
                }

                for dy in 0..y {
                    if tree_height[dy][x] >= height {
                        tree_vis.top = false;
                        break;
                    }
                }

                for dy in (y + 1)..tree_height.len() {
                    if tree_height[dy][x] >= height {
                        tree_vis.bottom = false;
                        break;
                    }
                }
            }
        }

        visibility_grid
            .iter()
            .flatten()
            .filter(|tree| tree.is_visible())
            .count()
            .to_string()
    }

    fn part2(&self, _input: &str) -> String {
        todo!();
    }
}
