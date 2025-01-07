use crate::Solution;
use std::collections::HashMap;

pub struct Day11;

fn to_hashmap(lines: &Vec<String>) -> HashMap<u64, u64> {
    let mut stones: HashMap<u64, u64> = HashMap::new();
    lines
    .iter()
    .for_each(|l| l.split_whitespace()
        .for_each(|val| {
            let stone: u64 = val.parse().unwrap();
            stones.insert(stone, 1);
        })
    );
    stones
}

fn split_digits(val: u64) -> Option<(u32, u32)> {
    let val_as_str = val.to_string();
    let str_len = val_as_str.chars().count();
    if str_len % 2 == 0 {
        let splits = val_as_str.split_at(val_as_str.len()/2);
        return Some((splits.0.parse::<u32>().unwrap(), splits.1.parse::<u32>().unwrap()));
    }
    None
}

fn blink(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones: HashMap<u64, u64> = HashMap::new();
    for (&stone, &count) in stones.iter() {
        if stone == 0 {
            *new_stones.entry(1).or_insert(0) += count;
        } else if let Some(digits) = split_digits(stone) {
            *new_stones.entry(digits.0 as u64).or_insert(0) += count;
            *new_stones.entry(digits.1 as u64).or_insert(0) += count;
        } else {
            *new_stones.entry(stone*2024 ).or_insert(0) += count;
        }
    }
    new_stones
}

fn blink_n_times(n_blinks: u8, stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut stones: HashMap<u64, u64> = stones.clone();
    for _ in 0..n_blinks {
        stones = blink(&stones);
    }
    stones
}

impl Solution for Day11 {
    type Item = u64;

    fn day(&self) -> u8 {
        return 11;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, String> { 
        let mut stones = to_hashmap(lines);
        stones = blink_n_times(25, &stones);
        Ok(stones.values().sum())
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, String> { 
        let mut stones = to_hashmap(lines);
        stones = blink_n_times(75, &stones);
        Ok(stones.values().sum())
    }

}

