use rustc_hash::FxHashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(11);

type Stone = u64;
type Step = u8;
type MemoKey = (Step, Stone);
type Memo = FxHashMap<MemoKey, u64>;

fn blink_rec(memo: &mut Memo, stone: Stone, times: Step) -> u64 {
    if times == 0 {
        return 1;
    }
    if let Some(count) = memo.get(&(times, stone)) {
        return *count;
    }

    let res = {
        let mut digit_count = 0;
        let mut temp = stone;
        while temp > 0 {
            digit_count += 1;
            temp /= 10;
        }

        if stone == 0 {
            blink_rec(memo, 1, times - 1)
        } else if digit_count % 2 == 0 {
            let divisor = 10_u64.pow(digit_count / 2);
            blink_rec(memo, stone / divisor, times - 1)
                + blink_rec(memo, stone % divisor, times - 1)
        } else {
            blink_rec(memo, stone * 2024, times - 1)
        }
    };

    memo.insert((times, stone), res);
    res
}

fn parse_input(input: &str) -> Vec<Stone> {
    input
        .split_whitespace()
        .filter(|p| !p.is_empty())
        .map(|s| s.parse::<u64>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let times = 25;
    let stones = parse_input(input);
    let mut memo = FxHashMap::with_capacity_and_hasher(100000, Default::default());
    let count = stones
        .iter()
        .map(|start_stone| blink_rec(&mut memo, *start_stone, times))
        .sum::<u64>();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let times = 75;
    let stones = parse_input(input);
    let count = stones
        .par_iter()
        .map(|start_stone| {
            let mut memo = FxHashMap::with_capacity_and_hasher(100000, Default::default());
            blink_rec(&mut memo, *start_stone, times)
        })
        .sum::<u64>();

    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one("125");
    //     assert_eq!(result, Some(55312));
    // }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
