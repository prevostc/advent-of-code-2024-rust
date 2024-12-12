use heapless::Vec as HeaplessVec;
use std::collections::VecDeque;

use mygrid::{direction::ORTHOGONAL, grid::Grid};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new_char_grid_from_str(input);
    let mut visited = Grid::new(grid.width, grid.height, false);
    let base_region = visited.clone();

    let mut res: u32 = 0;

    for pos in grid.iter_positions() {
        if visited[pos] {
            continue;
        }

        let mut region_perimeter = 0;
        let mut region_area = 0;
        let plant = grid[pos];
        let mut region = base_region.clone();
        let mut q = VecDeque::with_capacity(100);
        q.push_back(pos);
        while let Some(current_pos) = q.pop_front() {
            if region[current_pos] {
                continue;
            }

            region[current_pos] = true;
            visited[current_pos] = true;

            region_area += 1;

            let positions: HeaplessVec<_, 4> = ORTHOGONAL
                .iter()
                .map(|&d| current_pos + d)
                .filter(|&n| grid.is_in_bounds(n))
                .filter(|&n| grid[n] == plant)
                .collect();

            let region_neighbors = positions.iter().filter(|&&n| region[n]).count() as isize;
            region_perimeter += -region_neighbors + (4 - region_neighbors);

            for neighbor in positions.iter().filter(|&&n| !region[n]) {
                q.push_back(*neighbor);
            }
        }

        let res_price = region_area * region_perimeter;
        res += res_price as u32;
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new_char_grid_from_str(input);
    let mut visited = Grid::new(grid.width, grid.height, false);
    let base_region = visited.clone();

    let mut res: u32 = 0;

    for pos in grid.iter_positions() {
        if visited[pos] {
            continue;
        }
        // dbg!("================================================================");
        // dbg!(&pos, &grid[pos]);
        // dbg!("================================================================");

        let mut region_sides = 0;
        let mut region_area = 0;
        let plant = grid[pos];
        let mut region = base_region.clone();
        let mut q = VecDeque::with_capacity(100);
        q.push_back(pos);
        while let Some(current_pos) = q.pop_front() {
            if region[current_pos] {
                continue;
            }

            region[current_pos] = true;
            visited[current_pos] = true;

            region_area += 1;

            // dbg!(&region.to_debug());

            let mut sides_diff = 0;
            for &d in ORTHOGONAL.iter() {
                let n = current_pos + d;
                let in_region = region.is_true(n);
                if in_region {
                    let mut removes_side = true;
                    let n_1 = current_pos + d.rotate_clockwise();
                    let d_1 = n_1 + d;
                    let n_1_keep_side = region.is_false(n_1) && region.is_true(d_1);

                    let n_2 = current_pos + d.rotate_counterclockwise();
                    let d_2 = n_2 + d;
                    let n_2_keep_side = region.is_false(n_2) && region.is_true(d_2);
                    if n_1_keep_side || n_2_keep_side {
                        removes_side = false;
                    }

                    if removes_side {
                        sides_diff -= 1;
                        // dbg!("removes side", &n, &d, &n_1_keep_side, &n_2_keep_side);
                    } else {
                        // dbg!("not removing side", &n, &d, &n_1_keep_side, &n_2_keep_side);
                    }

                    // add 1 if we break a side
                    let splits_existing_side = region.is_false(n_1)
                        && region.is_false(n_2)
                        && region.is_true(d_1)
                        && region.is_true(d_2);
                    if splits_existing_side {
                        sides_diff += 1;
                        //dbg!("splits existing side", &n, &d);
                    }
                } else {
                    let mut adds_side = true;
                    let n_1 = current_pos + d.rotate_clockwise();
                    let d_1 = n_1 + d;
                    let n_1_has_same_side = region.is_true(n_1) && region.is_false(d_1);

                    let n_2 = current_pos + d.rotate_counterclockwise();
                    let d_2 = n_2 + d;
                    let n_2_has_same_side = region.is_true(n_2) && region.is_false(d_2);

                    if n_1_has_same_side || n_2_has_same_side {
                        adds_side = false;
                    }

                    if adds_side {
                        sides_diff += 1;
                        //dbg!("adds side", &n, &d, &n_1_has_same_side, &n_2_has_same_side);
                    } else {
                        // dbg!(
                        //     "not adding side",
                        //     &n,
                        //     &d,
                        //     &n_1_has_same_side,
                        //     &n_2_has_same_side
                        // );
                    }

                    let merges_existing_sides = region.is_true(n_1)
                        && region.is_true(n_2)
                        && region.is_false(d_1)
                        && region.is_false(d_2);
                    if merges_existing_sides {
                        sides_diff -= 1;
                    }
                }
            }

            region_sides += sides_diff;
            //dbg!(&region_sides, sides_diff);

            for neighbor in ORTHOGONAL
                .iter()
                .map(|&d| current_pos + d)
                .filter(|&n| grid.is_in_bounds(n))
                .filter(|&n| grid[n] == plant)
                .filter(|&n| !region[n])
            {
                q.push_back(neighbor);
            }
        }

        let res_price = region_area * region_sides;

        // dbg!(pos, plant, &region_area, &region_sides, &res_price);
        res += res_price as u32;
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let input = "AAAA\nBBCD\nBBCC\nEEEC\n";

        let result = part_one(&input);
        assert_eq!(result, Some(10 * 4 + 10 * 4 + 8 * 4 + 8 * 3 + 4 * 1));
    }

    #[test]
    fn test_part_one_2() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_2() {
        let input = "AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA\n";
        let result = part_two(&input);
        assert_eq!(result, Some(368));
    }

    #[test]
    fn test_part_two_3() {
        let input = "EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE\n";
        let result = part_two(&input);
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_4() {
        let input = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO\n";
        let result = part_two(&input);
        assert_eq!(result, Some(436));
    }
}
