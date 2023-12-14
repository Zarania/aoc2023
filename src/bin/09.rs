advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .as_bytes()
            .split(|b| b == &b'\n')
            .map(|line| {
                line.split(|b| b == &b' ')
                    .map(|n| atoi::atoi(n).unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|v| {
                let mut stack = vec![v];
                while let Some(last) = stack.last() {
                    if last.iter().all(|&n| n == 0) {
                        break;
                    }

                    stack.push(
                        last.windows(2)
                            .map(|window| window[1] - window[0])
                            .collect::<Vec<_>>(),
                    );
                }

                stack
            })
            .map(|stack| stack.iter().fold(0, |acc, vec| acc + vec.last().unwrap()))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .as_bytes()
            .split(|b| b == &b'\n')
            .map(|line| {
                line.split(|b| b == &b' ')
                    .map(|n| atoi::atoi(n).unwrap())
                    .rev()
                    .collect::<Vec<_>>()
            })
            .map(|v| {
                let mut stack = vec![v];
                while let Some(last) = stack.last() {
                    if last.iter().all(|&n| n == 0) {
                        break;
                    }

                    stack.push(
                        last.windows(2)
                            .map(|window| window[1] - window[0])
                            .collect::<Vec<_>>(),
                    );
                }

                stack
            })
            .map(|stack| stack.iter().fold(0, |acc, vec| acc + vec.last().unwrap()))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2101499000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1089));
    }
}
