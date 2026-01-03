use core::num;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

/// Runs the solution for Advent of Code Day 3.
pub fn day_3() -> u64 {
    let part_1 = find_total_joltage(true);
    let part_2 = find_total_joltage(false);
    println!("Day 3! Part 1: {:?}, part 2: {:?}", part_1, part_2);
    return part_1;
}

fn find_total_joltage(part_1: bool) -> u64 {
    let input: Vec<String> = split_lines(get_aoc_input(2025, 3));
    let mut all_nums: VecDeque<u64> = VecDeque::new();
    for num in input {
        if part_1 {
            all_nums.push_front(find_largest_number(num));
        } else {
            all_nums.push_front(find_largest_number_variable_length(num, 12));
        }
    }
    return all_nums.iter().sum();
}

fn find_largest_number(num_as_string: String) -> u64 {
    let len: usize = num_as_string.len();
    let mut current_pos: usize = 0;
    let mut largest_substring_num: u64 = 0;
    for first_ch in num_as_string[current_pos..len].chars() {
        for second_ch in num_as_string[current_pos + 1..len].chars() {
            let full_num: u64 = format!("{}{}", first_ch, second_ch)
                .parse::<u64>()
                .unwrap_or(0);
            if full_num > largest_substring_num {
                largest_substring_num = full_num;
            }
        }
        current_pos += 1;
    }
    return largest_substring_num;
}

fn find_largest_number_variable_length(num: String, target_len: usize) -> u64 {
    let mut to_remove = num.len() - target_len;
    let mut stack: Vec<char> = Vec::new();

    for char in num.chars() {
        while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < char {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(char);
    }

    // Remove extra digits from the end if needed
    stack.truncate(target_len);

    return stack.iter().collect::<String>().parse().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_3() {
        assert_eq!(find_total_joltage(), 357);
    }
    #[test]
    fn test_check_range_for_repeats() {
        assert_eq!(find_largest_number("987654321111111".to_string()), 98);
        assert_eq!(find_largest_number("811111111111119".to_string()), 89);
        assert_eq!(find_largest_number("234234234234278".to_string()), 78);
        assert_eq!(find_largest_number("818181911112111".to_string()), 92);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            find_largest_number_variable_length("234234234234278", 12),
            "434234234278"
        )
    }
}
