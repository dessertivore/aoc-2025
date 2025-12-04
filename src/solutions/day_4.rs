use core::num;
use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

/// Runs the solution for Advent of Code Day 4.
pub fn day_4() -> u64 {
    let part_1 = find_total_accessible_rolls();
    println!("Day 4! Part 1: {:?}", part_1);
    return part_1;
}

fn get_neighbours(x: usize, y: usize) -> Vec<(isize, isize)> {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    directions
        .iter()
        .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
        .collect()
}

fn get_roll_coords() -> (HashSet<(usize, usize)>, usize, usize) {
    let input: Vec<String> = split_lines(get_aoc_input(2025, 4));
    let mut roll_map: HashSet<(usize, usize)> = HashSet::new();
    let max_x = input[0].len();
    let max_y = input.len();
    for (x_coord, row) in input.iter().enumerate() {
        for (y_coord, char) in row.chars().enumerate() {
            if char.to_string() == "@".to_string() {
                roll_map.insert((x_coord, y_coord));
            }
        }
    }
    return (roll_map, max_x, max_y);
}
fn find_total_accessible_rolls() -> u64 {
    let (roll_map, max_x, max_y) = get_roll_coords();
    let mut accessible_rolls: u64 = 0;
    for roll in roll_map.iter() {
        let neighbours = get_neighbours(roll.0, roll.1);
        let mut adjacent_rolls = 0;
        for neighbour in neighbours.iter() {
            if let (Ok(nx), Ok(ny)) = (usize::try_from(neighbour.0), usize::try_from(neighbour.1)) {
                if roll_map.contains(&(nx, ny)) {
                    adjacent_rolls += 1;
                }
            }
        }

        if adjacent_rolls < 4 {
            accessible_rolls += 1;
        }
    }
    return accessible_rolls;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_3() {
        assert_eq!(find_total_accessible_rolls(), 13);
    }
}
