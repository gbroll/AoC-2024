use crate::Solution;
use std::collections::VecDeque;

pub struct Day09;

#[derive(Debug, Clone, Copy)]
struct Block {
    is_gap: bool,
    length: u8,
    file_id: u32
}

fn to_block_q(lines: &Vec<String>) -> VecDeque<Block> {
    let mut counter = 0;
    lines
        .iter()
        .flat_map(|l| {
            l.chars().map(move|c| {
                let block = Block {
                    is_gap: counter % 2 != 0,
                    length: c.to_digit(10).unwrap() as u8,
                    file_id: counter / 2
                };
                counter += 1;
                block      
            })
        })
        .collect()
}

fn to_q(lines: &Vec<String>) -> VecDeque<u8> {
    lines
        .iter()
        .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8))
        .collect()
}

fn blocks_to_fs(blocks: &Vec<Block>) -> Vec<u32> {
    let mut fs: Vec<u32> = Vec::new();
    for b in blocks {
        let val = match b.is_gap {
            true => 0,
            false => b.file_id
        };
        fs.extend(std::iter::repeat(val).take(b.length as usize).collect::<Vec<u32>>());
    }
    fs
}

fn calc_checksum(fs: &Vec<u32>) -> u64 {
    fs.iter()
        .enumerate()
        .map(|(pos, id)| (pos as u64 * *id as u64))
        .sum()
}

impl Solution for Day09 {
    type Item = u64;

    fn day(&self) -> u8 {
        return 9;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, String> {
        let mut input_q = to_q(lines);
        let input_len = input_q.len();
        let mut fs: Vec<u32> = Vec::new();
        let mut front_ix = 0;
        let mut rear_ix = input_len as u32 - 1;
        let mut rear_data: VecDeque<u32> = VecDeque::new();
        while !input_q.is_empty() {
            let val = input_q.pop_front().unwrap() as usize;
            if front_ix % 2 == 0 {
                fs.extend(std::iter::repeat(front_ix / 2).take(val));
            } else {
                // free space to be filled with data from the rear
                let mut gap_len = val;
                while gap_len > 0 {
                    while rear_data.is_empty() {
                        let read_val = input_q.pop_back().unwrap();
                        if rear_ix % 2 == 0 {
                            rear_data
                                .extend(std::iter::repeat(rear_ix / 2).take(read_val as usize));
                        } else {
                            // free space at the rear, do nothing
                        }
                        rear_ix -= 1;
                    }
                    fs.push(rear_data.pop_front().unwrap());
                    gap_len -= 1;
                }
            }
            front_ix += 1;
        }
        // Flush rear data
        while !rear_data.is_empty() {
            fs.push(rear_data.pop_front().unwrap());
        }
        Ok(calc_checksum(&fs))
    }

    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, String> {
        let mut input = to_block_q(lines);
        let input_len = input.len();
        let mut output: Vec<Block> = Vec::new();
        let mut front_ix: usize = 0;

        while front_ix < input.len() {
           
            let mut front_block = input[front_ix].clone();
            if !front_block.is_gap && front_block.length > 0 {
                output.push(front_block.clone());
            } else if front_block.length > 0 {
                // Free space here
                let mut rear_ix = input_len - 1;
                while rear_ix > front_ix {

                        let mut rear_block = input[rear_ix].clone();
                        if !rear_block.is_gap && rear_block.length <= front_block.length {
                            front_block.length -= rear_block.length;
                            output.push(rear_block.clone());
                            rear_block.is_gap = true;
                            input[front_ix] = front_block;
                            input[rear_ix] = rear_block;
                        }
                    rear_ix -= 1;
                    if front_block.length == 0 {
                        break;
                    }
                }
                if front_block.length > 0 {
                    output.push(front_block);
                }
            }
            front_ix += 1;
        }
        let fs = blocks_to_fs(&output);
        Ok(calc_checksum(&fs))
    }
}

