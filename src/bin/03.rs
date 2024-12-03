advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)").unwrap();
    let sum = regex
        .captures_iter(input)
        .map(|captures| {
            let x = &captures["x"].parse::<u32>().unwrap();
            let y = &captures["y"].parse::<u32>().unwrap();
            x * y
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex =
        Regex::new(r"((?P<op>do\(\)|don't\(\))|(mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)))").unwrap();
    let (_, sum) = regex
        .captures_iter(input)
        .fold((true, 0), |(on, sum), captures| {
            if let Some(op) = captures.name("op") {
                if op.as_str() == "don't()" {
                    (false, sum)
                } else if op.as_str() == "do()" {
                    (true, sum)
                } else {
                    panic!("Unknown op: {}", op.as_str());
                }
            } else if on {
                let x = &captures["x"].parse::<u32>().unwrap();
                let y = &captures["y"].parse::<u32>().unwrap();
                (true, sum + x * y)
            } else {
                (false, sum)
            }
        });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
