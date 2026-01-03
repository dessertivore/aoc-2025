use std::collections::{HashMap, HashSet};

use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

/// Runs the solution for Advent of Code Day 7.
pub fn day_7() -> u32 {
    let part_1 = count_splits();
    let manifold = parse_input();
    let part_2 = recursive_find_paths(manifold, HashSet::new(), HashMap::new());
    println!("Day 7! Part 1: {:?}, Part 2: {:?}", part_1, part_2);

    part_1
}

#[derive(Debug, Clone)]
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

    fn advance_tachyons_by_1(&mut self, go_left: bool, go_right: bool) {
        let mut next_line_of_tachyons: HashSet<u32> = HashSet::new();
        self.current_x_cord += 1;
        for tachyon in &self.current_tachyon_beams_coords {
            if self.splitters.contains(&(self.current_x_cord, *tachyon)) {
                let new_y_right = tachyon + 1;
                let new_y_left = tachyon - 1;
                if go_left {
                    next_line_of_tachyons.insert(new_y_left);
                }
                if go_right {
                    next_line_of_tachyons.insert(new_y_right);
                }
                self.splits_so_far += 1
            } else {
                next_line_of_tachyons.insert(*tachyon);
            }
        }
        self.all_beam_coords
            .insert(self.current_x_cord, next_line_of_tachyons.clone());
        self.current_tachyon_beams_coords = next_line_of_tachyons;
    }

    fn move_to_bottom_of_map(&mut self) {
        while self.current_x_cord < self.max_x {
            self.advance_tachyons_by_1(true, true);
        }
    }

    fn single_path_coords(self) -> Vec<u32> {
        let mut vec_path = Vec::new();
        for (_, coords) in self.all_beam_coords {
            let vec = Vec::from_iter(coords);
            vec_path.push(vec[0]);
        }

        vec_path
    }
}

fn recursive_find_paths(
    manifold: TachyonManifold,
    mut coords_so_far: HashSet<Vec<u32>>,
    memo: HashMap<u32, HashSet<Vec<u32>>>,
) -> HashSet<Vec<u32>> {
    let check_cache: u32 = manifold.clone().current_x_cord;
    let cached: Option<HashSet<Vec<u32>>> = memo.get(&check_cache).cloned();
    if cached.is_some() {
        return memo.get(&check_cache).unwrap().clone();
    }
    if manifold.current_x_cord >= manifold.max_x {
        coords_so_far.insert(manifold.clone().single_path_coords());
        return coords_so_far;
    }
    let mut left = manifold.clone();
    left.advance_tachyons_by_1(true, false);
    let mut right = manifold.clone();
    right.advance_tachyons_by_1(false, true);
    let all_left = recursive_find_paths(left, coords_so_far.clone(), memo.clone());
    let all_right = recursive_find_paths(right, coords_so_far.clone(), memo.clone());

    let mut combined = all_left;
    combined.extend(all_right);

    combined
}

fn parse_input() -> TachyonManifold {
    let raw_input: Vec<String> = split_lines(get_aoc_input(2025, 7));
    let mut manifold = TachyonManifold::new();
    for (x_coord, line) in raw_input.iter().enumerate() {
        for (y_coord, char) in line.chars().enumerate() {
            if char.to_string() == "S" {
                manifold.current_tachyon_beams_coords.insert(y_coord as u32);
            } else if char.to_string() == "^" {
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
    manifold
}
fn count_splits() -> u32 {
    let mut manifold = parse_input();
    manifold.move_to_bottom_of_map();

    manifold.splits_so_far
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_7() {
        assert_eq!(count_splits(), 21);
    }

    #[test]
    fn test_part_2() {
        let test_manifold = parse_input();
        let resp = recursive_find_paths(test_manifold, HashSet::new(), HashMap::new());
        assert_eq!(resp.len(), 40);
    }
}
