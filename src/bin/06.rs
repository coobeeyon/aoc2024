use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Copy, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
        Direction::N => Direction::E,
    }
}

fn forward(dir: Direction, row: i32, col: i32) -> (i32, i32) {
    let next_row = match dir {
        Direction::N => row - 1,
        Direction::E => row,
        Direction::S => row + 1,
        Direction::W => row,
    };
    let next_col = match dir {
        Direction::N => col,
        Direction::E => col + 1,
        Direction::S => col,
        Direction::W => col - 1,
    };
    (next_row, next_col)
}

fn in_bounds(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let mut guard_row = map
        .iter()
        .find_position(|row| row.contains(&'^'))
        .unwrap()
        .0 as i32;
    let mut guard_col = map[guard_row as usize]
        .iter()
        .find_position(|c| **c == '^')
        .unwrap()
        .0 as i32;
    let mut guard_direction = Direction::N;

    while in_bounds(guard_col, guard_row, width, height) {
        map[guard_row as usize][guard_col as usize] = 'X';
        let (next_row, next_col) = forward(guard_direction, guard_row, guard_col);
        if in_bounds(next_col, next_row, width, height)
            && map[next_row as usize][next_col as usize] == '#'
        {
            guard_direction = turn_right(guard_direction);
        } else {
            (guard_col, guard_row) = (next_col, next_row);
        }
    }
    let num_guarded: u32 = map
        .iter()
        .map(|row| row.iter().filter(|c| **c == 'X').count() as u32)
        .sum();
    Some(num_guarded)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
