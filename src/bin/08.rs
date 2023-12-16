use std::cmp::{max, min};

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

#[inline(always)]
fn encode(slice: &[u8]) -> u32 {
    ((slice[0] - b'A') as u32) << 10 | ((slice[1] - b'A') as u32) << 5 | ((slice[2] - b'A') as u32)
}

fn cycle_length(directions: &[u8], start: u32, map: [u32; 27483]) -> Option<u32> {
    directions
        .iter()
        .cycle()
        .scan(start, |node, step| {
            *node = if step == &b'L' {
                map[*node as usize] & u16::MAX as u32
            } else {
                map[*node as usize] >> 16
            };

            Some(*node & 0b11111 == (b'Z' - b'A') as u32)
        })
        .position(|node| node)
        .map(|n| n as u32 + 1)
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let section_split = input.iter().position(|b| b == &b'\n').unwrap();
    let mut map = [0u32; (26 << 10 | 26 << 5 | 26) + 1];

    input[section_split + 2..]
        .split(|b| b == &b'\n')
        .for_each(|node| {
            map[encode(&node[0..3]) as usize] = encode(&node[7..10]) | encode(&node[12..15]) << 16;
        });

    let directions = &input[0..section_split];

    cycle_length(directions, encode(b"AAA"), map)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.as_bytes();
    let section_split = input.iter().position(|b| b == &b'\n').unwrap();
    let mut map = [0u32; (26 << 10 | 26 << 5 | 26) + 1];
    let mut starts = Vec::with_capacity(6);

    input[section_split + 2..]
        .split(|b| b == &b'\n')
        .for_each(|node| {
            map[encode(&node[0..3]) as usize] = encode(&node[7..10]) | encode(&node[12..15]) << 16;

            if node[2] == b'A' {
                starts.push(encode(&node[0..3]));
            }
        });

    let directions = &input[0..section_split];

    Some(
        starts
            .into_iter()
            .filter_map(|node| {
                cycle_length(directions, node, map)
            })
            .fold(1, |acc, x| acc * x as usize / gcd(acc, x as usize)),
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
    #[ignore]
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
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(7309459565207));
    }
}
