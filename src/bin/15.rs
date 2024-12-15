use heapless::Vec as HeaplessVec;
use itertools::Itertools;
use mygrid::{
    direction::{Direction, LEFT, RIGHT},
    grid::Grid,
    point::Point,
};

advent_of_code::solution!(15);

#[inline]
fn parse_input(input: &str) -> (Grid<char>, Point, Vec<Direction>) {
    let (grid_str, path_str) = input.split_once("\n\n").unwrap();
    let grid = Grid::new_char_grid_from_str(grid_str);
    let start = grid
        .iter_item_and_position()
        .find(|&(_, v)| *v == '@')
        .map(|(p, _)| p)
        .unwrap();
    let path = path_str
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| Direction::from(c))
        .collect();
    (grid, start, path)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, start, path) = parse_input(input);

    let mut robot_pos = start;
    for direction in path {
        //dbg!(&grid, &robot_pos, &direction.to_string());
        // find the first hole
        let mut lookup_pos = robot_pos;
        while grid[lookup_pos] != '.' && grid[lookup_pos] != '#' {
            lookup_pos = lookup_pos + direction;
        }
        if grid[lookup_pos] == '.' {
            //dbg!("found hole", lookup_pos);
            // backtrack to the robot position, moving everything in the way
            let mut backtrack_pos = lookup_pos;
            while backtrack_pos != robot_pos {
                let next_pos = backtrack_pos - direction;
                grid[backtrack_pos] = grid[next_pos];
                backtrack_pos = next_pos;
            }
            grid[robot_pos] = '.';
            robot_pos = robot_pos + direction;
        }
    }

    Some(
        grid.iter_item_and_position()
            .filter(|&(_, v)| *v == 'O')
            .map(|(p, _)| 100 * p.line as u32 + p.column as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (base_grid, base_start, path) = parse_input(input);

    // transform grid for part 2
    let mut start: Point = base_start;
    let mut grid = Grid::new(base_grid.width * 2, base_grid.height, '.');
    base_grid.iter_item_and_position().for_each(|(p, v)| {
        let p = Point::new(p.line, p.column * 2);
        (grid[p], grid[p + RIGHT]) = match *v {
            'O' => ('[', ']'),
            '@' => {
                start = p;
                ('@', '.')
            }
            _ => (*v, *v),
        };
    });

    let mut robot_pos = start;
    for direction in path {
        // fast path when the next position is a hole
        let next_pos = robot_pos + direction;
        if grid[next_pos] == '.' {
            grid[robot_pos] = '.';
            grid[next_pos] = '@';
            robot_pos = robot_pos + direction;
            continue;
        }

        // horizontal movement is just like p1
        if direction == LEFT || direction == RIGHT {
            let mut lookup_pos = robot_pos;
            while grid[lookup_pos] != '.' && grid[lookup_pos] != '#' {
                lookup_pos = lookup_pos + direction;
            }
            if grid[lookup_pos] == '.' {
                let mut backtrack_pos = lookup_pos;
                while backtrack_pos != robot_pos {
                    let next_pos = backtrack_pos - direction;
                    grid[backtrack_pos] = grid[next_pos];
                    backtrack_pos = next_pos;
                }
                grid[robot_pos] = '.';
                robot_pos = robot_pos + direction;
            }

            continue;
        }

        /**
         * vertical movement is backtracking but remembering positions at each steps
         *
         * Interesting cases
         *
         * Standard multi-push:
         *  ##########      ##########
         *  ##......##      ##.[][].##
         *  ##.[][].##  ->  ##..[]..##
         *  ##..[]..##      ##...@..##
         *  ##...@..##      ##......##
         *  ##########      ##########
         *
         * Extraction:
         *  ##########      ##########
         *  ##......##      ##.[][].##
         *  ##.[][].##  ->  ##..[]..##
         *  ##[][][]##      ##[].@[]##
         *  ##...@..##      ##......##
         *  ##########      ##########
         *
         * The snake
         *  ##########      ##########
         *  ##......##      ##..[]..##
         *  ##..[]..##      ##...[].##
         *  ##...[].##  ->  ##..[]..##
         *  ##..[]..##      ##...@..##
         *  ##...@..##      ##......##
         *  ##########      ##########
         *
         * moving up from @ will hit the hole at the bottom
         * moving down from @ will hit the hole at the top
         */
        type LookupSteps = HeaplessVec<LookupPos, 16>;
        type LookupPos = HeaplessVec<Point, 128>;
        let mut lookup_steps: LookupSteps = LookupSteps::new();
        let mut lookup_pos: LookupPos = LookupPos::new();
        lookup_pos.push(robot_pos).unwrap();
        lookup_steps.push(lookup_pos.clone()).unwrap();
        let mut can_move = true;

        while lookup_pos.len() > 0 && can_move {
            let mut next_lookup_pos: LookupPos = LookupPos::new();
            for &p in lookup_pos.iter() {
                if grid[p] == '.' {
                    continue;
                }
                if grid[p] == '#' {
                    can_move = false;
                    break;
                }
                let next_pos = p + direction;
                match grid[next_pos] {
                    '[' => {
                        next_lookup_pos.push(next_pos).unwrap();
                        next_lookup_pos.push(next_pos + RIGHT).unwrap();
                    }
                    ']' => {
                        next_lookup_pos.push(next_pos).unwrap();
                        next_lookup_pos.push(next_pos + LEFT).unwrap();
                    }
                    _ => next_lookup_pos.push(next_pos).unwrap(),
                }
            }
            lookup_steps.push(next_lookup_pos.clone()).unwrap();
            lookup_pos = next_lookup_pos;
        }

        if can_move {
            for (lookup_pos, prev_lookup_pos) in lookup_steps.iter().rev().tuple_windows() {
                for &p in lookup_pos.iter() {
                    if prev_lookup_pos.contains(&(p - direction)) {
                        grid[p] = grid[p - direction];
                    } else {
                        grid[p] = '.';
                    }
                }
            }

            grid[robot_pos] = '.';
            robot_pos = robot_pos + direction;
        }
    }

    Some(
        grid.iter_item_and_position()
            .filter(|&(_, v)| *v == '[')
            .map(|(p, _)| 100 * p.line as u32 + p.column as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(618));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
