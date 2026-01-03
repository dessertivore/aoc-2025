// use bitgauss::BitMatrix;
// use regex::Regex;
// use std::{
//     cmp::{max, min},
//     collections::{HashMap, HashSet},
//     f32::INFINITY,
//     thread::current,
// };

// use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

// /// Runs the solution for Advent of Code Day 10.
pub fn day_10() -> u64 {
    let part_1 = 0;
    println!(
        "Day 10! Part 1: {:?}, Part 2: {:?}",
        part_1, "Not done yet!"
    );

    part_1
}

// #[derive(Debug, Clone)]
// struct ChristmasLights {
//     id: u32,
//     current_configuration: Vec<bool>,
//     desired_configuration: Vec<bool>,
//     wiring_schematics: Vec<Vec<u32>>,
//     joltage_requirements: Vec<u32>,
//     fewest_button_presses: f32,
// }

// impl ChristmasLights {
//     fn new(id: u32) -> Self {
//         ChristmasLights {
//             id: id,
//             current_configuration: Vec::new(),
//             desired_configuration: Vec::new(),
//             wiring_schematics: Vec::new(),
//             joltage_requirements: Vec::new(),
//             fewest_button_presses: INFINITY,
//         }
//     }

//     fn from_line(id: u32, line: &str) -> Self {
//         let re = Regex::new(
//             r"\[(?P<desired_config>.*?)\]|\((?P<wiring_schematic>.*?)\)|\{(?P<joltage_req>.*?)\}",
//         )
//         .unwrap();
//         let mut new = ChristmasLights::new(id);
//         for group in re.captures_iter(line) {
//             if let Some(v) = group.name("desired_config") {
//                 new.desired_configuration = v.as_str().chars().map(|s| s == '#').collect();
//             }
//             if let Some(v) = group.name("wiring_schematic") {
//                 new.wiring_schematics.push(
//                     v.as_str()
//                         .split(',')
//                         .map(|s| s.parse::<u32>().expect("invalid u32"))
//                         .collect(),
//                 );
//             }
//             if let Some(v) = group.name("joltage_req") {
//                 new.joltage_requirements = v
//                     .as_str()
//                     .split(',')
//                     .map(|s| s.parse::<u32>().expect("invalid u32"))
//                     .collect();
//             }
//         }
//         new.current_configuration = vec![false; new.desired_configuration.len()];
//         new
//     }

//     fn find_fewest_button_presses(&mut self) -> Option<usize> {
//         let mut matrix = BitMatrix::build(
//             self.desired_configuration.len(),
//             self.wiring_schematics.len(),
//             |_, _| false,
//         );

//         for (j, btn) in self.wiring_schematics.iter().enumerate() {
//             for &i in btn {
//                 matrix.set(i as usize, j, true);
//             }
//         }

//         // target: desired XOR current
//         let mut target = vec![false; m];
//         for i in 0..m {
//             target[i] = self.desired_configuration[i] ^ self.current_configuration[i];
//         }
//         matrix.add_column(&target);

//         matrix.gauss(false); // or true, depending on API
//         if !matrix.is_consistent() {
//             return None;
//         }

//         let base = matrix.extract_solution(); // Vec<bool> length n
//         let nulls = matrix.nullspace(); // Vec<Vec<bool>>

//         let d = nulls.len();
//         let mut best = base.iter().filter(|&&b| b).count();
//         for mask in 1..(1usize << d) {
//             let mut x = base.clone();
//             for i in 0..d {
//                 if (mask >> i) & 1 != 0 {
//                     for j in 0..n {
//                         x[j] ^= nulls[i][j];
//                     }
//                 }
//             }
//             let w = x.iter().filter(|&&b| b).count();
//             if w < best {
//                 best = w;
//             }
//         }
//         Some(best)
//     }
// }

// fn parse_input() -> Vec<ChristmasLights> {
//     let raw_input: Vec<String> = split_lines(get_aoc_input(2025, 10));
//     let mut all_lights: Vec<ChristmasLights> = Vec::new();
//     for (id, line) in raw_input.iter().enumerate() {
//         let mut line_of_lights = ChristmasLights::new(id as u32);
//         all_lights.push(ChristmasLights::from_line(id as u32, line));
//     }
//     all_lights
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_day_10() {
//         // assert_eq!(largest_area(), 50);
//         parse_input();
//     }
// }
