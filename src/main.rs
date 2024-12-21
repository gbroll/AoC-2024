use AoC_2024::*;
use AoC_2024::utils;

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
        _ => panic!("Unexpected day"),
    };
}

fn main() {
    let test_data: bool = false;
    let day: u8 = 9;
    run(day, test_data);
}
