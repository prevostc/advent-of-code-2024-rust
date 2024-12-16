use object_pool::Pool;
use std::collections::VecDeque;

use mygrid::{
    direction::{Direction, DOWN, LEFT, RIGHT, UP},
    grid::Grid,
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

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, start) = Grid::new_from_str_capture_start(input, &|c| c, &|c| c == 'S');
    let target_pos = grid.find_position_of(&'E').unwrap();

    let mut q = VecDeque::new();
    q.push_back((start, RIGHT, 0));

    let mut best_cost_grid = Grid::new(grid.width, grid.height, [u64::MAX; 4]);

    while let Some((pos, dir, cost)) = q.pop_front() {
        if pos == target_pos {
            best_cost_grid[pos][dir_to_index(dir)] = cost;
            continue;
        }

        let right = dir.rotate_clockwise();
        let left = dir.rotate_counterclockwise();

        for &(pos, dir, cost) in [
            (pos + dir, dir, cost + 1),
            (pos + right, right, cost + 1000 + 1),
            (pos + left, left, cost + 1000 + 1),
        ]
        .iter()
        {
            if grid[pos] == '#' {
                continue;
            }
            let best_cost = best_cost_grid[pos][dir_to_index(dir)];
            if best_cost <= cost {
                continue;
            }
            best_cost_grid[pos][dir_to_index(dir)] = cost;

            q.push_back((pos, dir, cost));
        }
    }

    let min_cost = *best_cost_grid[target_pos].iter().min().unwrap();
    Some(min_cost)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, start) = Grid::new_from_str_capture_start(input, &|c| c, &|c| c == 'S');
    let target_pos = grid.find_position_of(&'E').unwrap();

    const PATH_CAPACITY: usize = 512;
    let path_pool = Pool::new(256, || Vec::with_capacity(PATH_CAPACITY));
    let mut path = path_pool.pull(|| Vec::with_capacity(PATH_CAPACITY));
    path.push(start);

    let mut q = VecDeque::new();
    q.push_back((path, start, RIGHT, 0));

    let base_false_grid = Grid::new(grid.width, grid.height, false);
    let mut best_spots_grid = base_false_grid.clone();
    let mut best_target_cost = u64::MAX;
    let mut best_cost_grid = Grid::new(grid.width, grid.height, [u64::MAX; 4]);

    while let Some((mut path, pos, dir, cost)) = q.pop_front() {
        if pos == target_pos {
            if best_target_cost < cost {
                continue;
            }

            // reset best_spots_grid if we found a better path
            if best_target_cost > cost {
                best_spots_grid = base_false_grid.clone();
            }

            for &p in path.iter() {
                best_spots_grid[p] = true;
            }

            best_target_cost = cost;
            continue;
        }

        best_cost_grid[pos][dir_to_index(dir)] = cost;

        let right = dir.rotate_clockwise();
        let left = dir.rotate_counterclockwise();
        let is_viable = |&(pos, dir, cost)| {
            if grid[pos] == '#' {
                return false;
            }
            let best_cost_to_next_pos = best_cost_grid[pos][dir_to_index(dir)];
            if best_cost_to_next_pos < cost {
                return false;
            }
            if cost > best_target_cost {
                return false;
            }
            true
        };

        let all_options = [
            (pos + dir, dir, cost + 1),
            (pos + left, left, cost + 1000 + 1),
            (pos + right, right, cost + 1000 + 1),
        ];

        let viable_options = [
            is_viable(&all_options[0]),
            is_viable(&all_options[1]),
            is_viable(&all_options[2]),
        ];

        // fast path when there is only one option to avoid copying the path
        let viable_count = viable_options.iter().filter(|&&b| b).count();
        match viable_count {
            0 => continue,
            1 => {
                let idx = viable_options.iter().position(|&b| b).unwrap();
                let (pos, dir, cost) = all_options[idx];
                path.push(pos);
                q.push_back((path, pos, dir, cost));
            }
            _ => viable_options
                .iter()
                .enumerate()
                .for_each(|(idx, &is_viable)| {
                    if !is_viable {
                        return;
                    }

                    let (pos, dir, cost) = all_options[idx];
                    let mut new_path = path_pool.pull(|| Vec::with_capacity(PATH_CAPACITY));
                    new_path.clone_from(&path);
                    new_path.push(pos);
                    q.push_back((new_path, pos, dir, cost));
                }),
        }
    }

    let tile_count = best_spots_grid.iter().filter(|&&b| b).count();
    Some(tile_count as u64)
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
}
