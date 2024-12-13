use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy)]
struct Claw {
    button_a: [i128; 2],
    a_cost: i128,
    button_b: [i128; 2],
    b_cost: i128,
    prize: [i128; 2],
}

fn parse_xy(s: &str, ch: char) -> [i128; 2] {
    let (x, y) = s.split_once(", ").unwrap();
    [
        x.split(ch).nth(1).unwrap().parse().unwrap(),
        y.split(ch).nth(1).unwrap().parse().unwrap(),
    ]
}

fn parse_input(input: &str) -> Vec<Claw> {
    input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            let lines = chunk.collect::<Vec<_>>();
            let button_a = lines[0].split("Button A: ").nth(1).unwrap();
            let button_b = lines[1].split("Button B: ").nth(1).unwrap();
            let prize = lines[2].split("Prize: ").nth(1).unwrap();
            Claw {
                button_a: parse_xy(button_a, '+'),
                a_cost: 3,
                button_b: parse_xy(button_b, '+'),
                b_cost: 1,
                prize: parse_xy(prize, '='),
            }
        })
        .collect()
}

fn solve_cost<const PART: u8>(claw: &Claw) -> Option<i128> {
    let det = claw.button_a[0] * claw.button_b[1] - claw.button_a[1] * claw.button_b[0];
    if det == 0 {
        return None;
    }
    let (cx, cy) = if PART == 1 {
        (claw.prize[0], claw.prize[1])
    } else {
        (
            claw.prize[0] + 10000000000000,
            claw.prize[1] + 10000000000000,
        )
    };

    let a_pressed = cx * claw.button_b[1] - cy * claw.button_b[0];
    if a_pressed % det != 0 {
        return None;
    }
    let b_pressed = cy * claw.button_a[0] - cx * claw.button_a[1];
    if b_pressed % det != 0 {
        return None;
    }

    let a_cost = claw.a_cost * (a_pressed / det);
    let b_cost = claw.b_cost * (b_pressed / det);
    Some(a_cost + b_cost)
}

pub fn part_one(input: &str) -> Option<i128> {
    let mut cost = 0;
    for claw in parse_input(input) {
        let claw_cost = solve_cost::<1>(&claw);
        cost += claw_cost.unwrap_or(0);
    }
    Some(cost)
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut cost = 0;
    for claw in parse_input(input) {
        let claw_cost = solve_cost::<2>(&claw);
        cost += claw_cost.unwrap_or(0);
    }
    Some(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(459236326669));
    }
}
