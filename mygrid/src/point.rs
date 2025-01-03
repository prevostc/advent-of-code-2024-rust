// grid library
// contains everything related to grids, points and directions
// heavily inspired by the amazing maneatingape repo, from which I learned a lot, plz see:
// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/util/point.rs

use std::{
    hash::{Hash, Hasher},
    ops::Mul,
};

use crate::direction::Direction;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Point {
    pub line: isize,
    pub column: isize,
}

impl Point {
    #[inline]
    pub const fn new_i32(line: i32, column: i32) -> Self {
        assert!(line >= 0);
        assert!(column >= 0);
        Point::new(line as isize, column as isize)
    }

    #[inline]
    pub const fn new_usize(line: usize, column: usize) -> Self {
        Point::new(line as isize, column as isize)
    }

    #[inline]
    pub const fn new(line: isize, column: isize) -> Self {
        Point { line, column }
    }

    #[inline]
    pub fn apply_direction(&self, direction: Direction) -> Self {
        Point::new(
            self.line + direction.vertical,
            self.column + direction.horizontal,
        )
    }

    #[inline]
    pub fn max(&self, other: &Point) -> Self {
        Point::new(self.line.max(other.line), self.column.max(other.column))
    }

    #[inline]
    pub fn min(&self, other: &Point) -> Self {
        Point::new(self.line.min(other.line), self.column.min(other.column))
    }

    #[inline]
    pub fn is_aligned(&self, other: &Point) -> bool {
        self.line == other.line || self.column == other.column
    }

    #[inline]
    pub fn is_between_inclusive(&self, a: &Point, b: &Point) -> bool {
        if !self.is_aligned(a) || !self.is_aligned(b) || !a.is_aligned(b) {
            return false;
        }

        let min_line = a.line.min(b.line);
        let max_line = a.line.max(b.line);
        let min_column = a.column.min(b.column);
        let max_column = a.column.max(b.column);

        let is_between_lines = (min_line..=max_line).contains(&self.line);
        let is_between_columns = (min_column..=max_column).contains(&self.column);

        (a.line == b.line && a.line == self.line && is_between_columns)
            || (a.column == b.column && a.column == self.column && is_between_lines)
    }

    #[inline]
    pub fn from_usize(value: usize, grid_width: usize) -> Self {
        Point::new(
            value as isize / grid_width as isize,
            value as isize % grid_width as isize,
        )
    }

    #[inline]
    pub fn to_usize(&self, grid_width: usize) -> usize {
        (self.line as usize) * grid_width + self.column as usize
    }

    #[inline]
    pub fn to_u128(&self) -> u128 {
        ((self.line as u128) << 64) | (self.column as u128)
    }

    #[inline]
    pub fn from_u128(value: u128) -> Self {
        Point::new(
            (value >> 64) as isize,
            (value & 0xFFFFFFFFFFFFFFFF) as isize,
        )
    }

    #[inline]
    pub fn as_direction(&self) -> Direction {
        Direction::new(self.line, self.column)
    }

    #[inline]
    pub fn as_vector_direction(&self, other: &Point) -> Direction {
        Direction::new(other.line - self.line, other.column - self.column)
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_isize(self.line);
        hasher.write_isize(self.column);
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(l:{}, c:{})", self.line, self.column)
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, direction: isize) -> Self {
        Point::new(self.line * direction, self.column * direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_point() {
        let point = Point::new(1, 2);
        assert_eq!(point.line, 1);
        assert_eq!(point.column, 2);
    }

    #[test]
    pub fn test_point_new_i32() {
        let point = Point::new_i32(1, 2);
        assert_eq!(point.line, 1);
        assert_eq!(point.column, 2);
    }

    #[test]
    pub fn test_point_max() {
        let point = Point::new(1, 2);
        let other = Point::new(3, 1);
        let max = point.max(&other);
        assert_eq!(max.line, 3);
        assert_eq!(max.column, 2);
    }

    #[test]
    pub fn test_point_min() {
        let point = Point::new(1, 2);
        let other = Point::new(3, 1);
        let min = point.min(&other);
        assert_eq!(min.line, 1);
        assert_eq!(min.column, 1);
    }

    #[test]
    pub fn test_point_is_aligned() {
        let point = Point::new(1, 2);
        let other = Point::new(1, 3);
        assert!(point.is_aligned(&other));

        let point = Point::new(1, 2);
        let other = Point::new(3, 1);
        assert!(!point.is_aligned(&other));

        let point = Point::new(1, 2);
        let other = Point::new(1, 2);
        assert!(point.is_aligned(&other));
    }

    #[test]
    pub fn test_point_is_between_inclusive() {
        let point = Point::new(1, 2);
        let a = Point::new(0, 2);
        let b = Point::new(2, 2);
        assert!(point.is_between_inclusive(&a, &b));

        let point = Point::new(1, 2);
        let a = Point::new(1, 0);
        let b = Point::new(1, 4);
        assert!(point.is_between_inclusive(&a, &b));

        let point = Point::new(1, 2);
        let a = Point::new(0, 1);
        let b = Point::new(2, 3);
        assert!(!point.is_between_inclusive(&a, &b));

        let point = Point::new(1, 2);
        let a = Point::new(1, 2);
        let b = Point::new(2, 3);
        assert!(!point.is_between_inclusive(&a, &b));

        let point = Point::new(1, 2);
        let a = Point::new(2, 1);
        let b = Point::new(3, 2);
        assert!(!point.is_between_inclusive(&a, &b));
    }

    // #[test]
    // pub fn test_infinite_grid_to_real_grid() {
    //     let point = Point::new(45, 20);
    //     let real_grid_lines = 3;
    //     let real_grid_columns = 4;
    //     let real_grid_point = point.infinite_grid_to_real_grid(real_grid_lines, real_grid_columns);
    //     assert_eq!(real_grid_point.line, 0);
    //     assert_eq!(real_grid_point.column, 0);
    // }

    // #[test]
    // pub fn test_infinite_grid_to_real_grid_negative() {
    //     let point = Point::new(-5, -8);
    //     let real_grid_lines = 3;
    //     let real_grid_columns = 4;
    //     let real_grid_point = point.infinite_grid_to_real_grid(real_grid_lines, real_grid_columns);
    //     assert_eq!(real_grid_point.line, 1);
    //     assert_eq!(real_grid_point.column, 0);
    // }
}
