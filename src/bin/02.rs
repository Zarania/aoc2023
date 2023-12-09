#![feature(iter_advance_by)]

use std::cmp;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .enumerate()
            .map(|(index, line)| {
                let mut red = 0u32;
                let mut blue = 0u32;
                let mut green = 0u32;

                let mut chars = line.chars();

                if index < 10 {
                    let _ = chars.advance_by(8);
                } else if index < 100 {
                    let _ = chars.advance_by(9);
                } else {
                    let _ = chars.advance_by(10);
                }

                let mut value = 0u32;
                while let Some(c) = chars.next() {
                    match c {
                        'r' => {
                            red = cmp::max(red, value);
                            value = 0;
                            let _ = chars.advance_by(4);
                        }
                        'b' => {
                            blue = cmp::max(blue, value);
                            value = 0;
                            let _ = chars.advance_by(5);
                        }
                        'g' => {
                            green = cmp::max(green, value);
                            value = 0;
                            let _ = chars.advance_by(6);
                        }
                        ' ' => {}
                        _ => {
                            value *= 10;
                            value += c.to_digit(10).unwrap();
                        }
                    }
                }
                
                (index, (red, blue, green))
            })
            .filter(|(_, game)| game.0 <= 12 && game.1 <= 14 && game.2 <= 13)
            .map(|(index, _)| index as u32 + 1)
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .enumerate()
            .map(|(index, line)| {
                let mut red = 0u32;
                let mut blue = 0u32;
                let mut green = 0u32;

                let mut chars = line.chars();

                if index < 10 {
                    let _ = chars.advance_by(8);
                } else if index < 100 {
                    let _ = chars.advance_by(9);
                } else {
                    let _ = chars.advance_by(10);
                }

                let mut value = 0u32;
                while let Some(c) = chars.next() {
                    match c {
                        'r' => {
                            red = cmp::max(red, value);
                            value = 0;
                            let _ = chars.advance_by(4);
                        }
                        'b' => {
                            blue = cmp::max(blue, value);
                            value = 0;
                            let _ = chars.advance_by(5);
                        }
                        'g' => {
                            green = cmp::max(green, value);
                            value = 0;
                            let _ = chars.advance_by(6);
                        }
                        ' ' => {}
                        _ => {
                            value *= 10;
                            value += c.to_digit(10).unwrap();
                        }
                    }
                }
                
                (red, blue, green)
            })
            .map(|game| game.0 * game.1 * game.2)
            .sum()
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
    fn solution_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2204));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
