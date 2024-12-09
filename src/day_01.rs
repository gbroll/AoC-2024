use crate::Solution;
use std::collections::HashMap;

pub struct Day01;

impl Solution for Day01 {
    type Item = u32;

    fn day(&self) -> u8 {
        return 1;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 

        let (left, right) = get_lists(lines);
        
        let (left_sorted, right_sorted) = (sorted_copy(&left), sorted_copy(&right));
        let result = sum_pairwise_absolute_diffs(left_sorted, right_sorted);
        return Ok(result);
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let (left, right) = get_lists(lines);
        let mut num_appearances: HashMap<u32,u32> = HashMap::new();
        for val in right.into_iter() {
            *num_appearances.entry(val).or_insert(0) += 1;
        }

        let result: u32 = left
        .into_iter()
        .map(|val| val * num_appearances.get(&val).copied().unwrap_or(0))
        .sum();

        return Ok(result);
    }

}

fn get_lists(lines: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let (left, right): (Vec<u32>, Vec<u32>) = lines
    .into_iter()
    .filter(|l| !l.trim().is_empty())
    .map(|l| {
        let mut columns = l.split_whitespace().map(|num| num.parse::<u32>().unwrap());
        (columns.next().unwrap(), columns.next().unwrap())
    })
    .unzip();
    (left, right)
}

fn sorted_copy(input: &Vec<u32>) -> Vec<u32> {
    let mut output = input.clone();
    output.sort();
    output
}

fn sum_pairwise_absolute_diffs(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let abs_diff_sum: i32 = left.iter()
    .zip(right.iter())
    .map(|(&l, &r)| (l as i32 - r as i32).abs())
    .sum();
    return abs_diff_sum as u32
}