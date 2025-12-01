use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

pub fn day_1() {
    println!("Part 1: {:?}", part_1());
}

fn part_1() -> i16 {
    let input = split_lines(get_aoc_input(2025, 1));
    let mut current_pos: i16 = 50;
    let mut at_zero: i16 = 0;
    for instruction in input {
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
        if current_pos < 0 || current_pos > 99 {
            current_pos = ((current_pos % 100) + 100) % 100;
        }
        if current_pos == 0 {
            at_zero += 1
        }
    }
    return at_zero;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), 3);
    }
}
