use mygrid::{
    direction::{Direction, ORTHOGONAL},
    grid::Grid,
    point::Point,
};
use rayon::iter::ParallelIterator;

advent_of_code::solution!(20);

#[inline]
fn parse_input(input: &str) -> (Grid<char>, Point, Point) {
    let mut grid = Grid::new_char_grid_from_str(input);
    let start = grid.find_position_of(&'S').unwrap();
    let end = grid.find_position_of(&'E').unwrap();
    grid[start] = '.';
    grid[end] = '.';
    (grid, start, end)
}

#[inline]
fn dijkstra(grid: &Grid<char>, start: Point, end: Point) -> Grid<i64> {
    let mut cost = Grid::new(grid.width, grid.height, i64::MAX);
    let mut state = Some((0, start));

    while let Some((dst, pos)) = state {
        cost[pos] = dst;

        if pos == end {
            break;
        }

        let next_cost = dst + 1;

        let next_pos = ORTHOGONAL
            .iter()
            .map(|&d| pos + d)
            .filter(|p| grid.is_in_bounds(*p))
            .filter(|&p| grid[p] != '#')
            .filter(|&p| cost[p] > next_cost)
            .next();

        match next_pos {
            Some(p) => state = Some((next_cost, p)),
            None => panic!("No next position"),
        }
    }

    cost
}

fn solve<const CHEAT_MOVES: isize>(input: &str, min_gain: i64) -> Option<i64> {
    assert!(CHEAT_MOVES > 1);
    assert!(min_gain > 0);

    let (grid, start, end) = parse_input(input);
    let cost = dijkstra(&grid, start, end);

    let diamond_diff = (0..(CHEAT_MOVES * 2 + 1))
        .into_iter()
        .flat_map(move |i| {
            let line = -CHEAT_MOVES + i;
            (0..(CHEAT_MOVES * 2 + 1)).into_iter().map(move |j: isize| {
                let col = -CHEAT_MOVES + j;
                let diff = Direction::new(line, col);
                let moves = (diff.vertical).abs() + (diff.horizontal).abs();
                (diff, moves as i64)
            })
        })
        .filter(|&(_, moves)| moves <= CHEAT_MOVES as i64)
        .collect::<Vec<_>>();

    let count = cost
        // makes p1 slightly slower, but p2 much faster
        .par_iter_item_and_position()
        .filter(|&(_, c)| *c != i64::MAX)
        .map(|(start_pos, &start_cost)| {
            let count = diamond_diff
                .iter()
                .map(|&(diff, moves)| (start_pos + diff, moves))
                .filter(|&(pos, _)| cost.is_in_bounds(pos))
                .filter(|&(pos, _)| cost[pos] != i64::MAX)
                .filter(|&(pos, moves)| cost[pos] - (start_cost + moves as i64) >= min_gain)
                .count() as i64;

            count
        })
        .sum();
    Some(count)
}

pub fn part_one(input: &str) -> Option<i64> {
    solve::<2>(input, 100)
}

pub fn part_two(input: &str) -> Option<i64> {
    solve::<20>(input, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_0() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = solve::<2>(&input, 1);
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_one_64() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = solve::<2>(&input, 64);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_40() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = solve::<2>(&input, 40);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_20() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = solve::<2>(&input, 20);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two_50() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = solve::<20>(&input, 50);
        assert_eq!(result, Some(285));
    }

    #[test]
    fn test_part_two_64() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = solve::<20>(&input, 64);
        assert_eq!(result, Some(86));
    }

    #[test]
    fn test_part_two_70() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = solve::<20>(&input, 70);
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two_74() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = solve::<20>(&input, 74);
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two_76() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = solve::<20>(&input, 76);
        assert_eq!(result, Some(3));
    }
}
