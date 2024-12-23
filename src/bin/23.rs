use itertools::Itertools;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(23);

type Node = (u8, u8);
type Connections = FxHashMap<Node, FxHashSet<Node>>;

fn parse_input(input: &str) -> Connections {
    let mut connections = FxHashMap::default();

    // nice technique from https://github.com/ndunnett/aoc/blob/main/rust/2024/src/bin/day23.rs
    for (a0, a1, _, b0, b1, _) in input.bytes().tuples() {
        connections
            .entry((a0, a1))
            .or_insert(FxHashSet::default())
            .insert((b0, b1));

        connections
            .entry((b0, b1))
            .or_insert(FxHashSet::default())
            .insert((a0, a1));
    }

    connections
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = parse_input(input);

    type Island = (Node, Node, Node);

    let all_islands = graph
        .iter()
        .filter(|(l1, _)| l1.0 == b't')
        .flat_map(|(a, adj_a)| {
            adj_a.iter().flat_map(|b| {
                adj_a.intersection(&graph[b]).map(|c| {
                    let mut island = [*a, *b, *c];
                    island.sort();
                    island.into_iter().next_tuple::<Island>().unwrap()
                })
            })
        });

    let islands = FxHashSet::from_iter(all_islands);
    Some(islands.len() as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let graph = parse_input(input);
    let mut cliques = graph
        .keys()
        .map(|&a| FxHashSet::from_iter([a]))
        .collect::<Vec<_>>();

    cliques.par_iter_mut().for_each(|clique| {
        graph.keys().for_each(|a| {
            if clique.iter().all(|b| graph[a].contains(b)) {
                clique.insert(*a);
            }
        });
    });

    let max_clique = cliques.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap();
    let password = max_clique
        .iter()
        .sorted()
        .map(|node| format!("{}{}", node.0 as char, node.1 as char))
        .join(",");

    Some(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
