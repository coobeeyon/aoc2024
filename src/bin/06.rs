use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

type State = (Pos, Direction);
type LabMap = Vec<Vec<char>>;

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
        Direction::N => Direction::E,
    }
}

fn forward(state: &State) -> Pos {
    let pos = &state.0;
    let dir = &state.1;
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

fn next_state(state: &State, lab_map: &LabMap) -> State {
    let next_pos = forward(state);
    if in_bounds(&next_pos, lab_map)
        && lab_map[next_pos.y as usize][next_pos.x as usize] == '#'
    {
        (state.0.clone(), turn_right(state.1))
    } else {
        (next_pos, state.1)
    }
}

fn in_bounds(pos: &Pos, lab_map: &LabMap) -> bool {
    let width = lab_map[0].len() as i32;
    let height = lab_map.len() as i32;
    pos.x >= 0 && pos.x < width && pos.y >= 0 && pos.y < height
}

fn initial_state(lab_map: &LabMap) -> State {
    let guard_pos = lab_map
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
    let guard_direction = Direction::N;
    (guard_pos, guard_direction)
}

pub fn part_one(input: &str) -> Option<u32> {
    let lab_map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let guard_state = initial_state(&lab_map);
    let num_guarded = itertools::iterate(guard_state, |gs| next_state(gs, &lab_map))
        .take_while(|gs| in_bounds(&gs.0, &lab_map))
        .unique_by(|gs| gs.0.clone())
        .count() as u32;
    Some(num_guarded)
}

pub fn part_two(input: &str) -> Option<u32> {
    let orig_map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let width = orig_map[0].len() as i32;
    let height = orig_map.len() as i32;
    let num_loop_places = 
    // All maps with one legally added obstacle
    (0..width).cartesian_product(0..height)
        .filter_map(|(col, row)| {
            if orig_map[row as usize][col as usize] != '.' {
                None
            } else {
                let mut new_map = orig_map.clone();
                new_map[row as usize][col as usize] = '#';
                Some(new_map)
            }
        })
        // Filter out ones without loops
        .filter_map(|lab_map| {
            let guard_state = initial_state(&orig_map);
            let slow_guard =
                itertools::iterate(guard_state.clone(), |state| next_state(state, &lab_map));
            let fast_guard = itertools::iterate(next_state(&guard_state, &lab_map), |state| {
                next_state(&next_state(state, &lab_map), &lab_map)
            });
            fast_guard
                .take_while(|gs| in_bounds(&gs.0, &lab_map))
                .zip(slow_guard)
                .find(|(a, b)| a == b)
        })
        // And count them
        .count();
    Some(num_loop_places as u32)
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
        assert_eq!(result, Some(6));
    }
}
