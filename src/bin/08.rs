use advent_of_code::into_group_map_heapless;
use heapless::FnvIndexMap as HeaplessHashMap;
use heapless::FnvIndexSet as HeaplessHashSet;
use heapless::Vec as HeaplessVec;
use itertools::Itertools;
use mygrid::grid::Grid;
use mygrid::point::Point;
advent_of_code::solution!(8);

const MAX_ANTENNA_TYPES: usize = 64;
const MAX_ANTENNA_PER_TYPE: usize = 16;
type AntennaMap<'a> =
    HeaplessHashMap<&'a char, HeaplessVec<Point, MAX_ANTENNA_PER_TYPE>, MAX_ANTENNA_TYPES>;

fn solve(
    input: &str,
    update_set: &mut impl FnMut(&Grid<char>, &mut HeaplessHashSet<Point, 2048>, (Point, Point)),
) -> Option<u32> {
    let grid = Grid::new_char_grid_from_str(input);

    let antennas: AntennaMap = into_group_map_heapless(
        grid.iter_item_and_position()
            .filter(|(_, &c)| c != '.')
            .map(|(point, c)| (c, point)),
    )
    .unwrap();

    let mut pos_set: HeaplessHashSet<Point, 2048> = HeaplessHashSet::new();
    for (_, vec) in antennas.iter() {
        for (a, b) in vec.iter().tuple_combinations() {
            for (&a, &b) in [(a, b), (b, a)] {
                update_set(&grid, &mut pos_set, (a, b));
            }
        }
    }

    Some(pos_set.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, &mut |grid, pos_set, (a, b)| {
        let dir = b.as_vector_direction(&a);
        let p = a + dir;
        if grid.is_in_bounds(p) {
            pos_set.insert(p).unwrap();
        }
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, &mut |grid, pos_set, (a, b)| {
        let dir = b.as_vector_direction(&a);
        let mut p = a + dir;
        pos_set.insert(a).unwrap();
        pos_set.insert(b).unwrap();
        while grid.is_in_bounds(p) {
            pos_set.insert(p).unwrap();
            p = p + dir;
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
