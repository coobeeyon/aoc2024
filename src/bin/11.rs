use std::collections::HashMap;

use itertools::Itertools;
use std::sync::{LazyLock, Mutex};

advent_of_code::solution!(11);

static CACHE: LazyLock<Mutex<HashMap<(u64, u32), u64>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn score_store(num: u64, max_depth: u32) -> u64 {
    if let Some(&ret) = CACHE.lock().unwrap().get(&(num, max_depth)) {
        return ret;
    }

    let ret = if max_depth == 0 {
        1
    } else if num == 0 {
        score_store(1, max_depth - 1)
    } else {
        let num_str = format!("{}", num);
        let len = num_str.len();
        if num_str.len() % 2 == 0 {
            let left = num_str[..len / 2].parse().unwrap();
            let right = num_str[len / 2..].parse().unwrap();
            score_store(left, max_depth - 1) + score_store(right, max_depth - 1)
        } else {
            score_store(num * 2024, max_depth - 1)
        }
    };
    CACHE.lock().unwrap().insert((num, max_depth), ret);
    ret
}

pub fn part_one(input: &str) -> Option<u64> {
    let nums = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();
    let num_rocks = nums.into_iter().map(|num| score_store(num, 25)).sum();
    Some(num_rocks)
}

pub fn part_two(input: &str) -> Option<u64> {
    let nums = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();
    let num_rocks = nums.into_iter().map(|num| score_store(num, 75)).sum();
    Some(num_rocks)
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
        assert_eq!(result, Some(65601038650482));
    }
}
