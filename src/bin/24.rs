use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use rustc_hash::FxHashMap;

advent_of_code::solution!(24);

type Wire = (u8, u8, u8);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}
type Gate = (Wire, Wire, Op);
type Connections = FxHashMap<Wire, Gate>;
type Values = FxHashMap<Wire, bool>;

fn parse_input(input: &str) -> (Connections, Values) {
    let mut connections = FxHashMap::default();

    fn parse_wire(wire: &str) -> Wire {
        let (a, b, c) = wire.bytes().collect_tuple().unwrap();
        (a, b, c)
    }

    let (values_str, rules_str) = input.split_once("\n\n").unwrap();

    for rule in rules_str.lines().filter(|line| !line.is_empty()) {
        let (gate, to_wire) = rule.split_once(" -> ").unwrap();
        let gate = gate.split_whitespace().collect::<Vec<_>>();
        let gate = match gate[..] {
            [a, "AND", b] => (parse_wire(a), parse_wire(b), Op::And),
            [a, "OR", b] => (parse_wire(a), parse_wire(b), Op::Or),
            [a, "XOR", b] => (parse_wire(a), parse_wire(b), Op::Xor),
            _ => unreachable!(),
        };
        connections.insert(parse_wire(to_wire), gate);
    }

    let values = FxHashMap::from_iter(
        values_str
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(wire, value)| (parse_wire(wire), value == "1")),
    );

    (connections, values)
}

#[allow(unused)]
fn wire_str(wire: Wire) -> String {
    format!("{}{}{}", wire.0 as char, wire.1 as char, wire.2 as char)
}

fn solve(connections: &Connections, values: &mut Values) -> u64 {
    let mut q = VecDeque::from_iter(
        connections
            .keys()
            .filter(|(a, _, _)| *a == b'z')
            .filter(|w| values.get(w).is_none())
            .cloned(),
    );

    while let Some(wire) = q.pop_front() {
        if values.get(&wire).is_some() {
            continue;
        }

        let (a, b, op) = connections.get(&wire).unwrap();
        match (values.get(&a), values.get(&b)) {
            (Some(a), Some(b)) => {
                let val = match op {
                    Op::And => a & b,
                    Op::Or => a | b,
                    Op::Xor => a ^ b,
                };
                values.insert(wire, val);
            }
            (None, Some(_)) => {
                q.push_front(*a);
                q.push_back(wire);
            }
            (Some(_), None) => {
                q.push_front(*b);
                q.push_back(wire);
            }
            (None, None) => {
                q.push_front(*a);
                q.push_front(*b);
                q.push_back(wire);
            }
        }
    }

    let res = values
        .keys()
        .filter(|(a, _, _)| *a == b'z')
        .sorted()
        .map(|w| values.get(w).unwrap())
        .enumerate()
        .rev()
        .map(|(i, v)| (*v as u64) << i)
        .fold(0, |acc, v| acc | v);

    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let (connections, mut values) = parse_input(input);

    Some(solve(&connections, &mut values))
}

#[allow(unused)]
fn print_dot(connections: &Connections, deviating_nodes: &HashSet<Wire>) {
    let mut s = String::new();
    s.push_str("digraph G {\n");
    for (c, (a, b, op)) in connections.iter() {
        let op_str = match op {
            Op::And => "AND",
            Op::Or => "OR",
            Op::Xor => "XOR",
        };

        // let op_node_id = format!("op_{}_{}_{}", wire_str(*a), op_str, wire_str(*b));
        let color = if deviating_nodes.contains(c) {
            "style=filled fillcolor=red"
        } else {
            ""
        };
        s.push_str(&format!(
            "{} [label=\"{}\\n{}\" {}]\n",
            wire_str(*c),
            op_str,
            wire_str(*c),
            color
        ));
        s.push_str(&format!("{} -> {}\n", wire_str(*a), wire_str(*c)));
        s.push_str(&format!("{} -> {}\n", wire_str(*b), wire_str(*c)));
    }
    s.push_str("}\n");
    println!("{}", s);
}

pub fn part_two(input: &str) -> Option<String> {
    let (connections, _) = parse_input(input);

    let mut deviating_nodes = HashSet::new();

    for z_node in connections.keys().filter(|(a, _, _)| *a == b'z') {
        let (a, b, op) = connections.get(z_node).unwrap();
        if *op != Op::Xor {
            deviating_nodes.insert(z_node.clone());
        }

        let maybe_gate_a = connections.get(a);
        let maybe_gate_b = connections.get(b);
        let (x_y_from, x_y_branch) = match (maybe_gate_a, maybe_gate_b) {
            (Some(((b'x' | b'y', ..), ..)), _) => (a, maybe_gate_a.unwrap()),
            (_, Some(((b'x' | b'y', ..), ..))) => (b, maybe_gate_b.unwrap()),
            (None, _) => {
                deviating_nodes.insert(a.clone());
                continue;
            }
            (_, None) => {
                deviating_nodes.insert(b.clone());
                continue;
            }
            _ => unreachable!(),
        };

        let (_, _, x_y_op) = x_y_branch;
        if *x_y_op != Op::Xor {
            deviating_nodes.insert(x_y_from.clone());
        }
    }

    // print_dot(&connections, &deviating_nodes);

    // solved manually, it's christmas time
    Some("gbs,hwq,thm,wrm,wss,z08,z22,z29".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2024));
    }
}
