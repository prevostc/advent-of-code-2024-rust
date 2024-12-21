use std::sync::atomic::AtomicUsize;

use itertools::Itertools;
use memoize::{lazy_static::lazy_static, memoize};
use rustc_hash::FxHashMap;

advent_of_code::solution!(21);

lazy_static! {
    static ref SUBST: FxHashMap<(char, char), String> = {
        [
            ('^', '<', "v<A"),
            ('^', '>', "v>A"),
            ('^', 'A', ">A"),
            ('^', 'v', "vA"),
            ('^', '^', ""),
            ('<', '^', ">^A"),
            ('<', '>', ">>A"),
            ('<', 'A', ">>^A"),
            ('<', 'v', ">A"),
            ('<', '<', ""),
            ('>', '^', "<^A"),
            ('>', '<', "<<A"),
            ('>', 'A', "^A"),
            ('>', 'v', "<A"),
            ('>', '>', ""),
            ('v', '^', "^A"),
            ('v', '<', "<A"),
            ('v', '>', ">A"),
            ('v', 'A', "^>A"),
            ('v', 'v', ""),
            ('0', '1', "^<A"),
            ('0', '2', "^A"),
            ('0', '3', "^>A"),
            ('0', '4', "^<^A"),
            ('0', '5', "^^A"),
            ('0', '6', "^^>A"),
            ('0', '7', "^^^<A"),
            ('0', '8', "^^^A"),
            ('0', '9', "^^^>A"),
            ('0', 'A', ">A"),
            ('0', '0', ""),
            ('1', '0', ">vA"),
            ('1', '1', ""),
            ('1', '2', ">A"),
            ('1', '3', ">>A"),
            ('1', '4', "^A"),
            ('1', '5', "^>A"),
            ('1', '6', "^>>A"),
            ('1', '7', "^^A"),
            ('1', '8', "^^>A"),
            ('1', '9', "^^>>A"),
            ('1', 'A', ">>vA"),
            ('2', '0', "vA"),
            ('2', '1', "<A"),
            ('2', '2', ""),
            ('2', '3', ">A"),
            ('2', '4', "<^A"),
            ('2', '5', "^A"),
            ('2', '6', "^>A"),
            ('2', '7', "<^^A"),
            ('2', '8', "^^A"),
            ('2', '9', "^^>A"),
            ('2', 'A', "v>A"),
            ('3', '0', "<vA"),
            ('3', '1', "<<A"),
            ('3', '2', "<A"),
            ('3', '3', ""),
            ('3', '4', "<<^A"),
            ('3', '5', "<^A"),
            ('3', '6', "^A"),
            ('3', '7', "<<^^A"),
            ('3', '8', "<^^A"),
            ('3', '9', "^^A"),
            ('3', 'A', "vA"),
            ('4', '0', ">vvA"),
            ('4', '1', "vA"),
            ('4', '2', "v>A"),
            ('4', '3', "v>>A"),
            ('4', '4', ""),
            ('4', '5', ">A"),
            ('4', '6', ">>A"),
            ('4', '7', "^A"),
            ('4', '8', "^>A"),
            ('4', '9', "^>>A"),
            ('4', 'A', ">>vvA"),
            ('5', '0', "vvA"),
            ('5', '1', "<vA"),
            ('5', '2', "vA"),
            ('5', '3', "v>A"),
            ('5', '4', "<A"),
            ('5', '5', ""),
            ('5', '6', ">A"),
            ('5', '7', "<^A"),
            ('5', '8', "^A"),
            ('5', '9', "^>A"),
            ('5', 'A', "vv>A"),
            ('6', '0', "<vvA"),
            ('6', '1', "<<vA"),
            ('6', '2', "<vA"),
            ('6', '3', "vA"),
            ('6', '4', "<<A"),
            ('6', '5', "<A"),
            ('6', '6', ""),
            ('6', '7', "<<^A"),
            ('6', '8', "<^A"),
            ('6', '9', "^A"),
            ('6', 'A', "vvA"),
            ('7', '0', ">vvvA"),
            ('7', '1', "vvA"),
            ('7', '2', "vv>A"),
            ('7', '3', "vv>>A"),
            ('7', '4', "vA"),
            ('7', '5', "v>A"),
            ('7', '6', "v>>A"),
            ('7', '7', ""),
            ('7', '8', ">A"),
            ('7', '9', ">>A"),
            ('7', 'A', ">>vvvA"),
            ('8', '0', "vvvA"),
            ('8', '1', "<vvA"),
            ('8', '2', "vvA"),
            ('8', '3', "vv>A"),
            ('8', '4', "<vA"),
            ('8', '5', "vA"),
            ('8', '6', "v>A"),
            ('8', '7', "<A"),
            ('8', '8', ""),
            ('8', '9', ">A"),
            ('8', 'A', "vvv>A"),
            ('9', '0', "<vvvA"),
            ('9', '1', "<<vvA"),
            ('9', '2', "<vvA"),
            ('9', '3', "vvA"),
            ('9', '4', "<<vA"),
            ('9', '5', "<vA"),
            ('9', '6', "vA"),
            ('9', '7', "<<A"),
            ('9', '8', "<A"),
            ('9', '9', ""),
            ('9', 'A', "vvvA"),
            ('A', '^', "<A"),
            ('A', '<', "v<<A"),
            ('A', '>', "vA"),
            ('A', '0', "<A"),
            ('A', '1', "^<<A"),
            ('A', '2', "<^A"),
            ('A', '3', "^A"),
            ('A', '4', "^^<<A"),
            ('A', '5', "<^^A"),
            ('A', '6', "^^A"),
            ('A', '7', "^^^<<A"),
            ('A', '8', "<^^^A"),
            ('A', '9', "^^^A"),
            ('A', 'A', ""),
            ('A', 'v', "<vA"),
        ]
        .iter()
        .map(|(from, to, rules)| ((*from, *to), rules.to_string()))
        .collect()
    };
}

#[inline]
fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = (String, u64)> + 'a {
    input.lines().filter(|line| !line.is_empty()).map(|line| {
        let num = line.split_once('A').unwrap().0.parse::<u64>().unwrap();
        (line.to_string(), num)
    })
}

#[memoize]
fn get_length(_cb: usize, sequence: String, depth: usize) -> u64 {
    if sequence.is_empty() {
        return 1;
    }
    if depth == 0 {
        return sequence.len() as u64;
    }

    std::iter::once('A')
        .chain(sequence.chars())
        .tuple_windows()
        .map(|pair| SUBST.get(&pair).expect("Invalid move"))
        .map(|next| get_length(_cb, next.to_owned(), depth - 1))
        .sum()
}

// make benchmarks relevant
static CACHE_BUSTER: AtomicUsize = AtomicUsize::new(0);

#[inline]
fn solve(input: &str, depth: usize) -> u64 {
    let cache_buster = CACHE_BUSTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let codes = parse_input(input);
    codes
        .map(|(seq, num)| {
            let len = get_length(cache_buster, seq, depth);
            let complexity = len * num;
            complexity
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 3))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 26))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
