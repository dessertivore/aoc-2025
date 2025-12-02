use crate::utils::{get_input::get_aoc_input, parsing::split_string_by_specified_char};
use std::{collections::HashSet, ptr::eq};

pub fn day_2() -> u64 {
    let part_1 = find_all_invalid_ids();
    println!("{:?}", part_1);
    return part_1;
}
fn parse_range(range_str: &str) -> std::ops::Range<u64> {
    let parts: Vec<&str> = range_str.split('-').collect();
    let start: u64 = parts[0].trim_end().parse().expect("Invalid start of range");
    let end: u64 = parts[1].trim_end().parse().expect("Invalid end of range");
    return start..(end + 1);
}

fn repeated_pattern_in_num(num: String) -> Option<String> {
    let chars: Vec<char> = num.chars().collect();
    let mid = chars.len() / 2;
    if &chars[..mid] == &chars[mid..] {
        return Some(num);
    }

    None
}

fn check_range_for_repeats(range_str: &str) -> HashSet<u64> {
    let range = parse_range(range_str);
    let mut results: HashSet<u64> = HashSet::new();
    for x in range {
        let repeats = repeated_pattern_in_num(x.to_string());
        results.extend(repeats.into_iter().map(|s| s.parse::<u64>().unwrap()));
    }
    return results;
}

fn find_all_invalid_ids() -> u64 {
    let input = split_string_by_specified_char(get_aoc_input(2025, 2), ",");
    let mut total: Vec<u64> = Vec::new();
    for item in input {
        let results = check_range_for_repeats(&item);
        total.extend(results);
    }
    return total.iter().fold(0, |acc, x| acc + x);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_2() {
        assert_eq!(day_2(), 1227775554);
    }
    #[test]
    fn test_check_range_for_repeats() {
        assert_eq!(check_range_for_repeats("11-22"), HashSet::from([11, 22]));
    }
    #[test]
    fn test_check_range_for_repeats_2() {
        assert_eq!(check_range_for_repeats("2-17"), HashSet::from([11]));
    }
}
