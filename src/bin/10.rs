use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let trailheads = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, n)| {
                    if *n == 0 {
                        Some(vec![(x as i32, y as i32)])
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    let score = trailheads
        .into_iter()
        .map(|trailhead| {
            (1..=9).fold(vec![trailhead], |paths, n| {
                paths
                    .into_iter()
                    .flat_map(|path| {
                        let (x0, y0) = path[path.len() - 1];
                        [(x0 - 1, y0), (x0 + 1, y0), (x0, y0 - 1), (x0, y0 + 1)]
                            .into_iter()
                            .filter_map(|pos| match pos {
                                (-1, _) => None,
                                (_, -1) => None,
                                (x, y) => map.get(y as usize).and_then(|row| {
                                    row.get(x as usize).and_then(|m| {
                                        if *m == n {
                                            Some(pos)
                                        } else {
                                            None
                                        }
                                    })
                                }),
                            })
                            .map(|pos| {
                                let mut path = path.clone();
                                path.push(pos);
                                path
                            })
                            .collect_vec()
                    })
                    .collect_vec()
            })
        })
        .map(|from_head| {
            from_head
                .into_iter()
                .filter_map(|v| v.last().cloned())
                .collect::<HashSet<(i32, i32)>>()
                .len()
        })
        .sum::<usize>();

    Some(score as u32)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
