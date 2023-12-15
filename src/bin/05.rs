#![feature(iter_array_chunks)]

use atoi::atoi;
use std::{collections::VecDeque, ops::RangeInclusive};
advent_of_code::solution!(5);

const SEED_MAPS: usize = 7;

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let mut lines = input.split(|b| b == &b'\n').skip(2);

    let transforms = (0..SEED_MAPS)
        .map(|_| {
            (&mut lines)
                .skip(1)
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    line.splitn(3, |b| b == &b' ')
                        .map(|n| atoi(n).unwrap())
                        .array_chunks::<3>()
                        .map(|line: [u64; 3]| (line[1]..line[1] + line[2], line[0]))
                        .take(1)
                        .next()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    input[SEED_MAPS..input.iter().position(|b| b == &b'\n').unwrap()]
        .split(|b| b == &b' ')
        .flat_map(atoi)
        .map(|seed| {
            transforms.iter().fold(seed, |seed, map| {
                map.iter()
                    .find(|(range, _)| range.contains(&seed))
                    .map(|(range, destination)| destination + seed - range.start)
                    .unwrap_or(seed)
            })
        })
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let mut lines = input.split(|b| b == &b'\n').skip(2);

    let transforms = (0..SEED_MAPS)
        .map(|_| {
            (&mut lines)
                .skip(1)
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    line.splitn(3, |b| b == &b' ')
                        .map(|n| atoi(n).unwrap())
                        .array_chunks::<3>()
                        .map(|line: [u64; 3]| (line[1]..=line[1] + line[2] - 1, line[0]))
                        .take(1)
                        .next()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut seeds = input[SEED_MAPS..input.iter().position(|b| b == &b'\n').unwrap()]
        .split(|b| b == &b' ')
        .flat_map(atoi::<u64>)
        .array_chunks::<2>()
        .map(|chunk| chunk[0]..=(chunk[0] + chunk[1] - 1))
        .collect::<VecDeque<_>>();

    for transform in &transforms {
        let mut mapped_seeds = Vec::new();
        while !seeds.is_empty() {
            let mut seed = seeds.pop_front().unwrap();
            for (range, destination) in transform {
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
    #[ignore]
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
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(104070862));
    }
}
