use std::fmt::Debug;

advent_of_code::solution!(9);

#[derive(Debug, Clone)]
struct Block {
    length: u32,
    file_id: Option<u32>,
}

impl Block {
    fn is_file(&self) -> bool {
        self.file_id.is_some()
    }

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

struct BlocksFmt<'a>(&'a Vec<Block>);

impl Debug for BlocksFmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.0 {
            for _ in 0..block.length {
                match block.file_id {
                    Some(id) => write!(f, "{}", id)?,
                    None => write!(f, ".")?,
                }
            }
        }
        Ok(())
    }
}

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

    let mut spaces_block_indices: Vec<usize> = Vec::with_capacity(1000);
    for (i, b) in blocks.iter().enumerate() {
        if b.file_id.is_none() {
            spaces_block_indices.push(i);
        }
    }

    let mut right_block_idx = blocks.len() - 1;
    while right_block_idx > 0 {
        let right_block = &blocks[right_block_idx];
        if right_block.is_free() {
            right_block_idx -= 1;
            continue;
        }

        // look for a free space block that can fit the file
        let mut free_space_block_idx = None;
        for (i, space_idx) in spaces_block_indices.iter().enumerate() {
            if *space_idx > right_block_idx {
                break;
            }
            let left_block = &blocks[*space_idx];
            if left_block.length > right_block.length {
                free_space_block_idx = Some(*space_idx);
                for i in i..spaces_block_indices.len() {
                    spaces_block_indices[i] += 1;
                }
                break;
            } else if left_block.length == right_block.length {
                free_space_block_idx = Some(*space_idx);
                spaces_block_indices.remove(i);
                break;
            }
        }

        if let Some(left_block_idx) = free_space_block_idx {
            let left_block = &blocks[left_block_idx];
            let (_, maybe_left_additional_space) = left_block.split(right_block.length);

            if let Some(left_additional_space) = maybe_left_additional_space {
                blocks.insert(left_block_idx, right_block.clone());
                blocks[left_block_idx + 1] = left_additional_space;
                right_block_idx += 1;
            } else {
                blocks[left_block_idx] = right_block.clone();
            }

            blocks[right_block_idx].file_id = None;
            right_block_idx -= 1;
        }

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
}
