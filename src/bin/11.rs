use memoize::memoize;
use num::traits::Euclid;

advent_of_code::solution!(11);

type Stone = u64;
type Step = u8;

#[memoize]
fn blink_rec(_cache_buster: u64, stone: Stone, times: Step) -> u64 {
    if times == 0 {
        return 1;
    }

    if stone == 0 {
        return blink_rec(_cache_buster, 1, times - 1);
    }

    let mut digit_count = 0;
    let mut temp = stone;
    while temp > 0 {
        digit_count += 1;
        temp /= 10;
    }

    if digit_count % 2 == 0 {
        let divisor: u64 = 10_u64.pow(digit_count / 2);
        let (upper, lower) = stone.div_rem_euclid(&divisor);
        return blink_rec(_cache_buster, lower, times - 1)
            + blink_rec(_cache_buster, upper, times - 1);
    }

    return blink_rec(_cache_buster, stone * 2024, times - 1);
}

static mut GLOBAL_SEQ: u64 = 0;

fn solve(input: &str, times: Step) -> u64 {
    let cache_buster = unsafe { GLOBAL_SEQ };
    unsafe {
        GLOBAL_SEQ += 1;
    }
    let stones = input
        .split_whitespace()
        .filter(|p| !p.is_empty())
        .map(|s| s.parse::<u64>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();

    let count = stones
        .iter() // so fast that parallel is slower
        .map(|start_stone| blink_rec(cache_buster, *start_stone, times))
        .sum::<u64>();

    return count;
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}