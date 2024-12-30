use std::fmt::Display;
use std::time::{Duration, Instant};

pub mod utils;
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_09;
pub mod day_10;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;

pub trait Solution {
    type Item: Display;
    
    fn day(&self) -> u8;

    fn part1(&self, _input: &Vec<String>) -> Result<Self::Item, &str> {
        return Err("Not yet implemented");
    }

    fn part2(&self, _input: &Vec<String>) -> Result<Self::Item, &str> {
        return Err("Not yet implemented");
    }

    fn solve(&self, input: &Vec<String>) -> ((Result<Self::Item, &str>, Duration), (Result<Self::Item, &str>, Duration)) {
        let mut start = Instant::now();
        let part1 = self.part1(input);
        let part1_dur = start.elapsed();
        start = Instant::now();
        let part2 = self.part2(input);
        let part2_dur = start.elapsed();
        ((part1, part1_dur), (part2, part2_dur))
    }

    fn print_res(&self, day: u8, res: ((Result<Self::Item, &str>, Duration), (Result<Self::Item, &str>, Duration))) {
            let parts = vec![res.0, res.1];
            for (part, part_res) in parts.iter().enumerate() {
                match &part_res.0 {
                    Ok(val) => {
                        println!("**AoC 2024 day {} part {}: {} ({} ms) **", 
                        day.to_string(), (part+1).to_string(), val.to_string(), part_res.1.as_millis()); 
                    }
                    Err(msg) => {
                        println!("**AoC 2024 day {} part {}: {} **", 
                        day.to_string(), (part+1).to_string(), msg.to_string());
                    }
                }
            }
    }

    fn run(&self, input: &Vec<String>) {
        let res = self.solve(input);
        self.print_res(self.day(), res);
    }

}