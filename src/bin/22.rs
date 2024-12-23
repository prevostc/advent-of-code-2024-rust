use bitvec::bitvec;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(22);

type Secret = u32;

// println!("{:#032b}", 16777216);
// println!("{:#032b}", 0b000001000000000000000000000000);
// println!("{:#032b}", (1 << 24) - 1);
// println!("{:#032b}", 2048); -> 2**11
const MASK: Secret = (1 << 24) - 1;

#[inline]
fn next_secret(n: Secret) -> Secret {
    let n = (n ^ n << 6) & MASK;
    let n = (n ^ n >> 5) & MASK;
    let n = (n ^ n << 11) & MASK;
    n
}

#[inline]
fn get_secret(s: Secret, nth: usize) -> Secret {
    let mut secret = s;
    for _ in 0..nth {
        secret = next_secret(secret)
    }
    secret
}

#[inline]
fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Secret> + 'a {
    input
        .lines()
        .filter(|&l| !l.is_empty())
        .map(|l| l.parse::<Secret>().unwrap())
}

pub fn part_one(input: &str) -> Option<u64> {
    let seeds = parse_input(input).collect::<Vec<_>>();
    let res = seeds
        .par_iter()
        .map(|&s| get_secret(s, 2000))
        .map(|s| s as u64)
        .sum();
    Some(res)
}

const SEQ_PART_SIZE: usize = 5;
const SEQ_MASK_4: u32 = (1 << (SEQ_PART_SIZE * 4)) - 1;
const SEQ_MASK_1: u32 = (1 << SEQ_PART_SIZE) - 1;

type Seq = u32;
type SeqDiff = i8;

#[inline]
fn push_seq(seq: Seq, diff: SeqDiff) -> Seq {
    // we are working mod 10 so + 10 is fine
    ((seq << SEQ_PART_SIZE) | (diff as Seq & SEQ_MASK_1)) & SEQ_MASK_4
}

pub fn part_two(input: &str) -> Option<i32> {
    let seeds = parse_input(input).collect::<Vec<_>>();

    let mut seq_bananas = vec![0; 1 << (SEQ_PART_SIZE * 4)];

    seeds.iter().for_each(|seed| {
        let mut seen = bitvec![0; 1 << (SEQ_PART_SIZE * 4)];

        let mut prev_price = 0;
        let mut secret = *seed;
        let mut seq = 0;

        for i in 0..2000 {
            secret = next_secret(secret);
            let new_price = (secret % 10) as i8;
            seq = push_seq(seq, new_price - prev_price);
            prev_price = new_price;

            if i < 4 {
                continue;
            }
            if seen[seq as usize] {
                continue;
            }

            seen.set(seq as usize, true);
            seq_bananas[seq as usize] += new_price as i32;
        }
    });

    let max_bananas = seq_bananas.into_iter().max().unwrap();
    Some(max_bananas)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_secrets() {
        let seed = 123;

        [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ]
        .iter()
        .enumerate()
        .for_each(|(i, &expected)| assert_eq!(get_secret(seed, i + 1), expected));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two_0() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(23));
    }
}
