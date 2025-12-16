use std::fs;

const FILE_PATH: &str = "./inputs/day3.input";

pub fn day3() -> Result<u64, String> {
    let Ok(input) = fs::read_to_string(FILE_PATH) else {
        panic!("Failed to read {} file", FILE_PATH)
    };

    return sum_joltages(input.trim().split('\n').collect());
}

fn sum_joltages(battery_banks: Vec<&str>) -> Result<u64, String> {
    let mut sum: u64 = 0;

    for bank in battery_banks {
        match calc_joltage(bank) {
            Ok(joltage) => sum = sum + joltage,
            Err(message) => {
                return Err(message);
            }
        }
    }

    return Ok(sum);
}

fn calc_joltage(battery_bank: &str) -> Result<u64, String> {
    let chars: Vec<char> = battery_bank.chars().collect();

    let mut max: u32 = 0;
    let mut index = 0;

    for i in 0..(chars.len() - 1) {
        let char = chars[i];
        let Some(digit) = char.to_digit(10) else {
            return Err(String::from("Failed to parse char into digit"));
        };

        if digit > max {
            max = digit;
            index = i;
        }
    }

    let mut trailing_max = 0;
    for i in (index + 1)..chars.len() {
        let char = chars[i];

        let Some(digit) = char.to_digit(10) else {
            return Err(String::from("Failed to parse char into digit"));
        };

        if digit > trailing_max {
            trailing_max = digit;
        }
    }

    return Ok((10 * max + trailing_max) as u64);
}

#[cfg(test)]
mod tests {
    use crate::day3::calc_joltage;

    #[test]
    fn returns_expected_joltage_1() {
        assert_eq!(calc_joltage("987654321111111"), Ok(98));
    }

    #[test]
    fn returns_expected_joltage_2() {
        assert_eq!(calc_joltage("811111111111119"), Ok(89));
    }

    #[test]
    fn returns_expected_joltage_3() {
        assert_eq!(calc_joltage("234234234234278"), Ok(78));
    }

    #[test]
    fn returns_expected_joltage_4() {
        assert_eq!(calc_joltage("818181911112111"), Ok(92));
    }
}
