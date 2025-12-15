// https://adventofcode.com/2025/day/1
const LOCK_SIZE: u16 = 100;

pub fn day1_door_code(lock_turns: Vec<&str>) -> Result<u16, &str> {
    let mut code: u16 = 0;
    let mut lock_position: u16 = 50;

    for turn in lock_turns {
        let mut chars = turn.trim().chars();

        let Some(dir) = chars.next() else {
            return Err("Failed to get turn direction");
        };

        let Ok(distance) = chars.as_str().parse::<u16>() else {
            return Err("Failed to parse turn distance");
        };

        lock_position = if dir == 'R' {
            (lock_position + distance).rem_euclid(LOCK_SIZE)
        } else {
            (lock_position as i16 - distance as i16).rem_euclid(LOCK_SIZE as i16) as u16
        };

        if lock_position == 0 {
            code = code + 1;
        }
    }

    return Ok(code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero_when_input_is_empty() {
        assert_eq!(day1_door_code(Vec::new()), Ok(0))
    }

    #[test]
    fn returns_expected_result() {
        let input = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];

        assert_eq!(
            day1_door_code(input),
            Ok(3),
            "Expected number of left facing zeros to be 3"
        );
    }
}
