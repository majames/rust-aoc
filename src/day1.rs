// https://adventofcode.com/2025/day/1

use std::fs;

const DIAL_SIZE: u16 = 100;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

struct Turn {
    dir: Direction,
    distance: u16,
}

pub fn day1() -> Result<u16, String> {
    let file_path = "./inputs/day1.input";
    let Ok(input) = fs::read_to_string(file_path) else {
        panic!("Failed to read {} file", file_path)
    };

    let lock_turns = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .collect();

    return door_code(lock_turns);
}

fn door_code(lock_turns: Vec<&str>) -> Result<u16, String> {
    let mut crossed_zero_count: u16 = 0;
    let mut dial_position: u16 = 50;

    for turn_str in lock_turns {
        let turn = match parse_direction(turn_str) {
            Ok(t) => t,
            Err(e) => {
                return Err(String::from(e));
            }
        };

        let Turn { dir, distance } = &turn;

        let full_revolution_count = distance / DIAL_SIZE;

        // update crossed zero count
        crossed_zero_count = crossed_zero_count
            + full_revolution_count
            + did_partial_revolution_cross_zero(&turn, dial_position);

        // update the dial position
        dial_position = if dir == &Direction::Right {
            (dial_position + distance).rem_euclid(DIAL_SIZE)
        } else {
            (dial_position as i16 - *distance as i16).rem_euclid(DIAL_SIZE as i16) as u16
        };
    }

    return Ok(crossed_zero_count);
}

fn did_partial_revolution_cross_zero(turn: &Turn, dial_position: u16) -> u16 {
    let Turn {
        dir: direction,
        distance,
    } = turn;

    let remainder = distance % DIAL_SIZE;

    if *direction == Direction::Right && dial_position + remainder >= DIAL_SIZE {
        return 1;
    }

    if *direction == Direction::Left
        && (dial_position as i32) - (remainder as i32) <= 0
        && dial_position != 0
    {
        return 1;
    }

    return 0;
}

fn parse_direction(str: &str) -> Result<Turn, &str> {
    let mut chars = str.trim().chars();

    let Some(dir_char) = chars.next() else {
        return Err("Failed to read direction");
    };

    if dir_char != 'R' && dir_char != 'L' {
        return Err("Invalid direction passed");
    }

    let dir = if dir_char == 'R' {
        Direction::Right
    } else {
        Direction::Left
    };

    let Ok(distance) = chars.as_str().parse::<u16>() else {
        return Err("Failed to parse turn distance");
    };

    return Ok(Turn { dir, distance });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero_when_input_is_empty() {
        assert_eq!(door_code(Vec::new()), Ok(0))
    }

    #[test]
    fn returns_expected_result() {
        let input = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];

        assert_eq!(door_code(input), Ok(6), "number of times zero passed");
    }
}
