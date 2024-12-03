advent_of_code::solution!(1);
use itertools::Itertools;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(i32::from_str)
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect_vec();
    let transposed = [0, 1]
        .into_iter()
        .map(|i| parsed.iter().cloned().map(|a| a[i]).sorted().collect_vec())
        .collect_vec();
    let distance = transposed[0]
        .iter()
        .zip(transposed[1].iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>() as u32;
    Some(distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(u32::from_str)
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect_vec();
    let transposed = [0, 1]
        .into_iter()
        .map(|i| parsed.iter().cloned().map(|a| a[i]).collect_vec())
        .collect_vec();
    if let [first, second] = &transposed[..] {
        let counts = second.iter().counts();
        let distance = first
            .iter()
            .map(|i| i * (*counts.get(i).unwrap_or(&0) as u32))
            .sum();
        Some(distance)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
