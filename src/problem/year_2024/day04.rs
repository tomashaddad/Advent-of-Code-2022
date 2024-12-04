use crate::problem::Day;

pub struct Code;

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // (row, column)
        let directions: [(i16, i16); 8] = [
            (-1, 0),  // up
            (1, 0),   // down
            (0, -1),  // left
            (0, 1),   // right
            (-1, -1), // upleft
            (1, -1),  // downleft
            (-1, 1),  // upright
            (1, 1),   // downright
        ];

        let xmas = "XMAS".chars().collect::<Vec<_>>();
        let mut count = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                count += directions
                    .iter()
                    .filter(|(dr, dc)| {
                        (0..4).all(|s| {
                            let rdrs = (r as i16 + (dr * s)) as usize;
                            let cdrs = (c as i16 + (dc * s)) as usize;
                            xmas.get(s as usize) == grid.get(rdrs).and_then(|row| row.get(cdrs))
                        })
                    })
                    .count();
            }
        }
        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // (row, column)
        // order matters (upleft <-> downright, upright <-> downleft) for answers array to work
        let directions: [(i16, i16); 4] = [
            (-1, -1), // upleft
            (1, 1),   // downright
            (1, -1),  // downleft
            (-1, 1),  // upright
        ];

        let answers = ["SMSM", "MSSM", "SMMS", "MSMS"];

        let mut count = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] != 'A' {
                    continue;
                }
                let surrounding = directions
                    .iter()
                    .filter_map(|(dr, dc)| {
                        let rdr = (r as i16 + dr) as usize;
                        let cdr = (c as i16 + dc) as usize;
                        grid.get(rdr).and_then(|row| row.get(cdr))
                    })
                    .collect::<String>();

                if answers.contains(&surrounding.as_str()) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }
}
