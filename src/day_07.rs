use crate::Solution;
use std::collections::VecDeque;

pub struct Day07;

fn get_numbers_and_test_value(lines: &Vec<String>) -> Vec<(Vec<u32>, u64)> {

    lines.iter()
    .map(|l| {
        let split: Vec<&str> = l.split(':').collect();
        let test_value = split[0].parse().unwrap();
        let numbers = split[1].split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
        (numbers, test_value)
        })
        .collect()
}

fn get_calibration_results(equations: &Vec<(Vec<u32>, u64)>, allow_concat: bool) -> <Day07 as Solution>::Item {
    let mut calibration_results = 0;
    for equation in equations {
        let mut nums: VecDeque<u64> = VecDeque::new();
        nums.push_back(equation.0[0] as u64);
        for ix in 1..equation.0.len() {
            let new_num = equation.0[ix] as u64;
            let len = nums.len();
            for _ in 0..len {
                let num = nums.pop_front().unwrap();
                let prod = num * new_num;
                let sm = num + new_num;
                if prod <= equation.1 {
                    nums.push_back(num * new_num);
                }
                if sm <= equation.1 {
                    nums.push_back(num + new_num);
                }
                if allow_concat {
                    let conc = format!("{}{}", num.to_string(), new_num.to_string()).parse::<u64>().unwrap();
                    if conc <= equation.1 {
                        nums.push_back(conc);
                    }
                }

            }
        }
        if nums.contains(&equation.1)  {
            calibration_results += equation.1
        }
    }
    calibration_results
}

impl Solution for Day07 {
    type Item = u64;

    fn day(&self) -> u8 {
        return 7;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, String> { 
        let equations = get_numbers_and_test_value(lines);
        let result: Self::Item = get_calibration_results(&equations, false);
        return Ok(result);
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, String> { 
        let equations = get_numbers_and_test_value(lines);
        let result: Self::Item = get_calibration_results(&equations, true);
        return Ok(result);
    }

}
