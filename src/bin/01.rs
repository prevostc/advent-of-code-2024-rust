advent_of_code::solution!(1);
use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut l1 = Vec::with_capacity(1000);
    let mut l2 = Vec::with_capacity(1000);
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let n1 = parts.next().unwrap().parse::<u32>().unwrap();
        let n2 = parts.next().unwrap().parse::<u32>().unwrap();
        l1.push(n1);
        l2.push(n2);
    }
    (l1, l2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut l1, mut l2) = parse_input(input);

    l1.sort();
    l2.sort();

    let mut total = 0;
    for (n1, n2) in l1.iter().zip(l2.iter()) {
        total += n1.abs_diff(*n2);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (l1, l2) = parse_input(input);
    let freq = l2.iter().counts();
    let mut total = 0;
    for n1 in l1 {
        if let Some(&n2) = freq.get(&n1) {
            total += n1 * n2 as u32;
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
