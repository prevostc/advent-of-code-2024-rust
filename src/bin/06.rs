advent_of_code::solution!(6);
use mygrid::direction::{Direction, UP};
use mygrid::grid::Grid;
use mygrid::point::Point;
use rustc_hash::FxHashSet;

#[derive(Default, Clone, PartialEq, Eq, Hash)]
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
) -> FxHashSet<Point> {
    let mut guard = Guard {
        position: start_pos,
        direction: UP,
    };
    let mut visited = FxHashSet::default();
    visited.insert(guard.position);

    while let Some(new_guard) = guard.turn(grid) {
        guard = new_guard;
        visited.insert(guard.position);
    }
    visited
}

fn does_start_pos_loop(grid: &Grid<char>, start_pos: Point) -> bool {
    let mut guard = Guard {
        position: start_pos,
        direction: UP,
    };
    let mut visited = FxHashSet::default();
    visited.insert(guard.clone());

    while let Some(new_guard) = guard.turn(grid) {
        guard = new_guard;
        if !visited.insert(guard.clone()) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start_pos) = parse_grid_and_start_pos(input);
    let visited = get_guard_path_positions_assuming_no_loops(&grid, start_pos);
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut grid, start_pos) = parse_grid_and_start_pos(input);
    let positions_to_check = get_guard_path_positions_assuming_no_loops(&grid, start_pos);

    let mut count = 0;
    for pos in positions_to_check {
        grid[pos] = 'O';
        if does_start_pos_loop(&grid, start_pos) {
            count += 1;
        }
        grid[pos] = '.';
    }
    Some(count)
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
