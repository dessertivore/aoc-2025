use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

/// Runs the solution for Advent of Code Day 9.
pub fn day_9() -> u64 {
    let part_1 = largest_area();
    println!("Day 9! Part 1: {:?}, Part 2: {:?}", part_1, "Not done yet!");
    return part_1;
}

#[derive(Debug, Clone)]
struct MovieTheatre {
    all_red_tile_coords: HashMap<u64, (u64, u64)>,
    rectangle_areas: HashMap<(u64, u64), u64>,
}

impl MovieTheatre {
    fn new() -> Self {
        MovieTheatre {
            all_red_tile_coords: HashMap::new(),
            rectangle_areas: HashMap::new(),
        }
    }

    fn add_coord(&mut self, coord: (u64, u64), id: u64) {
        for (other_id, other_coord) in &self.all_red_tile_coords {
            let sorted_key = (min(id, *other_id), max(id, *other_id));
            self.rectangle_areas
                .insert(sorted_key, rectangle_area(coord, *other_coord));
        }
        self.all_red_tile_coords.insert(id, coord);
    }
}

fn rectangle_area(coord_1: (u64, u64), coord_2: (u64, u64)) -> u64 {
    let y_diff = coord_2.1.abs_diff(coord_1.1) + 1;
    let x_diff = coord_2.0.abs_diff(coord_1.0) + 1;
    let area = x_diff * y_diff;
    return area;
}

fn parse_input() -> MovieTheatre {
    let raw_input: Vec<String> = split_lines(get_aoc_input(2025, 9));
    let mut movie_theatre = MovieTheatre::new();
    for (id, coords) in raw_input.iter().enumerate() {
        let coords_split: Vec<u64> = coords
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let x_coord = coords_split[0];
        let y_coord = coords_split[1];
        movie_theatre.add_coord((x_coord, y_coord), id as u64);
    }
    movie_theatre
}

fn largest_area() -> u64 {
    let movie_theatre = parse_input();
    let largest_area = movie_theatre.rectangle_areas.values().max().unwrap();
    return *largest_area;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_9() {
        assert_eq!(largest_area(), 50);
    }
}
