use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(19);

#[inline]
fn char_to_idx(c: char) -> usize {
    (c as usize - 'a' as usize) as usize
}

#[derive(Clone, Debug)]
struct TrieTree {
    next: [Option<Box<TrieTree>>; 26],
    max_len: usize,
    is_end: bool,
}

impl TrieTree {
    fn new() -> Self {
        Self {
            next: core::array::from_fn(|_| None),
            max_len: 0,
            is_end: false,
        }
    }

    fn from_words(words: &[&str]) -> Self {
        let mut tree = Self::new();

        let mut max_len = 0;
        for word in words {
            tree.insert(*word);
            max_len = max_len.max(word.len());
        }

        assert_eq!(tree.is_end, false);
        tree.max_len = max_len;
        tree
    }

    fn insert(&mut self, word: &str) {
        let mut node = self;

        for c in word.chars() {
            let index = char_to_idx(c);
            match node.next[index] {
                Some(ref mut child) => node = child,
                None => {
                    node.next[index] = Some(Box::new(TrieTree::new()));
                    node = node.next[index].as_mut().unwrap();
                }
            }
        }

        node.is_end = true;
        node.max_len = node.max_len.max(word.len());
    }

    #[inline]
    fn get_all_prefixes<'a>(&self, design: &'a str) -> Vec<&'a str> {
        let mut prefixes = Vec::with_capacity(10);
        let design_chars = design.chars().collect::<Vec<_>>();

        let mut q = Vec::new();
        q.push((self, 0));

        while let Some((node, design_idx)) = q.pop() {
            if node.is_end {
                prefixes.push(&design[..design_idx]);
            }
            if design_idx == design.len() {
                continue;
            }

            let index = char_to_idx(design_chars[design_idx]);
            if let Some(boxed_child) = &node.next[index] {
                q.push((boxed_child.as_ref(), design_idx + 1));
            }
        }

        prefixes
    }
}

fn is_valid_design(trie: &TrieTree, design: &str) -> bool {
    let mut q = Vec::new();
    q.push((trie, 0));

    let design_chars = design.chars().collect::<Vec<_>>();

    while let Some((node, design_idx)) = q.pop() {
        if design_idx == design.len() {
            match node.is_end {
                true => return true,
                false => continue,
            }
        }

        if node.is_end {
            q.push((trie, design_idx));
        }

        let idx = char_to_idx(design_chars[design_idx]);
        if let Some(boxed_child) = &node.next[idx] {
            q.push((boxed_child.as_ref(), design_idx + 1));
        }
    }

    false
}

fn count_valid_designs<'a>(
    cached_counts: &mut HashMap<&'a str, u64>,
    trie: &TrieTree,
    design: &'a str,
) -> u64 {
    if cached_counts.contains_key(design) {
        return *cached_counts.get(design).unwrap();
    }

    let mut count = 0;
    for prefix in trie.get_all_prefixes(design) {
        // dbg!(&design, &prefix);
        if prefix.len() == design.len() {
            count += 1;
        }
        count += count_valid_designs(cached_counts, trie, &design[prefix.len()..]);
    }

    // dbg!(&design, count);
    cached_counts.insert(design, count);
    count
}

fn parse_input(input: &str) -> (TrieTree, Vec<&str>) {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect::<Vec<_>>();
    let designs = designs.lines().collect::<Vec<_>>();
    (TrieTree::from_words(&towels), designs)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (trie, designs) = parse_input(input);

    let valid_count = designs
        .iter()
        .filter(|&design| is_valid_design(&trie, design))
        .count();

    Some(valid_count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (trie, designs) = parse_input(input);

    let valid_count = designs
        .par_iter()
        .map(|design| count_valid_designs(&mut HashMap::new(), &trie, design))
        .sum();

    Some(valid_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
