use std::collections::VecDeque;

use mygrid::{
    direction::{Direction, DOWN, LEFT, RIGHT, UP},
    grid::Grid,
    point::Point,
};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, start) = Grid::new_from_str_capture_start(input, &|c| c, &|c| c == 'S');
    let target_pos = grid
        .iter_item_and_position()
        .filter(|&(_, c)| *c == 'E')
        .map(|(p, _)| p)
        .next()
        .unwrap();

    #[derive(Debug, PartialEq, Eq)]
    struct State {
        pos: Point,
        dir: Direction,
        cost: u64,
    }

    let mut q = VecDeque::new();
    q.push_back(State {
        pos: start,
        dir: RIGHT,
        cost: 0,
    });

    let mut best_cost_grid = Grid::new(grid.width, grid.height, [u64::MAX; 4]);
    let dir_to_index = |d: Direction| match d {
        RIGHT => 0,
        DOWN => 1,
        LEFT => 2,
        UP => 3,
        _ => unreachable!(),
    };

    while let Some(s) = q.pop_front() {
        if s.pos == target_pos {
            best_cost_grid[s.pos][dir_to_index(s.dir)] = s.cost;
            continue;
        }

        let right = s.dir.rotate_clockwise();
        let left = s.dir.rotate_counterclockwise();

        for &(pos, dir, cost) in [
            (s.pos + s.dir, s.dir, s.cost + 1),
            (s.pos + right, right, s.cost + 1000 + 1),
            (s.pos + left, left, s.cost + 1000 + 1),
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

            let new_state = State { pos, dir, cost };
            q.push_back(new_state);
        }
    }

    let min_cost = *best_cost_grid[target_pos].iter().min().unwrap();
    Some(min_cost)
}

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

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, start) = Grid::new_from_str_capture_start(input, &|c| c, &|c| c == 'S');
    let target_pos = grid
        .iter_item_and_position()
        .filter(|&(_, c)| *c == 'E')
        .map(|(p, _)| p)
        .next()
        .unwrap();

    use object_pool::{Pool, Reusable};

    struct State<'a> {
        path: Reusable<'a, Vec<Point>>,
        pos: Point,
        dir: Direction,
        cost: u64,
    }

    const PATH_CAPACITY: usize = 512;
    let path_pool = Pool::new(256, || Vec::with_capacity(PATH_CAPACITY));
    let mut path = path_pool.pull(|| Vec::with_capacity(PATH_CAPACITY));
    path.push(start);

    let mut q = VecDeque::new();
    q.push_back(State {
        path,
        pos: start,
        dir: RIGHT,
        cost: 0,
    });

    let base_false_grid = Grid::new(grid.width, grid.height, false);
    let mut best_spots_grid = base_false_grid.clone();
    let mut best_target_cost = u64::MAX;
    let mut best_cost_grid = Grid::new(grid.width, grid.height, [u64::MAX; 4]);

    while let Some(mut s) = q.pop_front() {
        if s.pos == target_pos {
            if best_target_cost < s.cost {
                continue;
            }

            // reset best_spots_grid if we found a better path
            if best_target_cost > s.cost {
                best_spots_grid = base_false_grid.clone();
            }

            for &p in s.path.iter() {
                best_spots_grid[p] = true;
            }

            best_target_cost = s.cost;
            continue;
        }

        best_cost_grid[s.pos][dir_to_index(s.dir)] = s.cost;

        let right = s.dir.rotate_clockwise();
        let left = s.dir.rotate_counterclockwise();
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
            (s.pos + s.dir, s.dir, s.cost + 1),
            (s.pos + left, left, s.cost + 1000 + 1),
            (s.pos + right, right, s.cost + 1000 + 1),
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
                s.path.push(pos);
                s.cost = cost;
                s.pos = pos;
                s.dir = dir;
                q.push_back(s);
            }
            _ => {
                for (idx, &is_viable) in viable_options.iter().enumerate() {
                    if !is_viable {
                        continue;
                    }

                    let (pos, dir, cost) = all_options[idx];
                    let mut new_path = path_pool.pull(|| Vec::with_capacity(PATH_CAPACITY));
                    new_path.clone_from(&s.path);
                    new_path.push(pos);
                    let new_state = State {
                        path: new_path,
                        pos,
                        dir,
                        cost,
                    };
                    q.push_back(new_state);
                }
            }
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
