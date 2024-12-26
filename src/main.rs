use aoc_2024::*;
use aoc_2024::utils;

fn run(day: u8, test_data: bool) {
    let lines = match test_data {
        false => utils::get_input(day),
        true => utils::get_test_input(day)
    };
    match day {
        1 => day_01::Day01.run(&lines),
        2 => day_02::Day02.run(&lines),
        3 => day_03::Day03.run(&lines),
        4 => day_04::Day04.run(&lines),
        5 => day_05::Day05.run(&lines),
        6 => day_06::Day06.run(&lines),
        7 => day_07::Day07.run(&lines),
        9 => day_09::Day09.run(&lines),
        10 => day_10::Day10.run(&lines),
        12 => day_12::Day12.run(&lines),
        13 => day_13::Day13.run(&lines),
        _ => panic!("Unexpected day"),
    };
}

fn main() {
    let test_data: bool = false;
    let day: u8 = 13;
    run(day, test_data);
}
