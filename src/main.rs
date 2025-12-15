use std::fs;

use advent_of_code::day1_door_code;

fn main() {
    let file_path = "./inputs/day1.input";
    let Ok(input) = fs::read_to_string(file_path) else {
        panic!("Failed to read {} file", file_path)
    };

    let lock_turns = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .collect();

    let result = day1_door_code(lock_turns);
    println!("The code is: {:?}", result);
}
