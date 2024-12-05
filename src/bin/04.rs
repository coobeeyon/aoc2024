use itertools::{izip, multizip, Itertools};

advent_of_code::solution!(4);

pub fn count_xmas(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            ["XMAS", "SAMX"]
                .iter()
                .map(|pat| line.match_indices(pat).count() as u32)
                .sum::<u32>()
        })
        .sum()
}
fn transpose(rows: &[String]) -> Vec<String> {
    let num_cols = rows[0].len();
    let cols = (0..num_cols)
        .map(|i| {
            rows.iter()
                .map(|row| row.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect_vec();
    cols
}
pub fn part_one(input: &str) -> Option<u32> {
    let rows = input.lines().map(|line| line.to_string()).collect_vec();

    let num_horizontal_matches = count_xmas(&rows);

    let cols = transpose(&rows);
    let num_vertical_matches = count_xmas(&cols);

    let num_rows = rows.len();
    let left_pads = (0..num_rows).map(|n| ".".repeat(n)).collect_vec();
    let right_pads = (left_pads.iter().rev().cloned()).collect_vec();
    let diags_1 = transpose(
        &izip!(left_pads.clone(), rows.clone(), right_pads.clone())
            .map(|(l, c, r)| l + &c + &r)
            .collect_vec(),
    );
    let num_diag_matches_1 = count_xmas(&diags_1);

    let diags_2 = transpose(
        &izip!(left_pads, rows, right_pads)
            .map(|(l, c, r)| r + &c + &l)
            .collect_vec(),
    );
    let num_diag_matches_2 = count_xmas(&diags_2);

    Some(num_horizontal_matches + num_vertical_matches + num_diag_matches_1 + num_diag_matches_2)
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
