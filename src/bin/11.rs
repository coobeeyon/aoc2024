use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let nums = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();
    let num_rocks = itertools::iterate(nums, |nums| {
        nums.iter()
            .flat_map(|num| {
                if *num == 0 {
                    vec![1]
                } else {
                    let num_str = format!("{}", num);
                    let len = num_str.len();
                    if num_str.len() % 2 == 0 {
                        let left = num_str[..len / 2].parse().unwrap();
                        let right = num_str[len / 2..].parse().unwrap();
                        vec![left, right]
                    } else {
                        vec![num * 2024]
                    }
                }
            })
            .collect_vec()
    })
    .take(26)
    .last()
    .unwrap()
    .len();
    Some(num_rocks as u64)
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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
