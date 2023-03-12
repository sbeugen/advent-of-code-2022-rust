use itertools::Itertools;
use crate::input::fetch_input_for_id;

pub fn day_4(puzzle: usize) {
    let _input = get_input();
    let _data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    let lines = _input.lines().map(|line| {
        let split: (&str, &str) = line.split(",").collect_tuple().unwrap();
        (Range::from_range_string(split.0), Range::from_range_string(split.1))
    }).collect_vec();


    match puzzle {
        1 => puzzle_1(lines),
        2 => puzzle_2(lines),
        _ => panic!("Puzzle with number {} does not exist", puzzle)
    }
}

fn get_input() -> String {
    let mut data = fetch_input_for_id(4).expect("Error fetching input data");
    data.pop();
    data
}

fn puzzle_1(pairs: Vec<(Range, Range)>) {
    let result = pairs.iter().map(|(first, second)| {
        if (first.start >= second.start && first.end <= second.end) || (second.start >= first.start && second.end <= first.end) {
            1
        } else {
            0
        }
    }).sum::<usize>();

    println!("{}", result);
}

fn puzzle_2(pairs: Vec<(Range, Range)>) {
    let result = pairs.iter().map(|(first, second)| {
        if (first.start >= second.start && first.start <= second.end) ||
            (first.end >= second.start && first.end <= second.end) ||
            (second.start >= first.start && second.start <= first.end) ||
            (second.end >= first.start && second.end <= first.end) {
            1
        } else {
            0
        }
    }).sum::<usize>();
    println!("{}", result);
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
        }
    }

    fn from_range_string(range_str: &str) -> Self {
        let (start_str, end_str): (&str, &str) = range_str.split("-").collect_tuple().unwrap();
        let start = start_str.parse::<u32>().unwrap();
        let end = end_str.parse::<u32>().unwrap();

        Range::new(start, end)
    }
}