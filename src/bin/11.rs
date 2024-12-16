use rustc_hash::FxHashMap;

advent_of_code::solution!(11);

type Stone = u64;
type Step = u8;

const MAX_STONE_COUNT: usize = 5000;
const MAX_DIGIT_COUNT: usize = 40;

// take advantage of the fact that there is a limited number of stone numbers
// found by @maneatingape: https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2024/day11.rs#L3
struct StoneIdxPool {
    count: usize,
    stone_num_to_stone_idx: FxHashMap<Stone, usize>,
    stone_idx_to_stone_num: [Stone; MAX_STONE_COUNT],
}
impl StoneIdxPool {
    #[inline]
    fn new() -> Self {
        Self {
            count: 0,
            stone_num_to_stone_idx: FxHashMap::default(),
            stone_idx_to_stone_num: [0; MAX_STONE_COUNT],
        }
    }

    #[inline]
    fn get_idx(&mut self, stone_num: Stone) -> usize {
        if let Some(idx) = self.stone_num_to_stone_idx.get(&stone_num) {
            return *idx;
        }

        let idx = self.count;
        self.stone_idx_to_stone_num[idx] = stone_num;
        self.stone_num_to_stone_idx.insert(stone_num, idx);
        self.count += 1;
        idx
    }

    #[inline]
    fn get_stone_num(&self, idx: usize) -> Stone {
        self.stone_idx_to_stone_num[idx]
    }
}

fn solve(input: &str, times: Step) -> u64 {
    let mut idx_pool = StoneIdxPool::new();

    let mut divisors = [0; MAX_DIGIT_COUNT];
    for i in 1..MAX_DIGIT_COUNT {
        divisors[i] = 10_u64.pow(i as u32 / 2);
    }

    let mut stone_count: [Stone; MAX_STONE_COUNT] = [0; MAX_STONE_COUNT];
    input
        .split_whitespace()
        .filter(|p| !p.is_empty())
        .map(|s| s.parse::<Stone>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .for_each(|stone_num| {
            stone_count[idx_pool.get_idx(stone_num)] += 1;
        });

    for _ in 0..times {
        let mut new_stone_count: [Stone; MAX_STONE_COUNT] = [0; MAX_STONE_COUNT];
        stone_count
            .iter()
            .enumerate()
            .filter(|(_, &count)| count > 0)
            .for_each(|(stone_idx, &count)| {
                let stone_num = idx_pool.get_stone_num(stone_idx);
                if stone_num == 0 {
                    new_stone_count[idx_pool.get_idx(1)] += count;
                    return;
                }

                // this is a bit faster than ilog10
                let mut digit_count = 0;
                let mut temp = stone_num;
                while temp > 0 {
                    digit_count += 1;
                    temp /= 10;
                }

                if digit_count % 2 == 0 {
                    let divisor = divisors[digit_count];
                    new_stone_count[idx_pool.get_idx(stone_num / divisor)] += count;
                    new_stone_count[idx_pool.get_idx(stone_num % divisor)] += count;

                    return;
                }

                new_stone_count[idx_pool.get_idx(stone_num * 2024)] += count;
            });

        stone_count = new_stone_count;
    }

    stone_count.iter().sum()
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
