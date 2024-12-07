advent_of_code::solution!(7);
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Concat,
}

type OpVec = Vec<Op>;
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

    fn eval(&self, ops: &OpVec) -> u64 {
        let mut res = 0;
        let mut op = Op::Add;
        for i in 0..self.values.len() {
            let v = self.values[i];
            res = match op {
                Op::Add => res + v,
                Op::Mul => res * v,
                Op::Concat => {
                    let char_count = v.to_string().len() as u32;
                    res * 10_u64.pow(char_count) + v
                }
            };
            if i < ops.len() {
                op = ops[i];
            }
        }
        res
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    const ALL_OPS: [Op; 2] = [Op::Add, Op::Mul];
    let sum = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| EquationData::parse(l))
        .filter(|eq| {
            for ops in std::iter::repeat_n(ALL_OPS, eq.values.len() - 1).multi_cartesian_product() {
                if eq.eval(&ops) == eq.test_value {
                    return true;
                }
            }
            false
        })
        .map(|eq| eq.test_value)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    const ALL_OPS: [Op; 3] = [Op::Add, Op::Mul, Op::Concat];
    let sum = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| EquationData::parse(l))
        .filter(|eq| {
            for ops in std::iter::repeat_n(ALL_OPS, eq.values.len() - 1).multi_cartesian_product() {
                if eq.eval(&ops) == eq.test_value {
                    return true;
                }
            }
            false
        })
        .map(|eq| eq.test_value)
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let eq = EquationData {
            test_value: 10,
            values: vec![1, 2, 3],
        };
        assert_eq!(eq.eval(&vec![Op::Add, Op::Add]), 6);
        assert_eq!(eq.eval(&vec![Op::Add, Op::Mul]), 9);
        assert_eq!(eq.eval(&vec![Op::Mul, Op::Add]), 5);
        assert_eq!(eq.eval(&vec![Op::Mul, Op::Mul]), 6);
    }

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
