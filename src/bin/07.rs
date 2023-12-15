use std::cmp::{Ordering, Reverse};

advent_of_code::solution!(7);

#[derive(Eq, Debug)]
struct Hand {
    hand: [u32; 5],
    rank: u8,
    bid: u32,
}

impl Hand {
    pub fn parse(input: &str) -> Hand {
        let (hand, bid) = input.split_at(6);

        let mut cards: [u32; 15] = [0; 15];
        let mut original: [u32; 5] = [0; 5];

        for (i, card) in hand
            .chars()
            .take(5)
            .filter_map(|c| {
                c.to_digit(10).or(match c {
                    'T' => Some(10),
                    'J' => Some(11),
                    'Q' => Some(12),
                    'K' => Some(13),
                    'A' => Some(14),
                    _ => None,
                })
            })
            .enumerate()
        {
            original[i] = card;
            cards[card as usize] += 1;
        }

        let mut hand = cards
            .iter()
            .enumerate()
            .filter(|(_, &c)| c > 0)
            .map(|(i, c)| (i as u32, *c))
            .collect::<Vec<_>>();
        hand.sort_unstable_by_key(|card| Reverse((card.1, card.0)));

        let second = if hand.len() > 1 { hand[1] } else { (0, 0) };

        let rank = match (hand[0], second) {
            (a, _) if a.1 == 5 => 7,
            (a, _) if a.1 == 4 => 6,
            (a, b) if a.1 == 3 && b.1 == 2 => 5,
            (a, _) if a.1 == 3 => 4,
            (a, b) if a.1 == 2 && b.1 == 2 => 3,
            (a, _) if a.1 == 2 => 2,
            (_, _) => 1,
        };

        Hand {
            hand: original,
            rank,
            bid: bid.parse::<u32>().unwrap(),
        }
    }

    pub fn parse_jokers(input: &str) -> Hand {
        let (hand, bid) = input.split_at(6);

        let mut cards: [u32; 15] = [0; 15];
        let mut original: [u32; 5] = [0; 5];

        for (i, card) in hand
            .chars()
            .take(5)
            .filter_map(|c| {
                c.to_digit(10).or(match c {
                    'T' => Some(10),
                    'J' => Some(0),
                    'Q' => Some(12),
                    'K' => Some(13),
                    'A' => Some(14),
                    _ => None,
                })
            })
            .enumerate()
        {
            original[i] = card;
            cards[card as usize] += 1;
        }

        let jokers = cards[0];
        let mut hand = cards
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(_, &c)| c > 0)
            .map(|(i, c)| (i as u32, *c))
            .collect::<Vec<_>>();

        hand.sort_unstable_by_key(|card| Reverse((card.1, card.0)));

        if hand.is_empty() {
            hand.push((0, 0));
        }
        hand[0].1 += jokers;

        let second = if hand.len() > 1 { hand[1] } else { (0, 0) };

        let rank = match (hand[0], second) {
            (a, _) if a.1 == 5 => 7,
            (a, _) if a.1 == 4 => 6,
            (a, b) if a.1 == 3 && b.1 == 2 => 5,
            (a, _) if a.1 == 3 => 4,
            (a, b) if a.1 == 2 && b.1 == 2 => 3,
            (a, _) if a.1 == 2 => 2,
            (_, _) => 1,
        };

        Hand {
            hand: original,
            rank,
            bid: bid.parse::<u32>().unwrap(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand && self.rank == other.rank && self.bid == other.bid
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (_, _) if self.rank > other.rank => Ordering::Greater,
            (_, _) if self.rank < other.rank => Ordering::Less,
            (_, _) => self
                .hand
                .iter()
                .zip(&other.hand)
                .map(|(a, b)| a.cmp(b))
                .find(|&o| o != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input.lines().map(Hand::parse).collect::<Vec<_>>();
    hands.sort();

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(index, hand)| hand.bid * (index as u32 + 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = input.lines().map(Hand::parse_jokers).collect::<Vec<_>>();
    hands.sort();

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(index, hand)| hand.bid * (index as u32 + 1))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(249638405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(249776650));
    }
}
