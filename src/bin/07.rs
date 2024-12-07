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
        // Stack will store (current_target, remaining_numbers_index);
        let mut stack = vec![(self.test_value, self.values.len() - 1)];

        while let Some((current_target, idx)) = stack.pop() {
            // Base case - if we're at the first number
            if idx == 0 {
                if self.values[0] == current_target {
                    return true;
                }
                continue;
            }

            let n = self.values[idx];

            for &op in available_ops {
                match op {
                    Op::Add => {
                        if current_target >= n {
                            stack.push((current_target - n, idx - 1));
                        }
                    }
                    Op::Mul => {
                        if current_target % n == 0 {
                            stack.push((current_target / n, idx - 1));
                        }
                    }
                    Op::Concat => {
                        let mut digit_count = 0;
                        let mut temp = n;
                        while temp > 0 {
                            digit_count += 1;
                            temp /= 10;
                        }
                        let divisor = 10_u64.pow(digit_count);

                        if current_target % divisor == n {
                            stack.push((current_target / divisor, idx - 1));
                        }
                    }
                }
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
