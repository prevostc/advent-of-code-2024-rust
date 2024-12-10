use heapless::FnvIndexSet;
use mygrid::{direction::ORTHOGONAL, grid::Grid, point::Point};
use std::collections::VecDeque;

type Found = FnvIndexSet<Point, 128>;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<u8> = Grid::new_from_str(input, |c| c.to_digit(10).unwrap() as u8);

    let trailheads = grid
        // TODO: par_iter_item_and_position()
        .iter_item_and_position()
        .filter(|(_, &value)| value == 0)
        .map(|(pos, _)| pos);

    let trailhead_scores = trailheads
        .map(|starting_point| {
            let mut visited = Grid::new(grid.width, grid.height, false);
            let mut found = Found::new();

            let mut q = VecDeque::with_capacity(128);
            q.push_back(starting_point);
            visited[starting_point] = true;

            while let Some(current) = q.pop_front() {
                visited[current] = true;
                if grid[current] == 9 {
                    visited[current] = true;
                    found.insert(current).unwrap();
                    continue;
                }
                for n in ORTHOGONAL
                    .iter()
                    .map(|&dir| current + dir)
                    .filter(|&p| grid.is_in_bounds(p))
                    .filter(|&p| grid[p] == grid[current] + 1)
                    .filter(|&p| !visited[p])
                {
                    q.push_front(n);
                }
            }

            found.len() as u32
        })
        .sum();

    Some(trailhead_scores)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Grid<u8> = Grid::new_from_str(input, |c| c.to_digit(10).unwrap() as u8);

    let trailheads = grid
        // TODO: par_iter_item_and_position()
        .iter_item_and_position()
        .filter(|(_, &value)| value == 0)
        .map(|(pos, _)| pos);

    let trailhead_scores = trailheads
        .map(|starting_point| {
            let mut found = 0;

            let mut q = VecDeque::with_capacity(128);
            q.push_back(starting_point);

            while let Some(current) = q.pop_front() {
                if grid[current] == 9 {
                    found += 1;
                    continue;
                }
                for n in ORTHOGONAL
                    .iter()
                    .map(|&dir| current + dir)
                    .filter(|&p| grid.is_in_bounds(p))
                    .filter(|&p| grid[p] == grid[current] + 1)
                {
                    q.push_front(n);
                }
            }

            found
        })
        .sum();

    Some(trailhead_scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let input = "0123\n1234\n8765\n9876\n";
        let result = part_one(&input);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_2() {
        let input = "8880888\n8881888\n8882888\n6543456\n7888887\n8888888\n9888889\n";
        let result = part_one(&input);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_3() {
        let input = "8890889\n8881898\n8882887\n6543456\n7658987\n8768888\n9878888\n";
        let result = part_one(&input);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_4() {
        let input = "1088988\n2888888\n3888788\n4567654\n8888883\n8889882\n8888801\n";
        let result = part_one(&input);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_5() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
