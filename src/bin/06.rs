advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_ascii_whitespace();
    let distances = lines.next().unwrap().split_ascii_whitespace();

    Some(
        times
            .zip(distances)
            .skip(1)
            .map(|(t, d)| (t.parse::<u32>().unwrap(), d.parse::<u32>().unwrap()))
            .map(|(b, c)| {
                let root = ((b * b - 4 * c) as f32).sqrt();
                let min = ((-(b as f32) + root) / -2.0).floor() as u32 + 1;
                let max = ((-(b as f32) - root) / -2.0).ceil() as u32 - 1;
                max - min + 1
            })
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let b = String::from_iter(lines.next().unwrap().split_ascii_whitespace().skip(1))
        .parse::<u64>()
        .unwrap();
    let c = String::from_iter(lines.next().unwrap().split_ascii_whitespace().skip(1))
        .parse::<u64>()
        .unwrap();

    let root = ((b * b - 4 * c) as f64).sqrt();
    let min = ((-(b as f64) + root) / -2.0).floor() as u64 + 1;
    let max = ((-(b as f64) - root) / -2.0).ceil() as u64 - 1;
    Some(max - min + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(219849));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(29432455));
    }
}
