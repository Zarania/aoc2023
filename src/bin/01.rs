advent_of_code::solution!(1);
#[allow(clippy::manual_range_contains)]

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .as_bytes()
            .split(|b| b == &b'\n')
            .map(|line| {
                let first = line
                    .iter()
                    .find(|&&c| c >= b'0' && c <= b'9')
                    .map(|c| (*c - b'0') as u32)
                    .unwrap();
                let last = line
                    .iter()
                    .rev()
                    .find(|&&c| c >= b'0' && c <= b'9')
                    .map(|c| (*c - b'0') as u32)
                    .unwrap();
                first * 10 + last
            })
            .sum(),
    )
}

const NUMBERS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

#[inline(always)]
fn number(line: &[u8], i: usize) -> Option<u32> {
    line[i]
        .is_ascii_digit()
        .then_some((line[i] - b'0') as u32)
        .or(NUMBERS
            .iter()
            .enumerate()
            .find(|(_, n)| line[i..].starts_with(n))
            .map(|(n, _)| n as u32 + 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .as_bytes()
            .split(|b| b == &b'\n')
            .map(|line| {
                (0..line.len()).find_map(|i| number(line, i)).unwrap() * 10
                    + (0..line.len()).rev().find_map(|i| number(line, i)).unwrap()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    #[ignore]
    fn solution_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(53386));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }

    #[test]
    #[ignore]
    fn solution_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(53312));
    }
}
