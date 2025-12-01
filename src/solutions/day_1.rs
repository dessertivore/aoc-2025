use std::thread::current;

use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

pub fn day_1() -> (i16, i16) {
    let input = split_lines(get_aoc_input(2025, 1));
    let at_zero: i16;
    let total: i16;
    (at_zero, total) = put_it_together(input);
    return (at_zero, total);
}
fn put_it_together(input: Vec<String>) -> (i16, i16) {
    let mut current_pos: i16 = 50;
    let mut at_zero: i16 = 0;
    let mut times_passed_0: i16 = 0;
    for instruction in input {
        let mut times_passed_0_this_round = 0;
        (times_passed_0_this_round, current_pos) = move_dial(instruction, current_pos);
        times_passed_0 += times_passed_0_this_round;
        if current_pos == 0 {
            at_zero += 1
        }
    }
    println!("Day 1! Part 1: {:?}, Part 2: {:?}", at_zero, times_passed_0);
    return (at_zero, times_passed_0);
}

fn move_dial(instruction: String, mut current_pos: i16) -> (i16, i16) {
    let mut at_zero: i16 = 0;
    let mut times_passed_0: i16 = 0;

    println!(
        "{:?},{:?},{:?},{:?}",
        current_pos, instruction, times_passed_0, at_zero
    );
    let start_pos: i16 = current_pos;
    let direction = &instruction[..1];
    let how_far: i16 =
        str::parse::<i16>(&instruction[1..]).expect("Unable to convert how_far to number");
    if direction == "L" {
        current_pos -= how_far;
    } else if direction == "R" {
        current_pos += how_far
    } else {
        panic!("Invalid direction!")
    }
    if !(0..100).contains(&current_pos) {
        if current_pos < 0 {
            while current_pos < 0 {
                current_pos += 100;
                times_passed_0 += 1;
            }
        } else if current_pos > 99 {
            while current_pos > 99 {
                current_pos -= 100;
                times_passed_0 += 1;
            }
        }
    }
    if current_pos == 0 {
        at_zero += 1;
    }
    println!(
        "{:?},{:?},{:?},{:?}",
        current_pos, instruction, times_passed_0, at_zero
    );

    return (times_passed_0, current_pos);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_1() {
        assert_eq!(day_1(), (3, 6));
    }

    #[test]
    fn test_1() {
        // 50 -> 99 -> 01 without crossing zero
        let (at_zero, total) = put_it_together(vec!["R49".to_string(), "L98".to_string()]);
        assert_eq!(total, 0);
    }

    #[test]
    fn test_2() {
        // 50 -> 99 -> 00 ending up at zero
        let (at_zero, total) = put_it_together(vec!["R49".to_string(), "R1".to_string()]);
        assert_eq!(total, 1);
    }

    #[test]
    fn test_3() {
        // 50 -> 99 -> 00 -> 01 stopping at zero once
        let (at_zero, total) =
            put_it_together(vec!["R49".to_string(), "R1".to_string(), "R1".to_string()]);
        assert_eq!(total, 1);
    }

    #[test]
    fn test_4() {
        // 50 -> 01 -> 00 -> 99 stopping at zero once
        let (at_zero, total) =
            put_it_together(vec!["R49".to_string(), "R1".to_string(), "L1".to_string()]);
        assert_eq!(total, 1);
    }

    #[test]
    fn test_5() {
        // 50 -> 00 -> and a full rotation ending up at 00 again
        let (at_zero, total) = put_it_together(vec!["L50".to_string(), "L100".to_string()]);
        assert_eq!(total, 2);
    }

    #[test]
    fn test_6() {
        // 50 -> 00 -> and a full rotation ending up at 00 again
        let (at_zero, total) = put_it_together(vec!["R50".to_string(), "R100".to_string()]);
        assert_eq!(total, 2);
    }

    // #[test]
    // fn test_7() {
    //     // 50 -> 00 -> and 4 full rotations ending up at 00 again
    //     let count = move_dial(&[('L', 50), ('L', 400)]);
    //     assert_eq!(count, 5);
    // }

    // #[test]
    // fn test_8() {
    //     // 50 -> 00 -> and 4 full rotations ending up at 00 again
    //     let count = move_dial(&[('L', 50), ('R', 400)]);
    //     assert_eq!(count, 5);
    // }

    // #[test]
    // fn test_9() {
    //     // 50 and 10 full rotations ending up at 50 again
    //     let count = move_dial(&[('R', 1000)]);
    //     assert_eq!(count, 10);
    // }
}
