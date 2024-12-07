use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Clone, PartialEq, Eq, Debug)]
struct Calculation {
    result: u64,
    args: Vec<u64>,
}
impl Calculation {
    fn from_input_line(line: &str) -> Calculation {
        let (result, args) = line.split(':').collect_tuple().unwrap();
        let result: u64 = result.parse().unwrap();
        let args = args
            .split_ascii_whitespace()
            .map(|arg| arg.parse::<u64>().unwrap())
            .collect();
        Calculation { result, args }
    }
    fn from_input(input: &str) -> Vec<Calculation> {
        input.lines().map(Calculation::from_input_line).collect()
    }

    fn num_operations(&self) -> usize {
        self.args.len() - 1
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operation {
    Multiply,
    Add,
}

fn operator_combinations(n: usize) -> impl Iterator<Item = Vec<Operation>> {
    (0..2usize.pow(n as u32)).map(move |i| {
        format!("{i:0width$b}", width = n)
            .chars()
            .map(|c| match c {
                '0' => Operation::Add,
                '1' => Operation::Multiply,
                _ => panic!(),
            })
            .collect_vec()
    })
}

fn eval(calc: &Calculation, ops: &[Operation]) -> u64 {
    ops.iter()
        .zip(calc.args.iter().dropping(1))
        .fold(calc.args[0], |a, (op, arg)| match op {
            Operation::Add => a + arg,
            Operation::Multiply => a * arg,
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let puzzle = Calculation::from_input(input);
    let solution = puzzle
        .iter()
        .filter_map(|calc| {
            operator_combinations(calc.num_operations())
                .map(|ops| eval(calc, &ops))
                .find(|v| *v == calc.result)
        })
        .sum();
    Some(solution)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
