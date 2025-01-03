advent_of_code::solution!(6);
use mygrid::direction::{Direction, UP};
use mygrid::grid::Grid;
use mygrid::point::Point;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::{FxHashMap, FxHashSet};

type JumpTable = FxHashMap<Guard, Guard>;

#[derive(Default, Clone, PartialEq, Eq, Hash, Debug)]
struct Guard {
    position: Point,
    direction: Direction,
}

impl Guard {
    fn turn(&self, grid: &Grid<char>) -> Option<Guard> {
        let mut dir = self.direction;
        for _ in 0..5 {
            let next_pos = self.position.apply_direction(dir);

            let Some(&next_direction) = grid.get_item(next_pos) else {
                continue;
            };
            if next_direction == '.' {
                return Some(Guard {
                    position: next_pos,
                    direction: dir,
                });
            }
            dir = dir.rotate_clockwise();
        }
        None
    }
}

fn parse_grid_and_start_pos(input: &str) -> (Grid<char>, Point) {
    Grid::new_from_str_capture_start(
        input,
        &|c| match c {
            '^' => '.',
            _ => c,
        },
        &|c| c == '^',
    )
}

fn get_guard_path_positions_assuming_no_loops(
    grid: &Grid<char>,
    start_pos: Point,
) -> (FxHashSet<Point>, JumpTable) {
    let mut guard = Guard {
        position: start_pos,
        direction: UP,
    };
    let mut visited = FxHashSet::with_capacity_and_hasher(10_000, Default::default());
    let mut jump_table = JumpTable::with_capacity_and_hasher(10_000, Default::default());
    let mut old_guard = guard.clone();

    while let Some(new_guard) = guard.turn(grid) {
        if new_guard.direction != guard.direction {
            jump_table.insert(old_guard, guard.clone());
            old_guard = new_guard.clone();
        }
        guard = new_guard;
        visited.insert(guard.position);
    }
    (visited, jump_table)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start_pos) = parse_grid_and_start_pos(input);
    let (visited, _) = get_guard_path_positions_assuming_no_loops(&grid, start_pos);
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, start_pos) = parse_grid_and_start_pos(input);
    let (visited, jump_table) = get_guard_path_positions_assuming_no_loops(&grid, start_pos);

    let count = visited
        .par_iter()
        .filter(|&&pos| {
            let mut grid = grid.clone();
            grid[pos] = 'O';

            let jump_table: JumpTable = jump_table
                .iter()
                .filter(|(key, val)| !pos.is_between_inclusive(&key.position, &val.position))
                .map(|(key, val)| (key.clone(), val.clone()))
                .collect();

            // remove entries from jump_table that contain pos between key and value
            let mut guard = Guard {
                position: start_pos,
                direction: UP,
            };

            let mut visited = FxHashSet::with_capacity_and_hasher(10_000, Default::default());
            visited.insert(guard.clone());

            while let Some(new_guard) = guard.turn(&grid) {
                guard = new_guard;
                if !visited.insert(guard.clone()) {
                    return true;
                }
                if let Some(next_guard) = jump_table.get(&guard) {
                    guard = next_guard.clone();
                }
            }
            false
        })
        .count();

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
