// https://adventofcode.com/2025/day/2

use std::fs;

use log::info;

const FILE_PATH: &str = "./inputs/day2.input";

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

pub fn day2() -> Result<u64, String> {
    let Ok(input) = fs::read_to_string(FILE_PATH) else {
        panic!("Failed to read {} file", FILE_PATH)
    };

    let Some(line) = input.split('\n').next() else {
        return Err(String::from("Failed to parse first line of input file"));
    };

    let id_ranges = match parse_ids(line) {
        Ok(ir) => ir,
        Err(err) => {
            return Err(err);
        }
    };

    return Ok(sum_invalid_ids(id_ranges));
}

// O(R * L * N^2)
//  R - number of ranges
//  L - length of range
fn sum_invalid_ids(id_ranges: Vec<Range>) -> u64 {
    let mut sum = 0;

    for range in id_ranges {
        for id in range.start..=range.end {
            if !is_valid_id(id) {
                sum = sum + id;
            }
        }
    }

    return sum;
}

// O(N^2) runtime, N is number length
fn is_valid_id(id: u64) -> bool {
    let chars: Vec<char> = id.to_string().chars().collect();
    let num_len = chars.len();

    for window_size in 1..=(num_len / 2) {
        // the window size must divide evenly into the numbers length
        if num_len % window_size != 0 {
            continue;
        }

        let mut is_valid = false;
        let test = &chars[0..window_size];

        for i in (window_size..=(num_len - window_size)).step_by(window_size) {
            if test != &chars[i..(i + window_size)] {
                is_valid = true;
                break;
            }
        }

        if !is_valid {
            info!("'{}' ID is invalid", id);
            return false;
        }
    }

    return true;
}

fn parse_ids(line: &str) -> Result<Vec<Range>, String> {
    let id_ranges = line.split(',');

    let mut ranges = Vec::<Range>::new();
    for id_range in id_ranges {
        let mut split = id_range.split('-');

        let start = split
            .next()
            .ok_or("Failed to read first ID in range")?
            .trim()
            .parse::<u64>()
            .map_err(|_| "Failed to read first ID in range")?;

        let end = split
            .next()
            .ok_or("Failed to read second ID in range")?
            .trim()
            .parse::<u64>()
            .map_err(|_| "Failed to read second ID in range")?;

        ranges.push(Range { start, end });
    }

    return Ok(ranges);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero_when_input_is_empty() {
        assert_eq!(sum_invalid_ids(Vec::new()), 0)
    }

    #[test]
    fn returns_expected_result_for_one_range_1() {
        let input = "11-22";
        let Ok(id_ranges) = parse_ids(input) else {
            panic!("Failed to parse test IDs");
        };

        assert_eq!(id_ranges.len(), 1, "expected number of ID ranges");
        assert_eq!(
            sum_invalid_ids(id_ranges),
            33,
            "expected sum of invalid IDs"
        );
    }

    #[test]
    fn returns_expected_result_for_one_range_2() {
        let input = "998-1012";
        let Ok(id_ranges) = parse_ids(input) else {
            panic!("Failed to parse test IDs");
        };

        assert_eq!(id_ranges.len(), 1, "expected number of ID ranges");
        assert_eq!(
            sum_invalid_ids(id_ranges),
            2009,
            "expected sum of invalid IDs"
        );
    }

    #[test]
    fn returns_expected_result_for_one_range_3() {
        let input = "2121212118-2121212124";
        let Ok(id_ranges) = parse_ids(input) else {
            panic!("Failed to parse test IDs");
        };

        assert_eq!(id_ranges.len(), 1, "expected number of ID ranges");
        assert_eq!(
            sum_invalid_ids(id_ranges),
            2121212121,
            "expected sum of invalid IDs"
        );
    }

    #[test]
    fn returns_expected_result() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let Ok(id_ranges) = parse_ids(input) else {
            panic!("Failed to parse test IDs");
        };

        assert_eq!(id_ranges.len(), 11, "expected number of ID ranges");
        assert_eq!(
            sum_invalid_ids(id_ranges),
            4174379265,
            "expected sum of invalid IDs"
        );
    }
}
