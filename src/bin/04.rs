use mygrid::{
    direction::{ALL_AROUND, DOWN, LEFT, RIGHT, UP},
    grid::Grid,
};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new_from_str(input, |c| c);

    let res = grid
        .iter_item_and_position()
        .flat_map(|(point, &c)| ALL_AROUND.iter().map(move |d| (point, c, *d)))
        .filter(|&(_, c, _)| c == 'X')
        .filter(|&(point, _, d)| grid.is_in_bounds(point + (d * 3)))
        .filter(|&(point, _, d)| {
            let x_pos = point;
            let m_pos = x_pos + d;
            let a_pos = m_pos + d;
            let s_pos = a_pos + d;
            let spells_xmas = grid[x_pos] == 'X'
                && grid[m_pos] == 'M'
                && grid[a_pos] == 'A'
                && grid[s_pos] == 'S';

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
            let (Some(&top_left), Some(&top_right), Some(&bot_left), Some(&bot_right)) = (
                grid.get_item(point + UP + LEFT),
                grid.get_item(point + UP + RIGHT),
                grid.get_item(point + DOWN + LEFT),
                grid.get_item(point + DOWN + RIGHT),
            ) else {
                return false;
            };

            let spells_xmas_diag1 =
                (top_left == 'M' && bot_right == 'S') || (top_left == 'S' && bot_right == 'M');
            let spells_xmas_diag2 =
                (top_right == 'M' && bot_left == 'S') || (top_right == 'S' && bot_left == 'M');
            spells_xmas_diag1 && spells_xmas_diag2
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
