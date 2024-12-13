use std::collections::HashSet;

use itertools::Itertools;

type GridPos = (i32, i32);
type Grid<T> = Vec<Vec<T>>;

fn size<T>(grid: &Grid<T>) -> (i32, i32) {
    (grid[0].len() as i32, grid.len() as i32)
}

fn neighbors((x, y): GridPos, width: i32, height: i32) -> impl Iterator<Item = (i32, i32)> {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter_map(move |(x, y)| {
            if x >= 0 && x < width && y >= 0 && y < height {
                Some((x, y))
            } else {
                None
            }
        })
}

advent_of_code::solution!(12);
pub fn flood_fill(
    grid: &mut Grid<(char, Option<u32>)>,
    pos: GridPos,
    group_num: u32,
    group_char: char,
) {
    let (x, y) = (pos.0 as usize, pos.1 as usize);
    let (c, group) = grid[y][x];
    let (width, height) = size(grid);
    if group.is_none() && c == group_char {
        grid[y][x] = (c, Some(group_num));
        neighbors(pos, width, height).for_each(|pos| flood_fill(grid, pos, group_num, group_char));
    }
}

pub fn perimeter(group: &HashSet<GridPos>, width: i32, height: i32) -> u32 {
    4 * (group.len() as u32)
        - group
            .iter()
            .map(|pos| {
                neighbors(*pos, width, height)
                    .filter(|neighbor_pos| group.contains(neighbor_pos))
                    .count() as u32
            })
            .sum::<u32>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Grid<(char, Option<u32>)> = input
        .lines()
        .map(|line| line.chars().map(|c| (c, None)).collect())
        .collect();
    let mut current_group = 0;
    let (width, height) = size(&grid);
    (0..height).cartesian_product(0..width).for_each(|(y, x)| {
        let (c, group) = grid[y as usize][x as usize];
        if group.is_none() {
            flood_fill(&mut grid, (x, y), current_group, c);
            current_group += 1;
        }
    });
    let groups = grid
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter()
                .map(|(_c, group)| group.unwrap())
                .enumerate()
                .map(move |(x, group)| (group, (x as i32, y as i32)))
        })
        .sorted_by(|(ga, _pa), (gb, _pb)| ga.cmp(gb))
        .chunk_by(|(group, _pos)| *group)
        .into_iter()
        .map(|chunk| {
            chunk
                .1
                .map(|(_group, pos)| pos)
                .collect::<HashSet<GridPos>>()
        })
        .collect_vec();
    let sum_scores = groups
        .iter()
        .map(|group| group.len() as u32 * perimeter(group, width, height))
        .sum();
    Some(sum_scores)
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
