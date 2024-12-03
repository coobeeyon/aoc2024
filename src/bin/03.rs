advent_of_code::solution!(3);
use regex::Regex;

pub fn string_muls(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let a: u32 = cap[1].parse().unwrap();
            let b: u32 = cap[2].parse().unwrap();
            a * b
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(string_muls(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let full_input = (input.to_owned() + "do()").replace('\n', "");
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    Some(string_muls(&re.replace_all(&full_input, "")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
