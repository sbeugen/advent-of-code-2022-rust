use crate::input::fetch_input_for_id;
use itertools::Itertools;

pub fn day_6(puzzle: usize) {
    let _input = get_input();
    let _data = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");

    match puzzle {
        1 => puzzle_1(_input.as_str()),
        2 => puzzle_2(_input.as_str()),
        _ => panic!("Puzzle with number {} does not exist", puzzle),
    }
}

fn puzzle_1(input: &str) {
    println!("{}", find_marker(input, 4));
}

fn puzzle_2(input: &str) {
    println!("{}", find_marker(input, 14));
}

fn find_marker(input: &str, length: usize) -> usize {
    for i in (length - 1)..=input.len() {
        let window = &input[i - (length - 1)..=i];
        let unique_window = window.chars().unique().collect_vec();

        if window.len() == unique_window.len() {
            return i + 1;
        }
    }

    0
}

fn get_input() -> String {
    let mut data = fetch_input_for_id(6).expect("Error fetching input data");
    data.pop();
    data
}
