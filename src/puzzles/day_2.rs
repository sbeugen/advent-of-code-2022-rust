use std::collections::HashMap;
use itertools::Itertools;
use crate::input::fetch_input_for_id;

pub fn day_2(puzzle: usize) {
    let input = get_input();
    let input_lines = input.lines().map(|line| {
        let parts = line.split(" ").collect::<Vec<&str>>();
        (parts[0], parts[1])
    }).collect::<Vec<(&str, &str)>>();


    match puzzle {
        1 => puzzle1(input_lines),
        2 => puzzle2(input_lines),
        _ => panic!("Puzzle with number {} does not exist", puzzle)
    }
}

fn puzzle1(input_lines: Vec<(&str, &str)>) {
    let sum = get_result_sum(input_lines);

    println!("Result puzzle 1: {}", sum);
}

fn puzzle2(input_lines: Vec<(&str, &str)>) {
    let next_commands = get_next_commands();

    let rounds = input_lines.iter()
        .map(|round| (round.0, *next_commands.get(&*format!("{}{}", round.0, round.1)).unwrap())).collect_vec();

    println!("Result puzzle 2: {}", get_result_sum(rounds));
}

fn get_input() -> String {
    let mut data = fetch_input_for_id(2).expect("Error fetching input data");
    data.pop();
    data
}

fn get_result_sum(rounds: Vec<(&str, &str)>) -> i32 {
    let result_combinations = get_result_combinations();
    let win_points = get_win_points();

    rounds
        .iter()
        .map(|round| result_combinations.get(&*format!("{}{}", round.0, round.1)).unwrap() + win_points.get(round.1).unwrap())
        .sum()
}

fn get_result_combinations() -> HashMap<&'static str, i32> {
    let mut result_combinations = HashMap::new();
    result_combinations.insert("AX", 3);
    result_combinations.insert("AY", 6);
    result_combinations.insert("AZ", 0);
    result_combinations.insert("BX", 0);
    result_combinations.insert("BY", 3);
    result_combinations.insert("BZ", 6);
    result_combinations.insert("CX", 6);
    result_combinations.insert("CY", 0);
    result_combinations.insert("CZ", 3);

    result_combinations
}

fn get_win_points() -> HashMap<&'static str, i32> {
    let mut win_points = HashMap::new();
    win_points.insert("X", 1);
    win_points.insert("Y", 2);
    win_points.insert("Z", 3);

    win_points
}

fn get_next_commands() -> HashMap<&'static str, &'static str> {
    let mut next_commands = HashMap::new();
    next_commands.insert("AX", "Z");
    next_commands.insert("AY", "X");
    next_commands.insert("AZ", "Y");
    next_commands.insert("BX", "X");
    next_commands.insert("BY", "Y");
    next_commands.insert("BZ", "Z");
    next_commands.insert("CX", "Y");
    next_commands.insert("CY", "Z");
    next_commands.insert("CZ", "X");

    next_commands
}