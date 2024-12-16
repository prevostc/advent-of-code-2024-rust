use std::collections::VecDeque;

use mygrid::{
    direction::{Direction, DOWN, LEFT, RIGHT, UP},
    grid::Grid,
    point::Point,
};

advent_of_code::solution!(16);

#[inline]
fn dir_to_index(dir: Direction) -> usize {
    match dir {
        RIGHT => 0,
        DOWN => 1,
        LEFT => 2,
        UP => 3,
        _ => unreachable!(),
    }
}

#[inline]
fn index_to_dir(idx: usize) -> Direction {
    match idx {
        0 => RIGHT,
        1 => DOWN,
        2 => LEFT,
        3 => UP,
        _ => unreachable!(),
    }
}

#[inline]
fn build_best_cost_grid(grid: &Grid<char>, start: Point, target_pos: Point) -> Grid<[i64; 4]> {
    let mut best_cost_grid = Grid::new(grid.width, grid.height, [i64::MAX; 4]);
    let mut q = VecDeque::new();
    q.push_back((start, RIGHT, 0));

    let mut best_target_cost = i64::MAX;
    while let Some((pos, dir, cost)) = q.pop_front() {
        if pos == target_pos {
            best_target_cost = cost;
            continue;
        }

        let right = dir.rotate_clockwise();
        let left = dir.rotate_counterclockwise();

        [
            (pos + dir, dir, cost + 1),
            (pos + left, left, cost + 1000 + 1),
            (pos + right, right, cost + 1000 + 1),
        ]
        .iter()
        .for_each(|&(pos, dir, cost)| {
            if grid[pos] == '#' {
                return;
            }
            if best_cost_grid[pos][dir_to_index(dir)] <= cost {
                return;
            }
            if cost > best_target_cost {
                return;
            }

            best_cost_grid[pos][dir_to_index(dir)] = cost;
            q.push_back((pos, dir, cost));
        });
    }

    best_cost_grid
}

pub fn part_one(input: &str) -> Option<i64> {
    let (grid, start) = Grid::new_from_str_capture_start(input, &|c| c, &|c| c == 'S');
    let target_pos = grid.find_position_of(&'E').unwrap();

    let best_cost_grid = build_best_cost_grid(&grid, start, target_pos);
    let min_cost = *best_cost_grid[target_pos].iter().min().unwrap();

    Some(min_cost)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (grid, start) = Grid::new_from_str_capture_start(input, &|c| c, &|c| c == 'S');
    let target_pos = grid.find_position_of(&'E').unwrap();

    let best_cost_grid = build_best_cost_grid(&grid, start, target_pos);
    let (idx, &best_cost) = best_cost_grid[target_pos]
        .iter()
        .enumerate()
        .min_by_key(|&(_, &cost)| cost)
        .unwrap();
    let best_dir = index_to_dir(idx);
    let mut seen = Grid::new(grid.width, grid.height, false);

    // reverse lookup
    let (target_pos, start) = (start, target_pos);
    seen[start] = true;

    let mut q = VecDeque::new();
    q.push_back((start, best_dir, best_cost));

    while let Some((pos, dir, cost)) = q.pop_front() {
        if pos == target_pos {
            continue;
        }

        let right = dir.rotate_clockwise();
        let left = dir.rotate_counterclockwise();
        [
            (pos - dir, dir, cost - 1),
            (pos - dir, right, cost - 1000 - 1),
            (pos - dir, left, cost - 1000 - 1),
            (pos + left, left, cost - 1000 - 1),
            (pos + right, left, cost - 1000 - 1),
        ]
        .iter()
        .for_each(|&(pos, dir, cost)| {
            if cost < 0 {
                return;
            }
            if grid[pos] == '#' {
                return;
            }
            if cost == best_cost_grid[pos][dir_to_index(dir)] {
                seen[pos] = true;
                q.push_back((pos, dir, cost));
            }
        });
    }

    let tile_count = seen.iter().filter(|&&b| b).count();
    Some(tile_count as i64 + 1 /* + target */)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(9024));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(34));
    }
}
