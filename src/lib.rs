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
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;


pub struct PuzzleResult<T> {
    day: u8,
    part1_res: Result<T, String>,
    part1_dur: Option<Duration>,
    part2_res: Result<T, String>,
    part2_dur: Option<Duration>,
    total_dur: Duration
}

trait ResultPrinter {
    fn print(&self);
}

impl<T: Display> ResultPrinter for PuzzleResult<T> {
    fn print(&self) {
        let part1_res_str = match &self.part1_res {
            Ok(val) => val.to_string(),
            Err(msg) => msg.to_string(),
        };

        let part2_res_str = match &self.part2_res {
            Ok(val) => val.to_string(),
            Err(msg) => msg.to_string(),
        };

        let part1_dur_str = match self.part1_dur {
            Some(dur) => format!("{} ms", dur.as_millis()),
            None => "N/A ms".to_string(),
        };

        let part2_dur_str = match self.part2_dur {
            Some(dur) => format!("{} ms", dur.as_millis()),
            None => "N/A ms".to_string(),
        };

        println!(
            "Day: {}, part 1: {} ({}), part 2: {} ({}), total time elapsed: {} ms",
            self.day,
            part1_res_str,
            part1_dur_str,
            part2_res_str,
            part2_dur_str,
            self.total_dur.as_millis()
        );
    }
}

pub trait Solution {
    type Item: Display;
    
    fn day(&self) -> u8;
    
    fn combine_part1_and_part2(&self) -> bool {
        false
    }

    fn part1(&self, _input: &Vec<String>) -> Result<Self::Item, String> {
        return Err("Not yet implemented".to_string());
    }

    fn part2(&self, _input: &Vec<String>) -> Result<Self::Item, String> {
        return Err("Not yet implemented".to_string());
    }

    fn solve(&self, _input: &Vec<String>) -> Result<(Self::Item, Self::Item), String> {
        return Err("Not yet implemented".to_string());
    }

    fn solve_part_by_part(&self, input: &Vec<String>) -> PuzzleResult<Self::Item> {
        let mut start = Instant::now();
        let part1_res = self.part1(input);
        let part1_dur = start.elapsed();
               
        start = Instant::now();
        let part2_res = self.part2(input);
        let part2_dur = start.elapsed();
        
        PuzzleResult {
            day: self.day(),
            part1_res,
            part1_dur: Some(part1_dur),
            part2_dur: Some(part2_dur),
            part2_res,
            total_dur: part1_dur + part2_dur
        }

    }

    fn solve_both_parts(&self, input: &Vec<String>) -> PuzzleResult<Self::Item> {
        let start = Instant::now();
        let res = self.solve(input);
        let total_dur = start.elapsed();

        let part1_res: Result<Self::Item, String>;
        let part2_res: Result<Self::Item, String>;
        match res {
            Ok(val) => {
                part1_res = Ok(val.0);
                part2_res = Ok(val.1)
            }
            Err(msg) => {
                part1_res = Err(msg.clone());
                part2_res = Err(msg.clone());
            }
        }

        PuzzleResult {
            day: self.day(),
            part1_res,
            part1_dur: None,
            part2_res,
            part2_dur: None,
            total_dur
        }
    }

    fn run(&self, input: &Vec<String>) {
        let res = match self.combine_part1_and_part2() {
            true => self.solve_both_parts(input),
            false => self.solve_part_by_part(input)
        };
        res.print();
    }

}