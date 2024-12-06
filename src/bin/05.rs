use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::{izip, Itertools};
use regex::Regex;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let rules_re = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let (rules_str, updates_str) = input.split("\n\n").collect_tuple().unwrap();
    let rules: HashMap<u32, HashSet<u32>> = rules_str
        .lines()
        .flat_map(|line| {
            rules_re
                .captures_iter(line)
                .map(|c| c.extract())
                .map(|(_, [before, after])| {
                    (
                        before.parse::<u32>().unwrap(),
                        after.parse::<u32>().unwrap(),
                    )
                })
        })
        .sorted()
        .chunk_by(|e| e.0)
        .into_iter()
        .map(|chunk| (chunk.0, chunk.1.map(|e| e.1).collect()))
        .collect();

    let updates = updates_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let processed_rows = updates
        .iter()
        .map(|update| {
            (
                update[update.len() / 2],
                izip!(
                    update.iter(),
                    (0..update.len())
                        .map(|i| update[0..i].iter().cloned().collect::<HashSet<u32>>())
                        .collect_vec(),
                )
                .collect_vec(),
            )
        })
        .collect_vec();

    let valid_sum = processed_rows
        .iter()
        .map(|(mid, rolling_row)| {
            if rolling_row.iter().any(|(this_char, seen_chars)| {
                if let Some(forbidden_chars) = rules.get(this_char) {
                    forbidden_chars.intersection(seen_chars).count() > 0
                } else {
                    false
                }
            }) {
                0
            } else {
                *mid
            }
        })
        .sum();

    Some(valid_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules_re = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let (rules_str, updates_str) = input.split("\n\n").collect_tuple().unwrap();
    let rules: HashSet<(u32, u32)> = rules_str
        .lines()
        .flat_map(|line| {
            rules_re
                .captures_iter(line)
                .map(|c| c.extract())
                .map(|(_, [before, after])| {
                    (
                        before.parse::<u32>().unwrap(),
                        after.parse::<u32>().unwrap(),
                    )
                })
        })
        .collect();

    let updates = updates_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let ret = updates
        .iter()
        .filter_map(|update| {
            let sorted_update = update
                .iter()
                .cloned()
                .sorted_by(|a, b| {
                    if rules.contains(&(*a, *b)) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .collect_vec();
            if sorted_update == *update {
                None
            } else {
                Some(sorted_update)
            }
        })
        .map(|update| update[update.len() / 2])
        .sum();

    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
