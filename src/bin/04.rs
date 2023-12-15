advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let colon_position = input.iter().position(|&b| b == b':').unwrap();
    let pipe_position = input.iter().position(|&b| b == b'|').unwrap();

    Some(
        input
            .split(|b| b == &b'\n')
            .map(|game| {
                let winners = &game[colon_position + 1..pipe_position];
                game[pipe_position + 1..]
                    .chunks_exact(3)
                    .filter(|n| winners.chunks_exact(3).any(|c| &c == n))
                    .count() as u32
            })
            .filter(|&n| n > 0)
            .map(|n| 2u32.pow(n - 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let colon_position = input.iter().position(|&b| b == b':').unwrap();
    let pipe_position = input.iter().position(|&b| b == b'|').unwrap();
    let mut multipliers = [1usize; 256];

    Some(
        input
            .split(|b| b == &b'\n')
            .enumerate()
            .map(|(i, game)| {
                let multiplier = multipliers[i];
                let winners = &game[colon_position + 1..pipe_position];
                let wins = game[pipe_position + 1..]
                    .chunks_exact(3)
                    .filter(|n| winners.chunks_exact(3).any(|c| &c == n))
                    .count();
                (i..i + wins).for_each(|i| multipliers[i + 1] += multiplier);
                multiplier * wins + 1
            })
            .sum::<usize>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(25004));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(14427616));
    }
}
