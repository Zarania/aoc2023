use atoi::atoi;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input
        .as_bytes()
        .split(|b| b == &b'\n')
        .map(|hand| {
            let mut ranks = [0u8; 13];
            let mut power = 0;

            for i in 0..5 {
                let card = match hand[i] {
                    b'A' => 12,
                    b'K' => 11,
                    b'Q' => 10,
                    b'J' => 9,
                    b'T' => 8,
                    n => n - b'0' - 2,
                };
                ranks[card as usize] += 1;
                power |= (card as u32) << 4 * (4 - i);
            }

            ranks.sort_unstable_by(|a, b| b.cmp(a));

            power |= match ranks[0] {
                5 => 6,
                4 => 5,
                3 if ranks[1] == 2 => 4,
                3 => 3,
                2 if ranks[1] == 2 => 2,
                2 => 1,
                _ => 0,
            } << 29;
            (power, atoi::<u32>(&hand[6..]).unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort_unstable();

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, (_, bet))| bet * (i as u32 + 1))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = input
        .as_bytes()
        .split(|b| b == &b'\n')
        .map(|hand| {
            let mut ranks = [0u8; 13];
            let mut power = 0;
            let mut jokers = 0;

            for i in 0..5 {
                let card = match hand[i] {
                    b'A' => 12,
                    b'K' => 11,
                    b'Q' => 10,
                    b'J' => 0,
                    b'T' => 9,
                    n => n - b'0' - 1,
                };
                ranks[card as usize] += 1 * (card != 0) as u8;
                jokers += 1 * (card == 0) as u8;
                power |= (card as u32) << 4 * (4 - i);
            }

            ranks.sort_unstable_by(|a, b| b.cmp(a));

            power |= match ranks[0] + jokers {
                5 => 6,
                4 => 5,
                3 if ranks[1] == 2 => 4,
                3 => 3,
                2 if ranks[1] == 2 => 2,
                2 => 1,
                _ => 0,
            } << 29;
            (power, atoi::<u32>(&hand[6..]).unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort_unstable();

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, (_, bet))| bet * (i as u32 + 1))
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(249638405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(249776650));
    }
}
