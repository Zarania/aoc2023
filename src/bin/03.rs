advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut symbols = Vec::with_capacity(input.lines().size_hint().0);
    let mut numbers = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut value = 0u32;
        let mut start = 0u32;
        let mut symbol_row = Vec::new();
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                if value > 0 {
                    numbers.push((start, x as u32 - 1, y as u32, value));
                    value = 0;
                }
                continue;
            }

            if char.is_ascii_digit() {
                if value == 0 {
                    start = x as u32;
                }
                value *= 10;
                value += char.to_digit(10).unwrap();
                continue;
            }

            if value > 0 {
                numbers.push((start, x as u32 - 1, y as u32, value));
                value = 0;
            }

            symbol_row.push(x as u32);
        }

        if value > 0 {
            numbers.push((
                start,
                start + (0..).take_while(|i| 10u32.pow(*i) <= start).count() as u32 - 1,
                y as u32,
                value,
            ));
        }

        symbols.push(symbol_row);
    }

    Some(
        numbers
            .iter()
            .filter(|(start, end, row, _)| {
                let row_start = if *row == 0 { 0 } else { *row as usize - 1 };
                let row_end = if *row == (symbols.len() as u32 - 1) {
                    symbols.len() - 1
                } else {
                    *row as usize + 1
                };

                for symbol_row in symbols.iter().take(row_end + 1).skip(row_start) {
                    for symbol in symbol_row {
                        if &(symbol + 1) >= start && &(symbol - 1) <= end {
                            return true;
                        }
                    }
                }
                false
            })
            .map(|(_, _, _, n)| n)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut symbols = Vec::with_capacity(input.lines().size_hint().0);
    let mut numbers = Vec::with_capacity(input.lines().size_hint().0);

    for line in input.lines() {
        let mut value = 0u32;
        let mut start = 0u32;
        let mut symbol_row = Vec::new();
        let mut numbers_row = Vec::new();
        for (x, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                if value == 0 {
                    start = x as u32;
                }
                value *= 10;
                value += char.to_digit(10).unwrap();
                continue;
            }

            if value > 0 {
                numbers_row.push((start, x as u32 - 1, value));
                value = 0;
            }

            if char == '*' {
                symbol_row.push(x as u32);
                continue;
            }
        }

        if value > 0 {
            numbers_row.push((
                start,
                start + (0..).take_while(|i| 10u32.pow(*i) <= start).count() as u32 - 1,
                value,
            ));
        }

        numbers.push(numbers_row);
        symbols.push(symbol_row);
    }

    Some(
        symbols
            .iter()
            .enumerate()
            .map(|(y, gears)| {
                gears
                    .iter()
                    .map(|&x| {
                        let row_start = if y == 0 { 0 } else { y - 1 };
                        let row_end = if y == (numbers.len() - 1) {
                            numbers.len() - 1
                        } else {
                            y + 1
                        };

                        let mut first = 0;

                        for number_row in numbers.iter().take(row_end + 1).skip(row_start) {
                            for (start, end, value) in number_row {
                                if &(x + 1) >= start && &(x - 1) <= end {
                                    if first == 0 {
                                        first = *value;
                                        continue;
                                    }

                                    return first * value;
                                }
                            }
                        }

                        0
                    })
                    .sum::<u32>()
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn solution_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(525181));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }

    #[test]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(84289137));
    }
}
