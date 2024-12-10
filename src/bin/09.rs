use itertools::{iterate, Itertools};

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Free,
    Id(u64),
}

pub fn part_one(input: &str) -> Option<u64> {
    let trimmed_input = input.replace('\n', "");
    let nums = trimmed_input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64);
    let blocks = iterate(Block::Id(0), |b| match b {
        Block::Id(i) => Block::Id(i + 1),
        _ => panic!(),
    })
    .intersperse(Block::Free);
    let disk = nums.zip(blocks).collect_vec();
    let mut disk_iter = disk.iter().cloned();
    let mut reversed_files_iter = disk.iter().cloned().rev().filter_map(|(l, b)| match b {
        Block::Id(i) => Some((l, i)),
        Block::Free => None,
    });

    let mut packed = Vec::new();
    let mut current_block = disk_iter.next();
    let mut to_pack = reversed_files_iter.next();
    loop {
        if to_pack.is_none() {
            break;
        } else if current_block.is_none() {
            packed.push(to_pack.unwrap());
            to_pack = reversed_files_iter.next()
        } else if let Some((block_length, Block::Id(block_id))) = current_block {
            let (pack_length, pack_id) = to_pack.unwrap();
            if block_id >= pack_id {
                if block_id == pack_id {
                    packed.push((pack_length, block_id));
                }
                break;
            } else {
                packed.push((block_length, block_id));
                current_block = disk_iter.next();
            }
        } else {
            let (pack_length, pack_id) = to_pack.unwrap();
            let (free_length, _) = current_block.unwrap();
            if free_length == 0 {
                current_block = disk_iter.next();
            } else if free_length >= pack_length {
                packed.push((pack_length, pack_id));
                current_block = Some((free_length - pack_length, Block::Free));
                to_pack = reversed_files_iter.next();
            } else {
                // free < l
                packed.push((free_length, pack_id));
                current_block = disk_iter.next();
                to_pack = Some((pack_length - free_length, pack_id));
            }
        }
    }
    let packed_unique = packed
        .into_iter()
        .tuple_windows()
        .map(|((l0, i0), (l1, i1))| if i0 == i1 { (l0 + l1, i0) } else { (l0, i0) })
        .collect_vec();
    dbg!(&packed_unique);
    let checksum = packed_unique
        .into_iter()
        .fold((0, 0), |(pos, sum), (length, id)| {
            (
                pos + length,
                sum + (pos..pos + length).map(|i| id * i).sum::<u64>(),
            )
        });
    Some(checksum.1)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
