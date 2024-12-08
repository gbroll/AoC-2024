use AoC_2024::*;
use AoC_2024::utils;

fn run(day: u8) {
    let lines = utils::get_input(day);
    match day {
        1 => day_01::Day01.run(&lines),
        _ => panic!("Unexpected day"),
    };
}

fn main() {
    let day: u8 = 1;
    run(day);
}
