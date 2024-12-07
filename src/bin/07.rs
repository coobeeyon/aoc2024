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

trait Operation: Sized {
    fn operator_combinations(n: usize) -> impl Iterator<Item = Vec<Self>>;
    fn eval(calc: &Calculation, ops: &[Self]) -> u64;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PartOneOperation {
    Multiply,
    Add,
}

impl Operation for PartOneOperation {
    fn operator_combinations(n: usize) -> impl Iterator<Item = Vec<PartOneOperation>> {
        (0..2usize.pow(n as u32)).map(move |i| {
            format!("{i:0width$b}", width = n)
                .chars()
                .map(|c| match c {
                    '0' => PartOneOperation::Add,
                    '1' => PartOneOperation::Multiply,
                    _ => panic!(),
                })
                .collect_vec()
        })
    }

    fn eval(calc: &Calculation, ops: &[PartOneOperation]) -> u64 {
        ops.iter()
            .zip(calc.args.iter().dropping(1))
            .fold(calc.args[0], |a, (op, arg)| match op {
                PartOneOperation::Add => a + arg,
                PartOneOperation::Multiply => a * arg,
            })
    }
}

fn solve_puzzle<Op>(input: &str) -> Option<u64>
where
    Op: Operation,
{
    let puzzle = Calculation::from_input(input);
    let solution = puzzle
        .iter()
        .filter_map(|calc| {
            Op::operator_combinations(calc.num_operations())
                .map(|ops| Op::eval(calc, &ops))
                .find(|v| *v == calc.result)
        })
        .sum();
    Some(solution)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PartTwoOperation {
    Multiply,
    Add,
    Concat,
}

impl Operation for PartTwoOperation {
    fn operator_combinations(n: usize) -> impl Iterator<Item = Vec<PartTwoOperation>> {
        (0..n)
            .map(|_| {
                [
                    PartTwoOperation::Add,
                    PartTwoOperation::Multiply,
                    PartTwoOperation::Concat,
                ]
                .into_iter()
            })
            .multi_cartesian_product()
    }

    fn eval(calc: &Calculation, ops: &[PartTwoOperation]) -> u64 {
        ops.iter()
            .zip(calc.args.iter().dropping(1))
            .fold(calc.args[0], |a, (op, arg)| match op {
                PartTwoOperation::Add => a + arg,
                PartTwoOperation::Multiply => a * arg,
                PartTwoOperation::Concat => (a.to_string() + &arg.to_string()).parse().unwrap(),
            })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    solve_puzzle::<PartOneOperation>(input)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve_puzzle::<PartTwoOperation>(input)
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
        assert_eq!(result, Some(11387));
    }
}
