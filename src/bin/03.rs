use atoi::atoi;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.as_bytes();
    let width = grid.iter().position(|b| b == &b'\n').unwrap() as isize;

    Some(
        (0..grid.len())
            .filter(|i| {
                grid[*i].is_ascii_digit()
                    && !grid
                        .get(i.wrapping_sub(1))
                        .map_or(false, u8::is_ascii_digit)
            })
            .map(|i| {
                let size = (i + 1..).position(|i| !grid[i].is_ascii_digit()).unwrap() + 1;
                (i, size as _, atoi::<u32>(&grid[i..i + size]).unwrap())
            })
            .filter(|(i, size, _)| {
                //row above
                (-width - 2..-width + *size)
                    //left and right
                    .chain([-1, *size])
                    //row below
                    .chain(width..width + *size + 2)
                    .any(|j| {
                        grid.get((*i as isize + j) as usize)
                            .map_or(false, |b| b != &b'.' && b.is_ascii_punctuation())
                    })
            })
            .map(|(_, _, n)| n)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.as_bytes();
    let width = grid.iter().position(|b| b == &b'\n').unwrap() as isize;
    let mut ajacent: Vec<usize> = Vec::with_capacity(9);

    Some(
        (0..grid.len())
            .filter(|i| grid[*i] == b'*')
            .filter_map(|i| {
                ajacent.clear();

                ajacent.extend(
                    //above
                    (-width - 2..=-width)
                        //left, right
                        .chain([-1, 1])
                        //below
                        .chain(width..=width + 2)
                        .map(|offset| (i as isize + offset) as usize)
                        .filter(|offset| grid[*offset].is_ascii_digit())
                        .filter_map(|offset| {
                            (offset.saturating_sub(2)..=offset)
                                .rev()
                                .take_while(|n| grid[*n].is_ascii_digit())
                                .last()
                        }),
                );
                ajacent.dedup();
                (ajacent.len() == 2).then(|| {
                    ajacent
                        .iter()
                        .map(|i| atoi::<u32>(&grid[*i..i + 3]).unwrap())
                        .product::<u32>()
                })
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
    #[ignore]
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
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(84289137));
    }
}
