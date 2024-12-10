use heapless::FnvIndexSet;
use heapless::Vec as HeaplessVec;
use mygrid::heapless_grid::HeaplessGrid;
use mygrid::{direction::ORTHOGONAL, point::Point};

type Queue = HeaplessVec<Point, 128>;
type Found = FnvIndexSet<Point, 32>;

advent_of_code::solution!(10);

#[inline]
fn solve<const PART: u8>(input: &str) -> Option<u32> {
    let grid: HeaplessGrid<u8, 2048> =
        HeaplessGrid::new_from_str(input, |c| c.to_digit(10).unwrap() as u8);

    let trailheads = grid
        .iter_item_and_position()
        .filter(|(_, &value)| value == 0)
        .map(|(pos, _)| pos);

    let trailhead_scores = trailheads
        .map(|starting_point| {
            let mut found = 0;
            let mut found_unique = Found::new();

            let mut q = Queue::new();
            q.push(starting_point).unwrap();

            while let Some(current) = q.pop() {
                if grid[current] == 9 {
                    found += 1;
                    found_unique.insert(current).unwrap();
                    continue;
                }
                for n in ORTHOGONAL
                    .iter()
                    .map(|&dir| current + dir)
                    .filter(|&p| grid.is_in_bounds(p))
                    .filter(|&p| grid[p] == grid[current] + 1)
                {
                    q.push(n).unwrap();
                }
            }

            if PART == 1 {
                found_unique.len() as u32
            } else {
                found
            }
        })
        .sum();

    Some(trailhead_scores)
}

#[inline]
pub fn part_one(input: &str) -> Option<u32> {
    solve::<1>(input)
}

#[inline]
pub fn part_two(input: &str) -> Option<u32> {
    solve::<2>(input)
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
