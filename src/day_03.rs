use crate::Solution;
use regex::Regex;

pub struct Day03;

impl Solution for Day03 {
    type Item = u32;

    fn day(&self) -> u8 {
        return 3;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 

        let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
        let mut factors: Vec<(u32, u32)> = Vec::new();
        for line in lines {
            let tmp: Vec<(u32, u32)> = re.captures_iter(line).map(|caps| {
                let (_, [first, second]) = caps.extract();
                (first.parse::<u32>().unwrap(), second.parse::<u32>().unwrap())
            }).collect();
            factors.extend(tmp);
        }
    
        let result = factors
        .into_iter()
        .map(|(first, second)| first * second)
        .sum();

        return Ok(result);

    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        return Ok(3);
    }

}
