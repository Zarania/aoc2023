advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let first = line
                    .as_bytes()
                    .iter()
                    .find(|&&c| c >= b'0' && c <= b'9')
                    .map(|c| *c as u32 - 48)
                    .unwrap();
                let last = line
                    .as_bytes()
                    .iter()
                    .rev()
                    .find(|&&c| c >= b'0' && c <= b'9')
                    .map(|c| *c as u32 - 48)
                    .unwrap();
                first * 10 + last
            })
            .sum(),
    )
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn digit_start(input: &str) -> Option<u32> {
    NUMBERS
        .iter()
        .position(|&n| input.starts_with(n))
        .map(|p| p as u32 + 1)
}

fn digit_end(input: &str) -> Option<u32> {
    NUMBERS
        .iter()
        .position(|&n| input.ends_with(n))
        .map(|p| p as u32 + 1)
}
pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let first;
                let last;

                let mut chars = line.chars();

                loop {
                    if let Some(n) = digit_start(chars.as_str()) {
                        first = n;
                        break;
                    }

                    if let Some(c) = chars.next() {
                        if c.is_ascii_digit() {
                            first = c.to_digit(10).unwrap();
                            break;
                        }
                    }
                }

                chars = line.chars();
                loop {
                    if let Some(n) = digit_end(chars.as_str()) {
                        last = n;
                        break;
                    }

                    if let Some(c) = chars.next_back() {
                        if c.is_ascii_digit() {
                            last = c.to_digit(10).unwrap();
                            break;
                        }
                    }
                }

                first * 10 + last
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
    fn solution_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(53312));
    }
}
