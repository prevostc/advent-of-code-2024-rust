use heapless::Vec as HeaplessVec;
advent_of_code::solution!(2);

const MAX_LEVELS: usize = 20;

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = HeaplessVec<u32, MAX_LEVELS>> + 'a {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<HeaplessVec<_, MAX_LEVELS>>()
    })
}

#[inline]
fn is_safe(level: &[u32]) -> bool {
    let increasing = level.windows(2).all(|w| w[0] <= w[1]);
    let decreasing = level.windows(2).all(|w| w[0] >= w[1]);
    let diff = level
        .windows(2)
        .all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3);
    (increasing || decreasing) && diff
}

pub fn part_one(input: &str) -> Option<u32> {
    let c = parse_input(input).filter(|level| is_safe(level)).count();
    Some(c as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let c = parse_input(input)
        .filter(|level| {
            (0..level.len()).any(|i| {
                let mut l = level.clone();
                l.remove(i);
                is_safe(&l)
            })
        })
        .count();
    Some(c as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
