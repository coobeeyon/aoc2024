use itertools::{Chunk, Itertools};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let board = input.lines().collect_vec();
    let width = board[0].len();
    let height = board.len();
    let occupied = board
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                if c != '.' {
                    Some((c, (col, row)))
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
    dbg!(occupied);
    None
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
