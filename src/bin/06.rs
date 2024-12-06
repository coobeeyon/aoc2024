use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Copy, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

struct Pos {
    x: i32,
    y: i32,
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
        Direction::N => Direction::E,
    }
}

fn forward(dir: Direction, pos: &Pos) -> Pos {
    let next_row = match dir {
        Direction::N => pos.y - 1,
        Direction::E => pos.y,
        Direction::S => pos.y + 1,
        Direction::W => pos.y,
    };
    let next_col = match dir {
        Direction::N => pos.x,
        Direction::E => pos.x + 1,
        Direction::S => pos.x,
        Direction::W => pos.x - 1,
    };
    Pos {
        x: next_col,
        y: next_row,
    }
}

fn in_bounds(pos: &Pos, width: i32, height: i32) -> bool {
    pos.x >= 0 && pos.x < width && pos.y >= 0 && pos.y < height
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lab_map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let width = lab_map[0].len() as i32;
    let height = lab_map.len() as i32;
    let mut guard_pos = lab_map
        .iter()
        .find_position(|row| row.contains(&'^'))
        .map(|(row, v)| {
            (
                row as i32,
                v.iter().find_position(|c| **c == '^').unwrap().0 as i32,
            )
        })
        .map(|(row, col)| Pos { x: col, y: row })
        .unwrap();
    let mut guard_direction = Direction::N;

    while in_bounds(&guard_pos, width, height) {
        lab_map[guard_pos.y as usize][guard_pos.x as usize] = 'X';
        let next_pos = forward(guard_direction, &guard_pos);
        if in_bounds(&next_pos, width, height)
            && lab_map[next_pos.y as usize][next_pos.x as usize] == '#'
        {
            guard_direction = turn_right(guard_direction);
        } else {
            guard_pos = next_pos;
        }
    }
    let num_guarded: u32 = lab_map
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
