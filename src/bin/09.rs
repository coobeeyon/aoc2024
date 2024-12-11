use std::fmt::Debug;

use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Block {
    Id(u64),
    Free,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct File {
    block: Block,
    length: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let trimmed_input = input.replace('\n', "");
    let nums = trimmed_input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64);
    let mut disk = nums
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(i, mut c)| {
            let file_length = c.next().unwrap();
            let free_length = c.next();
            itertools::repeat_n(Block::Id(i as u64), file_length as usize)
                .chain(itertools::repeat_n(
                    Block::Free,
                    free_length.unwrap_or(0) as usize,
                ))
                .collect_vec()
        })
        .collect_vec();
    let mut start_index = 0;
    let mut end_index = disk.len() - 1;

    while start_index < end_index {
        while disk.get(end_index) == Some(&Block::Free) {
            disk.pop();
            end_index -= 1;
        }
        if start_index >= end_index {
            break;
        }
        if let Some(Block::Id(_)) = disk.get(start_index) {
            start_index += 1;
        } else {
            disk[start_index] = disk[end_index];
            disk.pop();
            start_index += 1;
            end_index -= 1;
        }
    }

    let checksum = disk
        .into_iter()
        .enumerate()
        .map(|(i, sector)| match sector {
            Block::Free => 0,
            Block::Id(id) => id * i as u64,
        })
        .sum();
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let trimmed_input = input.replace('\n', "");
    let nums = trimmed_input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64);
    let mut disk = nums
        .into_iter()
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(id, mut c)| {
            let file_length = c.next().unwrap();
            let free_length = c.next();
            [
                File {
                    length: file_length,
                    block: Block::Id(id as u64),
                },
                File {
                    length: free_length.unwrap_or(0),
                    block: Block::Free,
                },
            ]
        })
        .collect_vec();

    let mut move_index = disk.len() - 1;
    while move_index != 0 {
        let move_file = disk[move_index];
        if move_file.block == Block::Free {
            move_index -= 1;
        } else if let Some((gap_index, gap_file)) = disk
            .iter()
            .find_position(|file| file.block == Block::Free && file.length >= move_file.length)
        {
            if gap_index < move_index {
                let gap_file = *gap_file;
                if gap_file.length == move_file.length {
                    disk[move_index] = gap_file;
                    disk[gap_index] = move_file;
                    move_index -= 1;
                } else {
                    disk[gap_index] = move_file;
                    disk[move_index] = File {
                        length: move_file.length,
                        block: Block::Free,
                    };
                    disk.insert(
                        gap_index + 1,
                        File {
                            length: gap_file.length - move_file.length,
                            block: Block::Free,
                        },
                    );
                }
            } else {
                move_index -= 1;
            }
        } else {
            move_index -= 1;
        }
    }

    let checksum: u64 = disk
        .into_iter()
        .fold((0, 0), |(pos, sum), file| {
            (
                pos + file.length,
                (pos..pos + file.length)
                    .map(|pos| match file.block {
                        Block::Free => 0,
                        Block::Id(i) => i * pos,
                    })
                    .sum::<u64>()
                    + sum,
            )
        })
        .1;
    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
