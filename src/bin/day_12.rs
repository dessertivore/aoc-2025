use std::collections::HashMap;

use aoc_2025::utils::{get_input::get_aoc_input, parsing::split_lines};

// /// Runs the solution for Advent of Code Day 12.
pub fn main() {
    let part_1 = can_they_fit();
    println!(
        "Day 12! Part 1: {:?}, Part 2: {:?}",
        part_1, "not done yet!"
    );
}

#[derive(Debug)]
struct PresentsAndTrees {
    present_shapes: HashMap<i32, Present>,
    regions_under_trees: Vec<TreeRegion>,
}

#[derive(Debug)]
struct TreeRegion {
    region_dimensions: (u32, u32),
    presents_required: HashMap<u32, u32>,
    total_area: u32,
}

impl TreeRegion {
    fn new(region_dimensions: (u32, u32), presents_required: HashMap<u32, u32>) -> TreeRegion {
        TreeRegion {
            region_dimensions,
            presents_required,
            total_area: region_dimensions.0 * region_dimensions.1,
        }
    }
}
#[derive(Debug)]
struct Present {
    id: i32,
    coords: Vec<(u32, u32)>,
    dimensions: (u32, u32),
    outline_area: u32,
    total_area: u32,
}

impl Present {
    fn new(id: i32, coords: Vec<(u32, u32)>) -> Present {
        let total_area = coords.len() as u32;
        let dimensions = Present::shape_dimensions(&coords);
        Present {
            id,
            coords,
            dimensions,
            outline_area: dimensions.0 * dimensions.1,
            total_area,
        }
    }

    fn shape_dimensions(coords: &[(u32, u32)]) -> (u32, u32) {
        let min_r: u32 = coords.iter().map(|(r, _)| *r).min().unwrap();
        let max_r: u32 = coords.iter().map(|(r, _)| *r).max().unwrap();
        let min_c: u32 = coords.iter().map(|(_, c)| *c).min().unwrap();
        let max_c: u32 = coords.iter().map(|(_, c)| *c).max().unwrap();

        let height = max_r - min_r + 1; // y_up
        let width = max_c - min_c + 1; // x_across

        (width, height)
    }
}

fn parse_input() -> PresentsAndTrees {
    let mut patterns: HashMap<i32, Present> = HashMap::new();
    let mut numeric_lines: Vec<TreeRegion> = Vec::new();
    let mut raw_input = split_lines(get_aoc_input(2025, 12)).into_iter().peekable();

    while let Some(line) = raw_input.peek().cloned() {
        // Pattern header like "0:" or "1:"
        if let Some((num_str, _)) = line.split_once(':') {
            if let Ok(idx) = num_str.trim().parse::<i32>() {
                raw_input.next(); // consume this line

                // Parse subsequent grid lines until blank line or non-grid marker
                let mut coords = Vec::new();
                let mut row: u32 = 0;

                while let Some(grid_line) = raw_input.peek().cloned() {
                    if grid_line.trim().is_empty() || grid_line.contains(':') {
                        break;
                    }

                    for (col, ch) in grid_line.chars().enumerate() {
                        if ch == '#' {
                            coords.push((row, col as u32));
                        }
                    }

                    row += 1;
                    raw_input.next(); // consume grid line
                }

                patterns.insert(idx, Present::new(idx, coords));
                continue;
            }
        }

        // Numeric line like: "4x4: 0 0 0 0 2 0"
        if line.contains("x") && line.contains(":") {
            raw_input.next(); // consume
            let (label, nums) = line.split_once(':').unwrap();

            let (w_str, h_str) = label.trim().split_once('x').unwrap();
            let region_dimensions = (w_str.parse::<u32>().unwrap(), h_str.parse::<u32>().unwrap());

            let presents_raw = nums
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<u32>>();
            let mut presents_required: HashMap<u32, u32> = HashMap::new();

            for (idx, val) in presents_raw.into_iter().enumerate() {
                if val != 0 {
                    presents_required.insert(idx as u32, val);
                }
            }
            numeric_lines.push(TreeRegion::new(region_dimensions, presents_required));
            continue;
        }

        // Otherwise skip
        raw_input.next();
    }

    PresentsAndTrees {
        present_shapes: patterns,
        regions_under_trees: numeric_lines,
    }
}

fn can_they_fit() -> u32 {
    let input = parse_input();
    let mut possible_trees = 0;
    for tree in input.regions_under_trees.iter() {
        let mut area_taken_up_so_far = 0;
        for (present, quantity) in tree.presents_required.iter() {
            // Check if total space of gifts can feasibly fit under tree,
            // without bothering rotation so far
            let total_present_area = input.present_shapes[&(*present as i32)].total_area * quantity;
            area_taken_up_so_far += total_present_area;
        }

        if area_taken_up_so_far < tree.total_area {
            possible_trees += 1
        }
    }
    possible_trees
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_12() {
        // assert_eq!(largest_area(), 50);
        let mut test = parse_input();
        println!("{:?}", test);
        assert_eq!(can_they_fit(), 2)
    }
}
