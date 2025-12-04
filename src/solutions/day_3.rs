use core::num;
use std::collections::{HashSet, VecDeque};

use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

/// Runs the solution for Advent of Code Day 3.
pub fn day_3() -> u64 {
    let part_1 = find_total_joltage();
    println!("Day 3! Part 1: {:?}", part_1);
    return part_1;
}

fn find_total_joltage() -> u64 {
    let input: Vec<String> = split_lines(get_aoc_input(2025, 3));
    let mut all_nums: VecDeque<u64> = VecDeque::new();
    for num in input {
        all_nums.push_front(find_largest_number(num));
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

// fn find_largest_number_recursive(
//     num_as_string: String,
//     remaining_chars: String,
//     max_num_len: usize,
//     largest_substring_num: u64,
// ) -> Option<u64> {
//     if num_as_string.len() == max_num_len {
//         let final_num: u64 = num_as_string.parse::<u64>().unwrap_or(0);
//         if final_num > largest_substring_num {
//             return Some(final_num);
//         } else {
//             None
//         }
//     } else {
//         for second_ch in remaining_chars.chars() {
//             find_largest_number_recursive()
//         }
//     }
// }

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
}
