// https://adventofcode.com/2025/day/2

use std::fs;

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

// O(R * L) runtime
//   - R = number of ranges
//   - L = range length
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

// O(1) runtime
fn is_valid_id(id: u64) -> bool {
    let num_len = id.to_string().len();
    let base: u64 = 10;

    if is_odd(num_len) {
        // if ID length is odd it can never be repeated
        return true;
    }

    // split number into "back" and "front" halves
    let factor = base.pow((num_len / 2) as u32);
    let back = id % factor;
    let front = id / factor;

    return front != back;
}

fn is_odd(num: usize) -> bool {
    return num % 2 == 1;
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
    fn returns_expected_result() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let Ok(id_ranges) = parse_ids(input) else {
            panic!("Failed to parse test IDs");
        };

        assert_eq!(id_ranges.len(), 11, "expected number of ID ranges");
        assert_eq!(
            sum_invalid_ids(id_ranges),
            1227775554,
            "expected sum of invalid IDs"
        );
    }
}
