use std::{collections::VecDeque, u32};

use mygrid::{direction::ORTHOGONAL, grid::Grid, point::Point};

advent_of_code::solution!(18);

#[inline]
fn parse_points<'a>(input: &'a str) -> impl Iterator<Item = Point> + 'a {
    input.lines().filter(|line| !line.is_empty()).map(|line| {
        let (col, line) = line.split_once(',').unwrap();
        Point::new(line.parse().unwrap(), col.parse().unwrap())
    })
}

#[inline]
fn fill_min_dst_grid(grid: &Grid<char>, min_dst: &mut Grid<u32>, start: (Point, u32), end: Point) {
    let mut q = VecDeque::new();
    q.push_back(start);

    while let Some((current, dst)) = q.pop_front() {
        min_dst[current] = min_dst[current].min(dst);
        if current == end {
            continue;
        }
        ORTHOGONAL
            .iter()
            .map(|&dir| current + dir)
            .filter(|&p| grid.is_in_bounds(p))
            .filter(|&p| grid[p] == '.')
            .for_each(|p| {
                if min_dst[p] > min_dst[current] + 1 {
                    min_dst[p] = min_dst[current] + 1;
                    q.push_back((p, min_dst[p]));
                }
            })
    }
}

fn find_shortest_path(input: &str, width: usize, height: usize, take: usize) -> Option<u32> {
    let mut grid = Grid::new(width, height, '.');
    let points = parse_points(input);
    points.take(take).for_each(|p| {
        grid[p] = '#';
    });

    let start = Point::new(0, 0);
    let end = Point::new(width as isize - 1, height as isize - 1);
    let mut min_dst = Grid::new(width, height, u32::MAX);
    min_dst[start] = 0;

    fill_min_dst_grid(&grid, &mut min_dst, (start, 0), end);

    Some(min_dst[end])
}

pub fn part_one(input: &str) -> Option<u32> {
    find_shortest_path(input, 71, 71, 1024)
}

// work in reverse, fill them all in and find the first point that makes it possible to reach the end
fn find_cutoff(input: &str, width: usize, height: usize, _initial_take: usize) -> Option<Point> {
    let mut grid = Grid::new(width, height, '.');
    let points = parse_points(input).collect::<Vec<_>>();

    points.iter().for_each(|&p| {
        grid[p] = '#';
    });

    let start = Point::new(0, 0);
    let end = Point::new(width as isize - 1, height as isize - 1);
    let mut min_dst = Grid::new(width, height, u32::MAX);
    min_dst[start] = 0;

    fill_min_dst_grid(&grid, &mut min_dst, (start, 0), end);

    points.iter().rev().find_map(|&p| {
        grid[p] = '.';

        let min_neighbour_dst = ORTHOGONAL
            .iter()
            .map(|&dir| p + dir)
            .filter(|&p| grid.is_in_bounds(p))
            .map(|p| min_dst[p])
            .min()
            .unwrap_or(u32::MAX);

        if min_neighbour_dst == u32::MAX {
            return None;
        }
        let dst = min_neighbour_dst + 1;

        fill_min_dst_grid(&grid, &mut min_dst, (p, dst), end);

        if min_dst[end] != u32::MAX {
            return Some(p);
        }

        return None;
    })
}

pub fn part_two(input: &str) -> Option<String> {
    let cutoff = find_cutoff(input, 71, 71, 1024);
    cutoff.map(|p| format!("{},{}", p.column, p.line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = find_shortest_path(
            &advent_of_code::template::read_file("examples", DAY),
            7,
            7,
            12,
        );
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = find_cutoff(
            &advent_of_code::template::read_file("examples", DAY),
            7,
            7,
            12,
        );
        assert_eq!(result, Some(Point::new(1, 6)));
    }
}
