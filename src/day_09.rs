use crate::Solution;
use std::collections::VecDeque;

pub struct Day09;


fn to_q(lines: &Vec<String>) -> VecDeque<u8> {
    lines
    .iter()
    .flat_map(|l| l.chars() 
        .map(|c| c.to_digit(10).unwrap() as u8)
    )
    .collect()
}

fn calc_checksum(fs: &Vec<u32>) -> u64 {
    fs
    .iter()
    .enumerate()
    .map(|(pos, id)| (pos as u64 * *id as u64))
    .sum()
}

impl Solution for Day09 {
    type Item = u64;

    fn day(&self) -> u8 {
        return 1;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 

        let mut input_q = to_q(lines);
        let input_len = input_q.len();
        let mut fs: Vec<u32> = Vec::new();
        let mut front_ix = 0;
        let mut rear_ix = input_len as u32- 1;
        let mut rear_data: VecDeque<u32> = VecDeque::new();
        while !input_q.is_empty() {
            let val = input_q.pop_front().unwrap() as usize;
            if front_ix % 2 == 0 {
                fs.extend(std::iter::repeat(front_ix/2).take(val));
            } else {
                // free space to be filled with data from the rear
                let mut gap_len = val;
                while gap_len > 0 {
                    while rear_data.is_empty() {
                        let read_val = input_q.pop_back().unwrap();
                        if rear_ix % 2 == 0 {
                            rear_data.extend(std::iter::repeat(rear_ix / 2).take(read_val as usize));
                        } else {
                            // free space at the rear, do nothinh
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
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let result = 3;
        return Ok(result);
    }

}

