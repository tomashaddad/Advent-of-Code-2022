use std::{cmp::Ordering, collections::HashMap};

use crate::problem::Day;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char, jokers: bool) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => match jokers {
                true => Card::Joker,
                false => Card::Jack,
            },
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card value"),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    fn from_str(cards: &str, bid: &str, jokers: bool) -> Hand {
        let card_vec = cards
            .chars()
            .map(|char| Card::from_char(char, jokers))
            .collect::<Vec<_>>();

        let mut counts_map = card_vec.iter().fold(HashMap::new(), |mut acc, card| {
            let count = acc.entry(card).or_insert(0);
            *count += 1;
            acc
        });

        if jokers && card_vec.iter().filter(|&&card| card == Card::Joker).count() != 5 {
            let jokers_removed = counts_map.remove(&Card::Joker).unwrap_or(0);
            if jokers_removed > 0 {
                let highest_card = **counts_map.iter().max_by_key(|(_, &count)| count).unwrap().0;
                *counts_map.get_mut(&highest_card).unwrap() += jokers_removed;
            }
        }

        let card_counts = counts_map.values().copied().collect::<Vec<_>>();

        let max_count = card_counts
            .iter()
            .max()
            .expect("There must be a max count of some card type");

        let hand_type = match max_count {
            1 => HandType::HighCard,
            2 => {
                if card_counts.iter().filter(|&&count| count == 2).count() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            3 => {
                if card_counts.iter().any(|&count| count == 2) {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            4 => HandType::FourOfAKind,
            5 => HandType::FiveOfAKind,
            _ => panic!("Invalid number of cards"),
        };

        Hand {
            cards: card_vec,
            hand_type,
            bid: bid.parse().expect("The bid should be a number"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type != other.hand_type {
            return Some(self.hand_type.cmp(&other.hand_type));
        }

        for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
            match card1.cmp(card2) {
                Ordering::Equal => continue,
                ordering => return Some(ordering),
            }
        }
        None // will never be the case for this problem but y'know
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards // probably wrong but good enough for this since we never compare hands
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn run(input: &str, jokers: bool) -> String {
    let mut hands = input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(cards, bid)| Hand::from_str(cards, bid, jokers))
                .unwrap()
        })
        .collect::<Vec<_>>();

    hands.sort_by(|hand1, hand2| hand1.partial_cmp(hand2).unwrap());

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, hand| acc + hand.1.bid * (hand.0 + 1) as u32)
        .to_string()
}

pub struct Code;
impl Day for Code {
    fn part1(&self, input: &str) -> String {
        run(input, false)
    }

    fn part2(&self, input: &str) -> String {
        run(input, true)
    }
}
