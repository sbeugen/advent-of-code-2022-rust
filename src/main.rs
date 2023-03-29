extern crate core;

use std::env;
use std::time::Instant;


mod input;
mod puzzles;

fn main() {
    let start = Instant::now();

    let day = env::var("AOC_DAY").expect("Env variable AOC_DAY does not exist.").parse::<usize>().unwrap();
    let puzzle = env::var("AOC_PUZZLE").expect("Env variable AOC_PUZZLE does not exist.").parse::<usize>().unwrap();

    match day {
        1 => puzzles::day_1::day_1(puzzle),
        2 => puzzles::day_2::day_2(puzzle),
        3 => puzzles::day_3::day_3(puzzle),
        4 => puzzles::day_4::day_4(puzzle),
        6 => puzzles::day_6::day_6(puzzle),
        _ => panic!("Day {} not implemented yet", day)
    }

    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);
}
