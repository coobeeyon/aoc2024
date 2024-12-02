advent_of_code::solution!(2);
use itertools::Itertools;

fn is_safe(report: &Vec<i32>) -> bool {
    let differences = report.iter().tuple_windows().map(|(a, b)| b - a);
    (differences.clone().all(|i| i > 0) || differences.clone().all(|i| i < 0))
        && differences.clone().all(|i| i.abs() >= 1 && i.abs() <= 3)
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|i| i.parse::<i32>())
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect_vec();
    Some(parsed.into_iter().filter(is_safe).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
