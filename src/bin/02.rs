#![feature(iter_advance_by)]

use std::cmp;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .as_bytes()
            .split(|b| b == &b'\n')
            .enumerate()
            .map(|(index, line)| {
                let mut red = 0u32;
                let mut blue = 0u32;
                let mut green = 0u32;

                let mut i = 0;

                if index < 10 {
                    i += 8;
                } else if index < 100 {
                    i += 9;
                } else {
                    i += 10;
                }

                let mut value = 0u32;
                while i < line.len() {
                    match line[i] {
                        b'r' => {
                            red = cmp::max(red, value);
                            value = 0;
                            i += 4;
                        }
                        b'b' => {
                            blue = cmp::max(blue, value);
                            value = 0;
                            i += 5;
                        }
                        b'g' => {
                            green = cmp::max(green, value);
                            value = 0;
                            i += 6;
                        }
                        b' ' => {}
                        x => {
                            value *= 10;
                            value += (x - b'0') as u32;
                        }
                    }
                    i += 1;
                }
                (index, (red, blue, green))
            })
            .filter(|(_, game)| game.0 <= 12 && game.1 <= 14 && game.2 <= 13)
            .map(|(index, _)| index as u32 + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .as_bytes()
            .split(|b| b == &b'\n')
            .enumerate()
            .map(|(index, line)| {
                let mut red = 0u32;
                let mut blue = 0u32;
                let mut green = 0u32;

                let mut i = 0;

                if index < 10 {
                    i += 8;
                } else if index < 100 {
                    i += 9;
                } else {
                    i += 10;
                }

                let mut value = 0u32;
                while i < line.len() {
                    match line[i] {
                        b'r' => {
                            red = cmp::max(red, value);
                            value = 0;
                            i += 4;
                        }
                        b'b' => {
                            blue = cmp::max(blue, value);
                            value = 0;
                            i += 5;
                        }
                        b'g' => {
                            green = cmp::max(green, value);
                            value = 0;
                            i += 6;
                        }
                        b' ' => {}
                        x => {
                            value *= 10;
                            value += (x - b'0') as u32;
                        }
                    }
                    i += 1;
                }

                (red, blue, green)
            })
            .map(|game| game.0 * game.1 * game.2)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    #[ignore]
    fn solution_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2204));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }

    #[test]
    #[ignore]
    fn solution_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(71036));
    }
}
