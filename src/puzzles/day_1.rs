use crate::input::fetch_input_for_id;
use itertools::Itertools;

fn get_input() -> String {
    let mut data = fetch_input_for_id(1).expect("Error fetching input data");
    data.pop();
    data
}

pub fn day_1(puzzle: usize) {
    let input = get_input();

    let calories_per_elve = input
        .split("\n\n")
        .map(|elve| elve
            .split("\n")
            .map(|calories| calories.parse::<usize>().unwrap())
            .sum::<usize>()
        ).collect_vec();


    match puzzle {
        1 => println!("Result of day 1 first puzzle: {}", calories_per_elve.iter().max().unwrap()),
        2 => println!("Result of day 1 second puzzle: {}", calories_per_elve.iter().sorted().rev().take(3).sum::<usize>()),
        _ => panic!("Puzzle with number {} does not exist", puzzle)
    }
}
