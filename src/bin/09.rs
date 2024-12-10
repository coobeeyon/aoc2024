use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Sector {
    Id(u64),
    Free,
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
            itertools::repeat_n(Sector::Id(i as u64), file_length as usize)
                .chain(itertools::repeat_n(
                    Sector::Free,
                    free_length.unwrap_or(0) as usize,
                ))
                .collect_vec()
        })
        .collect_vec();
    let mut start_index = 0;
    let mut end_index = disk.len() - 1;
    None
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
