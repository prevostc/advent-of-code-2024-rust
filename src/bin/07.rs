use heapless::Vec as HeaplessVec;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Concat,
}

#[derive(Debug)]
struct EquationData {
    test_value: u64,
    values: Vec<u64>,
}

impl EquationData {
    fn parse(line: &str) -> Self {
        let (left, right) = line.split_once(": ").unwrap();
        let test_value = left.parse().unwrap();
        let values = right.split(" ").map(|s| s.parse().unwrap()).collect();
        Self { test_value, values }
    }

    #[inline]
    fn has_valid_ops_combination(&self, available_ops: &[Op]) -> bool {
        let needed_ops = self.values.len() - 1;
        if needed_ops == 0 {
            return self.values[0] == self.test_value;
        }

        const OPS_SIZE: usize = 16;
        const STACK_SIZE: usize = 32;

        let mut stack = HeaplessVec::<_, STACK_SIZE>::new();
        stack
            .push((self.values[0], HeaplessVec::<_, OPS_SIZE>::new()))
            .unwrap();

        while let Some((current_value, mut ops)) = stack.pop() {
            if ops.len() == needed_ops {
                if current_value == self.test_value {
                    return true;
                }
                continue;
            }

            let next_idx = ops.len() + 1;
            let next_value = self.values[next_idx];

            // Try each available operation
            for &op in available_ops {
                let new_value = match op {
                    Op::Add => current_value + next_value,
                    Op::Mul => current_value * next_value,
                    Op::Concat => {
                        let mut digit_count = 0;
                        let mut n = next_value;
                        while n > 0 {
                            digit_count += 1;
                            n /= 10;
                        }
                        current_value * 10_u64.pow(digit_count) + next_value
                    }
                };

                // Skip if we've already exceeded the target
                if new_value > self.test_value {
                    continue;
                }

                // Create new ops vector and push to stack
                ops.push(op).unwrap();
                stack.push((new_value, ops.clone())).unwrap();
                ops.pop();
            }
        }

        false
    }
}

fn solve(input: &str, ops: &[Op]) -> Option<u64> {
    let lines: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(EquationData::parse)
        .collect();
    let sum = lines
        .par_iter()
        .filter(|eq| eq.has_valid_ops_combination(ops))
        .map(|eq| eq.test_value)
        .sum();
    Some(sum)
}

pub fn part_one(input: &str) -> Option<u64> {
    const ALL_OPS: [Op; 2] = [Op::Add, Op::Mul];
    solve(input, &ALL_OPS)
}

pub fn part_two(input: &str) -> Option<u64> {
    const ALL_OPS: [Op; 3] = [Op::Add, Op::Mul, Op::Concat];
    solve(input, &ALL_OPS)
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
