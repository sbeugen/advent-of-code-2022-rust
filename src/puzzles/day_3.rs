use std::collections::HashMap;
use itertools::Itertools;
use crate::input::fetch_input_for_id;

pub fn day_3(puzzle: usize) {
    let input = get_input();
    let _data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    let lines = input.lines().collect_vec();


    match puzzle {
        1 => puzzle_1(lines),
        2 => puzzle_2(lines),
        _ => panic!("Puzzle with number {} does not exist", puzzle)
    }
}

fn get_input() -> String {
    let mut data = fetch_input_for_id(3).expect("Error fetching input data");
    data.pop();
    data
}

fn puzzle_1(rucksacks: Vec<&str>) {
    let result = rucksacks.iter().map(|rucksack| {
        let compartments = rucksack.split_at(rucksack.len() / 2);
        let first = compartments.0.split("").sorted().dedup().join("");
        let second = compartments.1.split("").sorted().dedup().collect_vec().join("");

        let filtered_rucksack = first + second.as_str();

        let counts_per_letter = get_counts_per_letter(&filtered_rucksack);

        let duplicate_letter = counts_per_letter.iter().find(|entry| *entry.1 > 1);

        if duplicate_letter.is_some() {
            get_letter_priority(duplicate_letter.unwrap().0)
        } else {
            0
        }
    }).sum::<u32>();

    println!("{}", result);
}

fn puzzle_2(rucksacks: Vec<&str>) {
    let deduped_rucksacks = rucksacks.iter().map(|rucksack| rucksack.split("").sorted().dedup().join("")).collect_vec();
    let grouped_rucksacks = deduped_rucksacks.chunks(3).map(|chunks| chunks.join("")).collect_vec();

    let result = grouped_rucksacks.iter().map(|rucksack| {
        let counts_per_letter = get_counts_per_letter(rucksack);

        let duplicate_letter = counts_per_letter.iter().find(|entry| *entry.1 == 3);

        if duplicate_letter.is_some() {
            get_letter_priority(duplicate_letter.unwrap().0)
        } else {
            0
        }
    }).sum::<u32>();

    println!("{}", result);
}

fn get_letter_priority(letter: &str) -> u32 {
    let ascii_value = letter.chars().next().unwrap() as u32;

    if letter.to_lowercase() == letter {
        ascii_value - 96
    } else {
        ascii_value - 38
    }
}

fn get_counts_per_letter(rucksack: &String) -> HashMap<String, usize> {
    let mut counts_per_letter = HashMap::new();
    let split = rucksack.split("").filter(|l| !l.is_empty());

    split.for_each(|letter| {
        if counts_per_letter.contains_key(letter) {
            counts_per_letter.insert(letter.to_string(), counts_per_letter.get(letter).unwrap() + 1);
        } else {
            counts_per_letter.insert(letter.to_string(), 1);
        }
    });

    counts_per_letter
}