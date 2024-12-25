use itertools::Itertools;

advent_of_code::solution!(25);

const WIDTH: usize = 5;
const HEIGHT: usize = 6;

fn parse_input(input: &str) -> (Vec<[usize; WIDTH]>, Vec<[usize; WIDTH]>) {
    let mut keys = Vec::new();
    let mut padlocks = Vec::new();

    for block in input.split("\n\n") {
        let mut data = [0; WIDTH];
        for line in block.lines().filter(|l| !l.is_empty()) {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    data[i] += 1;
                }
            }
        }
        let is_padlock = block.lines().next().unwrap() == "#".repeat(WIDTH);

        if is_padlock {
            padlocks.push(data);
        } else {
            keys.push(data);
        }
    }

    (keys, padlocks)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (keys, padlocks) = parse_input(input);

    let overlaps = keys
        .iter()
        .cartesian_product(padlocks.iter())
        .filter(|(&key, &padlock)| {
            let overlap = key
                .iter()
                .zip(padlock.iter())
                .any(|(k, p)| *k + *p > (HEIGHT + 1));

            !overlap
        })
        .count();

    Some(overlaps)
}

pub fn part_two(input: &str) -> Option<usize> {
    part_one(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
