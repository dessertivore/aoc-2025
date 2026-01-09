use std::collections::HashSet;

use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

/// Runs the solution for Advent of Code Day 4.
pub fn day_4() {
    let part_1 = find_total_accessible_rolls(None);
    println!(
        "Day 4! Part 1: {:?}, Part 2: {:?}",
        part_1.len(),
        remove_rolls_as_you_go()
    );
}

/// Returns the coordinates of all 8 neighbouring positions (including diagonals)
/// for a given (x, y) coordinate as a vector of (isize, isize) tuples.
///
/// # Arguments
///
/// * `x` - The x-coordinate.
/// * `y` - The y-coordinate.
///
/// # Returns
///
/// A vector of tuples representing the coordinates of all neighbouring positions.
fn get_neighbours(x: u32, y: u32) -> Vec<(isize, isize)> {
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

/// Parses the input for Day 4 and returns a set of coordinates where rolls ('@') are located.
///
/// # Returns
///
/// A `HashSet` containing tuples of (x, y) coordinates for each roll found in the input.
fn get_roll_coords() -> HashSet<(u32, u32)> {
    let input: Vec<String> = split_lines(get_aoc_input(2025, 4));
    let mut roll_map: HashSet<(u32, u32)> = HashSet::new();
    for (x_coord, row) in input.iter().enumerate() {
        for (y_coord, char) in row.chars().enumerate() {
            if char.to_string() == "@" {
                roll_map.insert((
                    u32::try_from(x_coord).unwrap(),
                    u32::try_from(y_coord).unwrap(),
                ));
            }
        }
    }
    roll_map
}

/// Finds all "accessible" rolls in the provided roll map.
/// A roll is considered accessible if it has fewer than 4 adjacent rolls (including diagonals).
///
/// # Arguments
///
/// * `roll_map` - An optional reference to a set of roll coordinates. If `None`, the function
///                will parse the input to generate the roll map.
///
/// # Returns
///
/// A vector of (x, y) coordinates for all accessible rolls.
fn find_total_accessible_rolls(roll_map: Option<&HashSet<(u32, u32)>>) -> Vec<(u32, u32)> {
    let default_roll_map = if roll_map.is_none() {
        get_roll_coords()
    } else {
        HashSet::new()
    };
    let roll_map: &HashSet<(u32, u32)> = roll_map.unwrap_or(&default_roll_map);
    let mut accessible_rolls: Vec<(u32, u32)> = Vec::new();
    for roll in roll_map.iter() {
        let neighbours = get_neighbours(roll.0, roll.1);
        let mut adjacent_rolls = 0;
        for neighbour in neighbours.iter() {
            if let (Ok(nx), Ok(ny)) = (u32::try_from(neighbour.0), u32::try_from(neighbour.1)) {
                if roll_map.contains(&(nx, ny)) {
                    adjacent_rolls += 1;
                }
            }
        }

        if adjacent_rolls < 4 {
            accessible_rolls.push(*roll);
        }
    }

    accessible_rolls
}

/// Iteratively removes all accessible rolls from the roll map until no more can be removed,
/// and returns the total number of rolls removed.
///
/// A roll is considered "accessible" if it has fewer than 4 adjacent rolls (including diagonals).
/// In each iteration, all currently accessible rolls are removed from the map. The process
/// repeats until no new accessible rolls can be found or removed.
///
/// # Returns
///
/// * `u32` - The total number of rolls that were removed from the map.
///
/// # Panics
///
/// Panics if the number of removed rolls cannot be converted to `u32`.
fn remove_rolls_as_you_go() -> u32 {
    let mut roll_map: HashSet<(u32, u32)> = get_roll_coords();
    let mut removed_rolls: HashSet<(u32, u32)> = HashSet::new();
    while !roll_map.is_empty() {
        let touched: HashSet<(u32, u32)> = find_total_accessible_rolls(Some(&roll_map))
            .into_iter()
            .collect();
        if removed_rolls.is_superset(&touched) || roll_map.intersection(&touched).count() == 0 {
            // If we've already seen all these items, stop the loop as we've maxed out
            // all the rolls we can touch
            break;
        }
        for item in touched.iter() {
            removed_rolls.insert(*item);
            roll_map.remove(item);
        }
    }

    u32::try_from(removed_rolls.len()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_3() {
        assert_eq!(find_total_accessible_rolls(None).len(), 13);
    }
    #[test]
    fn test_day_3_part_2() {
        assert_eq!(remove_rolls_as_you_go(), 43);
    }
}
