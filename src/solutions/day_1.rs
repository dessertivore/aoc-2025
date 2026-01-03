use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

pub fn day_1() {
    println!("Part 1: {:?}", move_dial(None, false));
    println!("Part 2: {:?}", move_dial(None, true));
}

fn move_dial(input: Option<Vec<String>>, part_2: bool) -> i16 {
    let input = input.unwrap_or_else(|| split_lines(get_aoc_input(2025, 1)));

    let mut current_pos: i16 = 50;
    let mut at_zero: i16 = 0;
    let mut pass_zero: i16 = 0;

    for instruction in input {
        let direction = &instruction[..1];
        let how_far: i16 = instruction[1..].parse().unwrap();
        let prev_pos = current_pos;
        if direction == "L" {
            current_pos -= how_far;
        } else if direction == "R" {
            current_pos += how_far
        } else {
            panic!("Invalid direction!")
        }

        if part_2 {
            let mut passes = 0;
            let mut pos = prev_pos;
            let step = if direction == "R" { 1 } else { -1 };

            for _ in 0..how_far {
                pos = (pos + step).rem_euclid(100);
                if pos == 0 {
                    passes += 1;
                }
            }

            pass_zero += passes;
        }

        // wrap number around dial
        current_pos %= 100;
        if current_pos == 0 {
            at_zero += 1;
        }
    }

    if part_2 {
        pass_zero
    } else {
        at_zero
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_dial() {
        assert_eq!(move_dial(None, false), 3);
    }

    #[test]
    fn part_1_not_crossing_zero() {
        // 50 -> 99 -> 01 without crossing zero
        let total = move_dial(vec!["R49".to_string(), "L98".to_string()].into(), false);
        assert_eq!(total, 0);
    }

    #[test]
    fn part_1_ending_on_zero() {
        // 50 -> 99 -> 00 ending up at zero
        let total = move_dial(vec!["R49".to_string(), "R1".to_string()].into(), false);
        assert_eq!(total, 1);
    }

    #[test]
    fn part_1_all_r_stopping_at_zero_once() {
        // 50 -> 99 -> 00 -> 01 stopping at zero once
        let total = move_dial(
            vec!["R49".to_string(), "R1".to_string(), "R1".to_string()].into(),
            false,
        );
        assert_eq!(total, 1);
    }

    #[test]
    fn part_1_1_l_stopping_at_zero_once() {
        // 50 -> 01 -> 00 -> 99 stopping at zero once
        let total = move_dial(
            vec!["R49".to_string(), "R1".to_string(), "L1".to_string()].into(),
            false,
        );
        assert_eq!(total, 1);
    }

    #[test]
    fn part_2_pass_zero_once() {
        // 50 -> 99 -> and a full rotation ending up at 99 again
        let total = move_dial(vec!["L50".to_string(), "R50".to_string()].into(), true);
        assert_eq!(total, 1);
    }
    #[test]
    fn part_2_pass_zero_once_2() {
        // 50 -> 99 -> and a full rotation ending up at 99 again
        let total = move_dial(vec!["L50".to_string(), "L50".to_string()].into(), true);
        assert_eq!(total, 1);
    }
    #[test]
    fn part_2_pass_zero_twice() {
        // 50 -> 99 -> and a full rotation ending up at 99 again
        let total = move_dial(vec!["L51".to_string(), "L100".to_string()].into(), true);
        assert_eq!(total, 2);
    }
    #[test]
    fn part_2_pass_zero_twice_2() {
        // 50 -> 0 (via 0) -> 50
        let total = move_dial(vec!["L150".to_string(), "R50".to_string()].into(), true);
        assert_eq!(total, 2);
    }
    #[test]
    fn part_2_pass_zero_twice_3() {
        // 50 -> 0 (via 0) -> 50
        let total = move_dial(vec!["R150".to_string(), "L50".to_string()].into(), true);
        assert_eq!(total, 2);
    }
}
