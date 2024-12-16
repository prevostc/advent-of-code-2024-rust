use std::fmt::Debug;

advent_of_code::solution!(9);

#[derive(Debug, Clone)]
struct Block {
    length: u32,
    file_id: Option<u32>,
}

impl Block {
    fn is_free(&self) -> bool {
        self.file_id.is_none()
    }

    fn split(&self, length: u32) -> (Block, Option<Block>) {
        if self.length <= length {
            (self.clone(), None)
        } else {
            (
                Block {
                    length: length,
                    file_id: self.file_id,
                },
                Some(Block {
                    length: self.length - length,
                    file_id: self.file_id,
                }),
            )
        }
    }
}

// struct BlocksFmt<'a>(&'a Vec<Block>);

// impl Debug for BlocksFmt<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for block in self.0 {
//             for _ in 0..block.length {
//                 match block.file_id {
//                     Some(id) => write!(f, "{}", id)?,
//                     None => write!(f, ".")?,
//                 }
//             }
//         }
//         Ok(())
//     }
// }

fn checksum(blocks: &Vec<Block>) -> u64 {
    let mut sum: u64 = 0;
    let mut idx = 0;
    for block in blocks {
        for _ in 0..block.length {
            if let Some(file_id) = block.file_id {
                sum += idx * (file_id as u64);
            }
            idx += 1;
        }
    }
    sum
}

fn parse_blocks(input: &str) -> Vec<Block> {
    let nums = input
        .chars()
        .map(|c| c.to_digit(10))
        .filter(|r| r.is_some())
        .map(|r| r.unwrap());

    let mut blocks = Vec::with_capacity(1000);
    for (i, num) in nums.enumerate() {
        let file_id = if i % 2 == 0 {
            Some((i / 2) as u32)
        } else {
            None
        };

        if num == 0 {
            continue;
        }

        let b = Block {
            file_id,
            length: num,
        };

        blocks.push(b);
    }

    blocks
}

pub fn part_one(input: &str) -> Option<u64> {
    const EMPTY: usize = usize::MAX;
    let nums: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10))
        .filter(|r| r.is_some())
        .map(|r| r.unwrap())
        .enumerate()
        // create the full disk with all the file ids
        .flat_map(|(i, num)| (0..num).map(move |_| if i % 2 == 0 { i / 2 } else { EMPTY }))
        .collect();

    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut checksum = 0;
    while left <= right {
        let left_num = nums[left];
        if left_num != EMPTY {
            checksum += (left * left_num) as u64;
            left += 1;
            continue;
        }

        let right_num = nums[right];
        if right_num != EMPTY {
            checksum += (left * right_num) as u64;
            right -= 1;
            left += 1;
            continue;
        }

        right -= 1;
    }

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut blocks = parse_blocks(input);

    // 2 pointers again but this time we want to move whole files to the leftmost span of free space blocks that could fit the file
    // we want to move the files in order of decreasing file ID number
    // if there is no span of free space to the left of a file that is large enough to fit the file, the file does not move

    let mut spaces_block_indices_by_size: Vec<Vec<usize>> =
        (0..10).map(|_| Vec::with_capacity(500)).collect();

    // store indices smaller last so we can pop from the end and don't need to shift that much
    blocks
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, b)| b.file_id.is_none())
        .filter(|(_, b)| b.length > 0)
        .for_each(|(i, b)| {
            spaces_block_indices_by_size[b.length as usize].push(i);
        });

    let mut right_block_idx = blocks.len() - 1;
    while right_block_idx > 0 {
        let right_block = &blocks[right_block_idx].clone();
        if right_block.is_free() {
            right_block_idx -= 1;
            continue;
        }

        // look for the best free space block that can fit the file, only looking at the last of each size
        let mut best_free_space_block_idx = usize::MAX;
        for s in (right_block.length as usize)..spaces_block_indices_by_size.len() {
            if let Some(&found_idx) = spaces_block_indices_by_size[s].last() {
                if found_idx < right_block_idx && found_idx < best_free_space_block_idx {
                    best_free_space_block_idx = found_idx;
                }
            }
        }

        if best_free_space_block_idx == usize::MAX {
            right_block_idx -= 1;
            continue;
        }

        let left_block_idx = best_free_space_block_idx;
        let left_block = &blocks[left_block_idx];
        spaces_block_indices_by_size[left_block.length as usize].pop();

        let (_, maybe_left_additional_space) = left_block.split(right_block.length);
        if let Some(left_additional_space) = maybe_left_additional_space {
            let new_idx = left_block_idx + 1;
            blocks.insert(left_block_idx, right_block.clone());

            // shift all the indices that are greater than the left block index
            spaces_block_indices_by_size
                .iter_mut()
                .flat_map(|space_list| space_list.iter_mut())
                .filter(|idx| **idx > left_block_idx)
                .for_each(|idx| *idx += 1);

            // add the new space to the list
            let space_list =
                &mut spaces_block_indices_by_size[left_additional_space.length as usize];
            let mut insert_idx = space_list.len();
            for i in (0..space_list.len()).rev() {
                if space_list[i] > new_idx {
                    break;
                }
                insert_idx = i;
            }
            space_list.insert(insert_idx, new_idx);

            blocks[new_idx] = left_additional_space;
            right_block_idx += 1;
        } else {
            blocks[left_block_idx] = right_block.clone();
        }

        blocks[right_block_idx].file_id = None;
        right_block_idx -= 1;
    }

    Some(checksum(&blocks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(60));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2858));
    }

    #[test]
    fn test_part_two_3() {
        let input = "55341271410101";
        let result = part_two(&input);
        assert_eq!(result, Some(638));
    }
}
