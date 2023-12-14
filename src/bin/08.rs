use hashers::fx_hash::FxHasher;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

advent_of_code::solution!(8);

fn gcd(a: usize, b: usize) -> usize {
    // Stein's algorithm
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => x,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn cycle_length(
    key: &str,
    nodes: &HashMap<&str, [&str; 2], BuildHasherDefault<FxHasher>>,
    steps: &str,
    target: &str,
) -> u32 {
    let mut key = key;
    steps
        .chars()
        .map(|s| match s {
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        })
        .cycle()
        .enumerate()
        .find(|&(_, step)| {
            if key.ends_with(target) {
                return true;
            }

            key = nodes[key][step];

            false
        })
        .unwrap()
        .0 as u32
}

fn parse(input: &str) -> (&str, HashMap<&str, [&str; 2], BuildHasherDefault<FxHasher>>) {
    let (steps, nodes) = input.split_once("\n\n").unwrap();

    let nodes = nodes
        .lines()
        .map(|l| {
            let key = &l[..3];
            let first = &l[7..10];
            let second = &l[12..15];

            (key, [first, second])
        })
        .collect::<HashMap<_, _, BuildHasherDefault<FxHasher>>>();
    (steps, nodes)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (steps, nodes) = parse(input);

    Some(cycle_length("AAA", &nodes, steps, "ZZZ"))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (steps, nodes) = parse(input);

    Some(
        nodes
            .iter()
            .filter(|(key, _)| key.ends_with('A'))
            .map(|k| cycle_length(k.0, &nodes, steps, "Z") as usize)
            .fold(1, |a, b| (a * b) / gcd(a, b)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_pt2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(13301));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(7309459565207));
    }
}
