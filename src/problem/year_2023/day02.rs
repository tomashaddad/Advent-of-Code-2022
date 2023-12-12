use crate::problem::Day;

pub struct Code;
impl Day for Code {
    fn part1(&self, input: &str) -> String {
        input
            .lines()
            .flat_map(|line| line.split_once(": "))
            .flat_map(|(game, rounds)| {
                rounds
                    .split("; ")
                    .all(|round| {
                        round
                            .split(", ")
                            .flat_map(Pull::try_from)
                            .all(|pull| pull.colour.is_valid_amount(&pull))
                    })
                    .then_some(game.split_once(' ').unwrap().1.parse::<u32>().unwrap())
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, _input: &str) -> String {
        todo!();
    }
}

enum Colour {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for Colour {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Colour::Red),
            "green" => Ok(Colour::Green),
            "blue" => Ok(Colour::Blue),
            _ => Err(()),
        }
    }
}

struct Pull {
    colour: Colour,
    amount: u32,
}

impl TryFrom<&str> for Pull {
    type Error = ();

    fn try_from(pair: &str) -> Result<Self, Self::Error> {
        let (value, colour) = pair.split_once(' ').ok_or(())?;
        let colour = Colour::try_from(colour)?;
        let amount = value.parse::<u32>().map_err(|_| ())?;
        Ok(Pull { colour, amount })
    }
}

impl Colour {
    fn is_valid_amount(&self, pull: &Pull) -> bool {
        match self {
            Colour::Red => pull.amount <= 12,
            Colour::Green => pull.amount <= 13,
            Colour::Blue => pull.amount <= 14,
        }
    }
}
