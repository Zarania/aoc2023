#![feature(iter_advance_by)]

use hashers::fx_hash::FxHasher;
use std::collections::HashSet;
use std::hash::BuildHasherDefault;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let colon_position = input.find(':').unwrap() + 2;

    Some(
        input
            .lines()
            .map(|line| {
                let mut game = line.chars();
                let _ = game.advance_by(colon_position);
                let mut numbers = HashSet::with_capacity_and_hasher(
                    10,
                    BuildHasherDefault::<FxHasher>::default(),
                );
                let mut winners = true;
                let mut value = 0;
                let mut count = 0;
                for c in game {
                    match c {
                        '|' => winners = false,
                        ' ' => {
                            if value == 0 {
                                continue;
                            }

                            if winners {
                                numbers.insert(value);
                            } else if numbers.contains(&value) {
                                count += 1;
                            }
                            value = 0;
                        }
                        _ => value = value * 10 + c.to_digit(10).unwrap(),
                    }
                }
                if numbers.contains(&value) {
                    count += 1;
                }
                count
            })
            .filter(|&n| n > 0)
            .map(|n| 2u32.pow(n - 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let colon_position = input.find(':').unwrap() + 2;
    let mut copies = vec![1; input.lines().count()];

    for (line_index, line) in input.lines().enumerate() {
        let mut game = line.chars();
        let _ = game.advance_by(colon_position);
        
        let mut numbers = HashSet::with_capacity_and_hasher(
            10,
            BuildHasherDefault::<FxHasher>::default(),
        );
        let mut winners = true;
        let mut count = 0;

        let mut value = 0;
        for c in game {
            match c {
                '|' => winners = false,
                ' ' => {
                    if value == 0 {
                        continue;
                    }
                    
                    if winners {
                        numbers.insert(value);
                    } else if numbers.contains(&value) {
                        count += 1;
                    }
                    
                    value = 0;
                }
                _ => value = value * 10 + c.to_digit(10).unwrap(),
            }
        }
        if numbers.contains(&value) {
            count += 1;
        }
        for i in 0..count {
            copies[line_index + i + 1] += copies[line_index];
        }
    }

    Some(copies.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(25004));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }

    #[test]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(14427616));
    }
}
