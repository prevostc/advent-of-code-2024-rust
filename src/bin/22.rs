use memoize::memoize;
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

#[memoize]
fn gen_all_seq() -> Vec<u32> {
    (-9..9)
        .flat_map(|a| {
            (-9..9).flat_map(move |b| {
                (-9..9).flat_map(move |c| {
                    (-9..9).map(move |d| {
                        (diff_to_seq_part(a) << 24)
                            | (diff_to_seq_part(b) << 16)
                            | (diff_to_seq_part(c) << 8)
                            | diff_to_seq_part(d)
                    })
                })
            })
        })
        .collect()
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

type Seq = u32;
type SeqDiff = i8;

#[inline]
fn diff_to_seq_part(diff: SeqDiff) -> Seq {
    (diff as Seq) & 0xFF
}

#[inline]
fn push_seq(seq: Seq, diff: SeqDiff) -> u32 {
    // we are working mod 10 so + 10 is fine
    (seq << 8) | diff_to_seq_part(diff)
}

pub fn part_two(input: &str) -> Option<i32> {
    let seeds = parse_input(input).collect::<Vec<_>>();
    let seeds_prices = seeds
        .par_iter()
        .map(|seed| {
            let mut prices = [(0_i8, 0_u32); 2000];

            let mut prev_price = 0;
            let mut secret = *seed;
            let mut seq = 0;
            for i in 0..2000 {
                secret = next_secret(secret);
                let new_price = (secret % 10) as i8;
                seq = push_seq(seq, new_price - prev_price);
                prices[i] = (new_price, seq);
                prev_price = new_price;
            }
            prices
        })
        .collect::<Vec<_>>();

    let seq_to_test = gen_all_seq();

    let max_bananas = seq_to_test
        .par_iter()
        .map(|&target_seq| {
            let bananas = seeds_prices
                .iter()
                .map(|&seed_data| {
                    let bananas = seed_data
                        .iter()
                        .skip(4)
                        .find(|&(_, seq)| *seq == target_seq)
                        .map(|(p, _)| *p)
                        .unwrap_or(0);

                    bananas as i32
                })
                .sum();

            bananas
        })
        .max();

    Some(max_bananas.unwrap())
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
