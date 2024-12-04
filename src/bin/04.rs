use mygrid::{
    direction::{ALL_AROUND, DOWN, LEFT, RIGHT, UP},
    grid::Grid,
};
use rayon::prelude::*;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new_from_str(input, |c| c);

    let res = grid
        .par_iter_item_and_position()
        .flat_map_iter(|(point, &c)| ALL_AROUND.iter().map(move |d| (point, c, *d)))
        .filter(|&(_, c, _)| c == 'X')
        .filter(|&(point, _, d)| grid.is_in_bounds(point + (d * 3)))
        .filter(|&(point, _, d)| {
            let spells_xmas = grid[point] == 'X'
                && grid[point + d] == 'M'
                && grid[point + d * 2] == 'A'
                && grid[point + d * 3] == 'S';
            spells_xmas
        })
        .count();

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new_from_str(input, |c| c);

    let res = grid
        .iter_item_and_position()
        .filter(|&(_, &c)| c == 'A')
        .filter(|&(point, _)| {
            let (Some(&tl), Some(&tr), Some(&bl), Some(&br)) = (
                grid.get_item(point + UP + LEFT),
                grid.get_item(point + UP + RIGHT),
                grid.get_item(point + DOWN + LEFT),
                grid.get_item(point + DOWN + RIGHT),
            ) else {
                return false;
            };

            let diag1 = (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M');
            let diag2 = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');
            diag1 && diag2
        })
        .count();

    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
