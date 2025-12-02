use crate::utils::{get_input::get_aoc_input, parsing::split_string_by_specified_char};
use std::collections::HashSet;

/// Runs the solution for Advent of Code Day 2.
///
/// Computes both Part 1 and Part 2 by searching for invalid IDs according
/// to two different rule sets. Prints the results and returns them as a tuple.
///
/// # Returns
/// A tuple `(part_1, part_2)` where each value is the sum of all invalid IDs
/// detected for the corresponding part.
pub fn day_2() -> (u64, u64) {
    let part_1 = find_all_invalid_ids(1);
    let part_2 = find_all_invalid_ids(2);
    println!("Day 2! Part 1: {:?}, part 2: {:?}", part_1, part_2);
    return (part_1, part_2);
}

/// Parses a numeric range of the form `"start-end"` into a `Range<u64>`.
///
/// The returned range is **inclusive**, meaning the upper bound is
/// incremented by 1 to support Rust's half-open `start..end` syntax.
///
/// # Parameters
/// * `range_str` – A string slice containing the range, e.g. `"5-10"`.
///
/// # Panics
/// Panics if the input is malformed or cannot be parsed into integers.
fn parse_range(range_str: &str) -> std::ops::Range<u64> {
    let parts: Vec<&str> = range_str.split('-').collect();
    let start: u64 = parts[0].trim_end().parse().expect("Invalid start of range");
    let end: u64 = parts[1].trim_end().parse().expect("Invalid end of range");
    return start..(end + 1);
}

/// Checks whether the first half of a string is exactly equal to the second half.
///
/// This detects simple repeated patterns of length 2, such as:
/// - `"1212"`
/// - `"9999"`
/// - `"4444"`
///
/// # Arguments
/// * `num` – A string representing the numeric value.
///
/// # Returns
/// * `Some(num)` if the number consists of two identical halves.
/// * `None` otherwise.
///
fn first_half_is_second_half_of_num(num: String) -> Option<String> {
    let chars: Vec<char> = num.chars().collect();
    let mid = chars.len() / 2;
    if &chars[..mid] == &chars[mid..] {
        return Some(num);
    }
    None
}

/// Attempts to determine whether a numeric string is composed of a repeating substring
/// of any length.
///
/// For example:
/// - `"121212"` → repeated substring `"12"`
/// - `"777"` → repeated substring `"7"`
/// - `"5050"` → repeated substring `"50"`
///
/// # Arguments
/// * `num` – A numeric string to analyze.
///
/// # Returns
/// * `Some(value_as_u64)` if the entire string is a perfect repetition.
/// * `None` if no repeating pattern is found.
fn find_repeats_in_num(num: String) -> Option<u64> {
    let n = num.len();

    for len in 1..=n / 2 {
        // The substring length must divide the whole string length
        if n % len != 0 {
            continue;
        }
        let pattern: &str = &num[..len];

        // Check if repeating the pattern reconstructs the full string
        if pattern.repeat(n / len) == num {
            return Some(str::parse::<u64>(&num).expect("Could not convert to u64 >:("));
        }
    }

    None
}

/// Evaluates all numbers in a numeric range and collects those that match
/// specific repeat-pattern rules.
///
/// ## Part Behavior
/// - **Part 1:** Includes numbers whose *first half equals the second half*.
/// - **Part 2:** Also includes numbers that are *any length repeating pattern*.
///
/// # Arguments
/// * `range_str` – A string like `"1000-2000"` defining the search range.
/// * `part` – The puzzle part (`1` or `2`) determining which rules apply.
///
/// # Returns
/// A `HashSet<u64>` containing all numbers in the range that meet the criteria.
///
fn check_range_for_repeats(range_str: &str, part: u8) -> HashSet<u64> {
    let range = parse_range(range_str);
    let mut results: HashSet<u64> = HashSet::new();
    let mut part_2: HashSet<u64> = HashSet::new();
    for x in range {
        let repeats: Option<String>;
        repeats = first_half_is_second_half_of_num(x.to_string());
        if part == 2 {
            if let Some(part_2_resp) = find_repeats_in_num(x.to_string()) {
                part_2.insert(part_2_resp);
            }
        }
        results.extend(repeats.into_iter().map(|s| s.parse::<u64>().unwrap()));
    }
    if part == 2 {
        results.extend(part_2);
    }
    return results;
}

/// Processes all comma-separated ranges from the input and sums
/// all invalid IDs found according to the specified puzzle part.
///
/// This function is responsible for:
/// - Splitting the input into ranges
/// - Applying repeat-pattern checking
/// - Collecting all unique invalid IDs
/// - Summing them into a final answer
///
/// # Arguments
/// * `part` – Either `1` or `2`, selecting the validation rule set.
///
/// # Returns
/// A `u64` representing the sum of all invalid IDs.
fn find_all_invalid_ids(part: u8) -> u64 {
    let input = split_string_by_specified_char(get_aoc_input(2025, 2), ",");
    let mut total: HashSet<u64> = HashSet::new();
    for item in input {
        let results = check_range_for_repeats(&item, part);
        total.extend(results);
    }
    return total.iter().fold(0, |acc, x| acc + x);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_2() {
        assert_eq!(find_all_invalid_ids(1), 1227775554);
        assert_eq!(find_all_invalid_ids(2), 4174379265);
    }
    #[test]
    fn test_check_range_for_repeats() {
        assert_eq!(check_range_for_repeats("11-22", 1), HashSet::from([11, 22]));
    }
    #[test]
    fn test_check_range_for_repeats_2() {
        assert_eq!(check_range_for_repeats("2-17", 1), HashSet::from([11]));
    }
    #[test]
    fn test_find_repeats_in_num() {
        assert_eq!(check_range_for_repeats("11-12", 2), HashSet::from([11]));
    }

    #[test]
    fn test_find_repeats_in_num_2() {
        assert_eq!(check_range_for_repeats("111-114", 2), HashSet::from([111]));
    }
}
