#![feature(iter_array_chunks)]

use std::{collections::VecDeque, ops::RangeInclusive};
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, transforms) = input.split_once("\n\n").unwrap();
    let mut seeds = seeds
        .split(' ')
        .skip(1)
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let transforms = transforms
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .skip(1)
                .map(|l| {
                    let mut split = l.split(' ');
                    let destination = split.next().unwrap().parse::<u64>().unwrap();
                    let source = split.next().unwrap().parse::<u64>().unwrap();
                    let length = split.next().unwrap().parse::<u64>().unwrap();
                    (destination, source..=(source + length))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for transform in &transforms {
        for seed in &mut seeds {
            *seed = transform
                .iter()
                .find(|(_, range)| range.contains(seed))
                .map(|(destination, range)| *seed - range.start() + destination)
                .unwrap_or(*seed);
        }
    }

    seeds.iter().min().copied()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, transforms) = input.split_once("\n\n").unwrap();
    let mut seeds = seeds
        .split(' ')
        .skip(1)
        .filter_map(|s| s.parse::<u64>().ok())
        .array_chunks::<2>()
        .map(|chunk| chunk[0]..=(chunk[0] + chunk[1] - 1))
        .collect::<VecDeque<_>>();

    let transforms = transforms
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .skip(1)
                .map(|l| {
                    let mut split = l.split(' ');
                    let destination = split.next().unwrap().parse::<u64>().unwrap();
                    let source = split.next().unwrap().parse::<u64>().unwrap();
                    let length = split.next().unwrap().parse::<u64>().unwrap();
                    (destination, source..=(source + length - 1))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for transform in &transforms {
        let mut mapped_seeds = Vec::new();
        while !seeds.is_empty() {
            let mut seed = seeds.pop_front().unwrap();
            for (destination, range) in transform {
                //if seed is fully out, exit early
                if seed.start() > range.end() || seed.end() < range.start() || seed.is_empty() {
                    continue;
                }

                //grab part before range and push
                if seed.start() < range.start() {
                    seeds.push_back(*seed.start()..=(range.start() - 1));
                    seed = *range.start()..=*seed.end();
                }

                //grab part after range and push
                if seed.end() > range.end() {
                    seeds.push_back((range.end() + 1)..=*seed.end());
                    seed = *seed.start()..=*range.end();
                }

                //map the new range
                let new_start = seed.start() - range.start() + destination;
                let length = seed.end() - seed.start();
                seed = RangeInclusive::new(1, 0);
                mapped_seeds.push(new_start..=(new_start + length));
            }

            if !seed.is_empty() {
                mapped_seeds.push(seed.clone());
            }
        }

        seeds = VecDeque::from(mapped_seeds);
    }

    seeds.iter().map(|s| s.start()).min().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(324724204));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(104070862));
    }
}
