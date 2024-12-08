use crate::Solution;

pub struct Day01;

impl Solution for Day01 {
    type Item = u32;

    fn day(&self) -> u8 {
        return 1;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let (left, right): (Vec<u32>, Vec<u32>) = lines
        .into_iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut columns = l.split_whitespace().map(|num| num.parse::<u32>().unwrap());
            (columns.next().unwrap(), columns.next().unwrap())
        })
        .unzip();

        let (left_sorted, right_sorted) = (sorted_copy(&left), sorted_copy(&right));
        let result = sum_pairwise_absolute_diffs(left_sorted, right_sorted);
        return Ok(result);
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let result = 11;
        return Ok(result);
    }

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