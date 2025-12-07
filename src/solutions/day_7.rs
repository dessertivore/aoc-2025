use std::collections::{HashMap, HashSet};

use crate::utils::{
    get_input::get_aoc_input,
    parsing::{split_lines, split_string_by_specified_char},
};

/// Runs the solution for Advent of Code Day 7.
pub fn day_7() -> u32 {
    let part_1 = count_splits();
    println!("Day 7! Part 1: {:?}", part_1);
    return part_1;
}

#[derive(Debug)]
struct TachyonManifold {
    splitters: HashSet<(u32, u32)>,
    all_beam_coords: HashMap<u32, HashSet<u32>>,
    current_tachyon_beams_coords: HashSet<u32>,
    splits_so_far: u32,
    max_x: u32,
    max_y: u32,
    current_x_cord: u32,
}

impl TachyonManifold {
    fn new() -> Self {
        TachyonManifold {
            splitters: HashSet::new(),
            all_beam_coords: HashMap::new(),
            current_tachyon_beams_coords: HashSet::new(),
            splits_so_far: 0,
            max_x: 0,
            max_y: 0,
            current_x_cord: 0,
        }
    }

    fn advance_tachyons_by_1(&mut self) {
        let mut next_line_of_tachyons: HashSet<u32> = HashSet::new();
        self.current_x_cord += 1;
        for tachyon in &self.current_tachyon_beams_coords {
            if self.splitters.contains(&(self.current_x_cord, *tachyon)) {
                let new_y_right = tachyon + 1;
                let new_y_left = tachyon - 1;
                next_line_of_tachyons.insert(new_y_left);
                next_line_of_tachyons.insert(new_y_right);
                self.splits_so_far += 1
            } else {
                next_line_of_tachyons.insert(*tachyon);
            }
        }
        self.all_beam_coords
            .insert(self.current_x_cord, next_line_of_tachyons.clone());
        self.current_tachyon_beams_coords = next_line_of_tachyons;
        if cfg!(test) {
            println!("{:?}", self);
        }
    }

    fn move_to_bottom_of_map(&mut self) {
        while self.current_x_cord < self.max_x {
            self.advance_tachyons_by_1();
        }
    }
}

fn count_splits() -> u32 {
    let raw_input: Vec<String> = split_lines(get_aoc_input(2025, 7));
    let mut manifold = TachyonManifold::new();
    for (x_coord, line) in raw_input.iter().enumerate() {
        for (y_coord, char) in line.chars().enumerate() {
            if char.to_string() == "S".to_string() {
                manifold.current_tachyon_beams_coords.insert(y_coord as u32);
            } else if char.to_string() == "^".to_string() {
                manifold.splitters.insert((x_coord as u32, y_coord as u32));
            }
            if y_coord as u32 > manifold.max_y {
                manifold.max_y = y_coord as u32
            }
        }
        if x_coord as u32 > manifold.max_x {
            manifold.max_x = x_coord as u32
        }
    }
    manifold.move_to_bottom_of_map();
    return manifold.splits_so_far;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_7() {
        assert_eq!(count_splits(), 21);
    }
}
