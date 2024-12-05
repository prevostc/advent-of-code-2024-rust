use heapless::FnvIndexMap as HeaplessMap;
use heapless::FnvIndexSet as HeaplessSet;
use heapless::Vec as HeaplessVec;

advent_of_code::solution!(5);

type PageNumBag = HeaplessSet<u8, 128>;
type OrderRules = HeaplessMap<u8, PageNumBag, 128>;
type PagesToPrint = HeaplessVec<u8, 128>;

fn parse_input(input: &str) -> (OrderRules, Vec<PagesToPrint>) {
    let mut map = OrderRules::new();
    let (rules, pages) = input.split_once("\n\n").unwrap();
    for line in rules.lines().filter(|l| !l.is_empty()) {
        let (left, right) = line.split_once('|').unwrap();
        let left: u8 = left.parse().unwrap();
        let right: u8 = right.parse().unwrap();
        if let Some(set) = map.get_mut(&left) {
            set.insert(right).unwrap();
        } else {
            let mut set = HeaplessSet::new();
            set.insert(right).unwrap();
            map.insert(left, set).unwrap();
        }
    }
    let mut pages_vec: Vec<PagesToPrint> = Vec::new();
    for line in pages.lines().filter(|l| !l.is_empty()) {
        let nums: PagesToPrint = line.split(',').map(|s| s.parse().unwrap()).collect();
        pages_vec.push(nums);
    }
    (map, pages_vec)
}

#[inline]
fn middle_num(page: &PagesToPrint) -> u8 {
    page[page.len() / 2]
}

#[inline]
fn is_correctly_ordered(page: &PagesToPrint, rules: &OrderRules) -> bool {
    let mut nums_before: PageNumBag = PageNumBag::new();
    for num in page {
        nums_before.insert(*num).unwrap();
        let is_num_valid = match rules.get(num) {
            None => true,
            Some(check_before) => check_before.intersection(&nums_before).count() == 0,
        };
        if !is_num_valid {
            return false;
        }
    }
    true
}

#[inline]
fn order_pages_in_place(pages: &mut PagesToPrint, rules: &OrderRules) {
    // find a non sorted pair
    // put right element before left element
    // repeat until sorted
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..pages.len() {
            if let Some(rule) = rules.get(&pages[i]) {
                for j in 0..i {
                    if rule.contains(&pages[j]) {
                        let tmp = pages[i];

                        // shift all elements from j to i - 1 to the right
                        for k in (j..i).rev() {
                            pages[k + 1] = pages[k];
                        }
                        pages[j] = tmp;
                        swapped = true;
                    }
                }
            };
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = parse_input(input);
    let sum = pages
        .iter()
        .filter(|p| is_correctly_ordered(p, &rules))
        .map(middle_num)
        .map(|n| n as u32)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, mut pages) = parse_input(input);
    let sum = pages
        .iter_mut()
        .filter(|p| !is_correctly_ordered(p, &rules))
        .map(|p| {
            order_pages_in_place(p, &rules);
            middle_num(p) as u32
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
