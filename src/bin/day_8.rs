// Code isn't working - will come back to this!

// use kd_tree::ItemAndDistance;
// use kd_tree::KdMap;
// use kd_tree::KdTreeN;
// use std::cmp::min;
// use std::cmp::Reverse;
// use std::collections::{BTreeSet, HashMap, HashSet};
// use typenum::U3;

// use crate::utils::{
//     get_input::get_aoc_input,
//     parsing::{split_lines, split_string_by_specified_char},
// };

// /// Runs the solution for Advent of Code Day 8.
pub fn main() {
    // let part_1 = parse_input();
    // println!("Day 8! Part 1: {:?}", part_1);
}

// #[derive(Debug, Clone)]
// struct JunctionBox {
//     x: f64,
//     y: f64,
//     z: f64,
// }

// fn parse_input() -> KdTreeN<([f64; 3], usize), U3> {
//     let raw_input: Vec<String> = split_lines(get_aoc_input(2025, 8));

//     // Collect all points first
//     let mut points: Vec<([f64; 3], usize)> = Vec::with_capacity(raw_input.len());

//     for (id, junction_box) in raw_input.iter().enumerate() {
//         let mut coords = junction_box.split(',');

//         let x: f64 = coords.next().unwrap().parse().unwrap();
//         let y: f64 = coords.next().unwrap().parse().unwrap();
//         let z: f64 = coords.next().unwrap().parse().unwrap();

//         points.push(([x, y, z], id));
//     }
//     println!("{:?}", points);
//     KdTreeN::build_by_ordered_float(points)
// }

// fn find_all_nearest(limit_new_connections: u32) -> Vec<HashSet<usize>> {
//     let kdtree = parse_input();
//     let mut nearests: Vec<(usize, usize, f64)> = Vec::new();
//     let mut connected: Vec<HashSet<usize>> = Vec::new();
//     let mut pos = 0;
//     for (point, id) in kdtree.iter() {
//         let mut flag: bool = false;
//         let mut addition = 0;
//         while !flag {
//             // Ask for the 2 nearest neighbors
//             let nearest = kdtree.nearests(point, 2 + addition)[1 + addition];
//             let key = (*id, nearest.item.1, nearest.squared_distance);
//             if !nearests.contains(&(key.1, key.0, key.2)) {
//                 nearests.push((*id, nearest.item.1, nearest.squared_distance));
//                 flag = true;
//             } else {
//                 println!("This pairing already exists. Finding next neighbour.");
//                 addition += 1;
//             }
//         }
//     }
//     nearests.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
//     println!("{:?}", nearests);
//     for (current, nearest, distance) in nearests.iter() {
//         let mut added = false;
//         for connections in connected.iter_mut() {
//             if connections.contains(nearest) || connections.contains(current) {
//                 added = true;
//                 pos += 1;
//                 connections.insert(current.clone());
//             }
//         }
//         if !added {
//             connected.push(HashSet::from_iter([current.clone(), nearest.clone()]));
//             pos += 1;
//         }
//         println!("{:?},{:?},{:?}", current, nearest, connected);
//         if pos as u32 >= limit_new_connections {
//             break;
//         }
//     }
//     println!("{:?}", connected);
//     connected
// }

// fn product_of_nearests(limit_new_connections: u32) -> usize {
//     let nearests = find_all_nearest(limit_new_connections);
//     let mut lengths: Vec<usize> = nearests.iter().map(|a| a.len()).collect();
//     lengths.sort();
//     lengths.reverse();
//     println!("{:?}", lengths);
//     return lengths[..3].iter().product();
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_day_8() {
//         // assert_eq!(find_all_nearest(1), [HashSet::from([19, 0])]);
//         // assert_eq!(find_all_nearest(2), [HashSet::from([19, 0, 7])]);
//         // assert_eq!(
//         //     find_all_nearest(3),
//         //     [HashSet::from([19, 0, 7]), HashSet::from([2, 13])]
//         // );
//         assert_eq!(product_of_nearests(10), 40);
//     }
// }
