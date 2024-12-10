use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let board = input.lines().collect_vec();
    let width = board[0].len() as i32;
    let height = board.len() as i32;
    let occupied = board
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                if c != '.' {
                    Some((c, (col as i32, row as i32)))
                } else {
                    None
                }
            })
        })
        .sorted_by(|(ca, _), (cb, _)| ca.cmp(cb))
        .chunk_by(|(c, _)| *c)
        .into_iter()
        .map(|(key, chunk)| (key, chunk.map(|(_, pos)| pos).collect_vec()))
        .collect_vec();

    let antinodes = occupied
        .iter()
        .flat_map(|(_c, positions)| positions.iter().cartesian_product(positions.iter()))
        .filter(|(a, b)| a != b)
        .flat_map(|((x0, y0), (x1, y1))| {
            let (dx, dy) = (x1 - x0, y1 - y0);
            [(x0 - dx, y0 - dy), (x1 + dx, y1 + dy)]
        })
        .filter(|(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
        .collect::<HashSet<(i32, i32)>>();

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = input.lines().collect_vec();
    let width = board[0].len() as i32;
    let height = board.len() as i32;
    let occupied = board
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                if c != '.' {
                    Some((c, (col as i32, row as i32)))
                } else {
                    None
                }
            })
        })
        .sorted_by(|(ca, _), (cb, _)| ca.cmp(cb))
        .chunk_by(|(c, _)| *c)
        .into_iter()
        .map(|(key, chunk)| (key, chunk.map(|(_, pos)| pos).collect_vec()))
        .collect_vec();

    let in_bounds = |(x, y): &(i32, i32)| x >= &0 && x < &width && y >= &0 && y < &height;

    let antinodes = occupied
        .iter()
        .flat_map(|(_c, positions)| positions.iter().cartesian_product(positions.iter()))
        .filter(|(a, b)| a != b)
        .flat_map(|((x0, y0), (x1, y1))| {
            let (dx, dy) = (x1 - x0, y1 - y0);
            itertools::iterate((*x0, *y0), move |(x, y)| ((x - dx), (y - dy)))
                .take_while(in_bounds)
                .chain(
                    itertools::iterate((*x1, *y1), move |(x, y)| ((x + dx), (y + dy)))
                        .take_while(in_bounds),
                )
        })
        .filter(in_bounds)
        .collect::<HashSet<(i32, i32)>>();

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
