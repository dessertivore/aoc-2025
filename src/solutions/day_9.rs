use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

/// Runs the solution for Advent of Code Day 9.
pub fn day_9() -> u64 {
    let part_1 = largest_area();
    println!("Day 9! Part 1: {:?}, Part 2: {:?}", part_1, "Not done yet!");

    part_1
}

#[derive(Debug, Clone)]
struct MovieTheatre {
    all_red_tile_coords: HashMap<u64, (u64, u64)>,
    all_green_tile_coords: HashSet<(u64, u64)>,
    rectangle_areas: HashMap<(u64, u64), u64>,
    max_id: u64,
}

impl MovieTheatre {
    fn new() -> Self {
        MovieTheatre {
            all_red_tile_coords: HashMap::new(),
            all_green_tile_coords: HashSet::new(),
            rectangle_areas: HashMap::new(),
            max_id: 0,
        }
    }

    fn add_coord(&mut self, coord: (u64, u64), id: u64) {
        for (other_id, other_coord) in &self.all_red_tile_coords {
            let sorted_key = (min(id, *other_id), max(id, *other_id));
            self.rectangle_areas
                .insert(sorted_key, rectangle_area(coord, *other_coord));
        }
        self.all_red_tile_coords.insert(id, coord);
        if id > self.max_id {
            self.max_id = id;
        }
    }

    fn find_all_greens(&mut self) {
        for (id, coord) in self.all_red_tile_coords.iter() {
            let previous_red: &(u64, u64);
            if *id == 0 {
                previous_red = self.all_red_tile_coords.get(&self.max_id).unwrap();
            } else {
                // There are other red tiles, time to calculate coords of green tiles!
                previous_red = self.all_red_tile_coords.get(&(id - 1)).unwrap();
            }
            let current_x = max(previous_red.0, coord.0);
            let target_x = min(previous_red.0, coord.0);
            let current_y = max(previous_red.1, coord.1);
            let target_y = min(previous_red.1, coord.1);

            let x_step: i64 = if current_x < target_x {
                1
            } else if current_x > target_x {
                -1
            } else {
                0
            };
            let y_step: i64 = if current_y < target_y {
                1
            } else if current_y > target_y {
                -1
            } else {
                0
            };

            let mut current = (current_x, current_y);
            while current != (target_x, target_y) {
                println!("{:?},{:?},{:?}", current, target_x, target_y);

                self.all_green_tile_coords.insert(current);
                current = (
                    (current.0 as i64 + x_step) as u64,
                    (current.1 as i64 + y_step) as u64,
                );
            }
        }
    }
}

fn rectangle_area(coord_1: (u64, u64), coord_2: (u64, u64)) -> u64 {
    let y_diff = coord_2.1.abs_diff(coord_1.1) + 1;
    let x_diff = coord_2.0.abs_diff(coord_1.0) + 1;

    x_diff * y_diff
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
    *largest_area
}

fn largest_red_area_without_greens() -> u64 {
    let mut movie_theatre = parse_input();
    movie_theatre.find_all_greens();
    println!("{:?}", movie_theatre.all_green_tile_coords);

    0 // Placeholder
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_9() {
        assert_eq!(largest_area(), 50);
    }

    #[test]
    fn test_day_9_part_2() {
        largest_red_area_without_greens();
    }
}
