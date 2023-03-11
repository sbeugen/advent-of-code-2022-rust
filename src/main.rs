extern crate core;

use std::env;

mod input;
mod puzzles;

fn main() {
    let day = env::var("AOC_DAY").expect("Env variable AOC_DAY does not exist.").parse::<usize>().unwrap();
    let puzzle = env::var("AOC_PUZZLE").expect("Env variable AOC_PUZZLE does not exist.").parse::<usize>().unwrap();

    match day {
        1 => puzzles::day_1::day_1(puzzle),
        2 => puzzles::day_2::day_2(puzzle),
        3 => puzzles::day_3::day_3(puzzle),
        _ => panic!("Day {} not implemented yet", day)
    }
}
