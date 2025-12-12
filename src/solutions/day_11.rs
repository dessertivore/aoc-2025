use rayon::prelude::*;
use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

// /// Runs the solution for Advent of Code Day 11.
pub fn day_11() -> usize {
    let mut server_rack = parse_input(11);
    server_rack.dfs(
        "you".to_string(),
        "out".to_string(),
        &mut Vec::new(),
        &mut HashMap::new(),
    );
    let part_1 = server_rack.paths_so_far.len();
    let part_2 = part_2();
    println!("Day 11! Part 1: {:?}, Part 2: {:?}", part_1, part_2);
    return part_1;
}

#[derive(Debug, Clone)]
struct ServerRack {
    connections: HashMap<String, HashSet<String>>,
    node_to_id: HashMap<String, u32>,
    id_to_node: HashMap<u32, String>,
    paths_so_far: HashSet<Vec<u32>>,
    dac_id: Option<u32>,
    fft_id: Option<u32>,
}

impl ServerRack {
    fn new() -> Self {
        ServerRack {
            connections: HashMap::new(),
            node_to_id: HashMap::new(),
            id_to_node: HashMap::new(),
            paths_so_far: HashSet::new(),
            dac_id: None,
            fft_id: None,
        }
    }

    fn dac_id(&mut self) -> Option<u32> {
        if self.dac_id.is_none() {
            self.dac_id = Some(self.node_to_id["dac"]);
        }

        return self.dac_id;
    }

    fn fft_id(&mut self) -> Option<u32> {
        if self.fft_id.is_none() {
            self.fft_id = Some(self.node_to_id["fft"]);
        }

        return self.fft_id;
    }

    fn dfs(
        &mut self,
        source: String,
        dest: String,
        path: &mut Vec<u32>,
        memo: &mut HashMap<(String, Vec<u32>), HashSet<Vec<u32>>>,
    ) {
        let key = (source.clone(), path.clone());
        if let Some(cached_paths) = memo.get(&key) {
            self.paths_so_far.extend(cached_paths.clone());
            return;
        }

        if source == dest.clone() {
            self.paths_so_far.insert(path.clone());
            memo.insert(key.clone(), self.paths_so_far.clone());
            return;
        } else {
            if let Some(&node_id) = self.node_to_id.get(&source) {
                path.push(node_id);
                if let Some(neighbors) = self.connections.get(&source) {
                    for neighbor in neighbors.clone() {
                        self.dfs(neighbor, dest.clone(), path, memo);
                    }
                    path.pop();
                }
            }
            memo.insert(key.clone(), self.paths_so_far.clone());
        }
    }

    fn dfs_part_2(
        &mut self,
        source: &String,
        dest: &String,
        path: &mut Vec<u32>,
        memo: &mut HashMap<u64, HashSet<Vec<u32>>>,
    ) {
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        path.last().unwrap_or(&0).hash(&mut hasher); // Use only the last node in the path
        let key = hasher.finish();

        if let Some(cached_paths) = memo.get(&key) {
            self.paths_so_far.extend(cached_paths.clone());
            return;
        }
        let mut current_paths: HashSet<Vec<u32>> = HashSet::new();

        if source == dest {
            // if path.contains(&self.dac_id().unwrap()) && path.contains(&self.fft_id().unwrap()) {
            self.paths_so_far.insert(path.clone());
            current_paths.insert(path.clone());
            // }
        } else {
            if let Some(&node_id) = self.node_to_id.get(source) {
                path.push(node_id);
                if let Some(neighbors) = self.connections.get(source).cloned() {
                    for neighbor in neighbors {
                        self.dfs_part_2(&neighbor, dest, path, memo);
                    }
                }
                path.pop();
            }
        }
        memo.insert(key.clone(), current_paths.clone());
        self.paths_so_far.extend(current_paths);
    }
}

fn parse_input(day: u8) -> ServerRack {
    let raw_input: Vec<String> = split_lines(get_aoc_input(2025, day as u32));
    let mut server_rack: ServerRack = ServerRack::new();
    let re = Regex::new(r"[a-zA-Z]{3}").unwrap();

    for (id, line) in raw_input.iter().enumerate() {
        let all_groups: Vec<String> = re
            .find_iter(line)
            .map(|mat| mat.as_str().to_string())
            .collect();
        server_rack.connections.insert(
            all_groups[0].clone(),
            HashSet::from_iter(all_groups[1..].to_vec()),
        );
        server_rack
            .node_to_id
            .insert(all_groups[0].clone(), id as u32);
        server_rack
            .id_to_node
            .insert(id as u32, all_groups[0].clone());
    }
    server_rack
}

fn part_2() -> u64 {
    let testing: bool = cfg!(test);
    let mut svr_to_dac: ServerRack;
    if testing {
        svr_to_dac = parse_input(25); // saved under day 25 because part 2 has diff test input
    } else {
        svr_to_dac = parse_input(11);
    }

    // Clone the initial `ServerRack` for each DFS call
    let racks = vec![
        ("svr", "dac", svr_to_dac.clone()),
        ("svr", "fft", svr_to_dac.clone()),
        ("fft", "dac", svr_to_dac.clone()),
        ("dac", "fft", svr_to_dac.clone()),
        ("fft", "out", svr_to_dac.clone()),
        ("dac", "out", svr_to_dac.clone()),
    ];

    // Use `par_iter` to parallelize the DFS calls
    let results: Vec<HashSet<Vec<u32>>> = racks
        .into_par_iter()
        .map(|(start, end, mut rack)| {
            rack.dfs_part_2(
                &start.to_string(),
                &end.to_string(),
                &mut Vec::new(),
                &mut HashMap::new(),
            );
            rack.paths_so_far
        })
        .collect();

    // Extract the results
    let svr_to_dac = results[0].len();
    let svr_to_fft = results[1].len();
    let fft_to_dac = results[2].len();
    let dac_to_fft = results[3].len();
    let fft_to_out = results[4].len();
    let dac_to_out = results[5].len();

    // Compute the final result
    let svr_dac_fft_out = svr_to_dac * dac_to_fft * fft_to_out;
    let svr_fft_dac_out = svr_to_fft * fft_to_dac * dac_to_out;

    svr_dac_fft_out as u64 + svr_fft_dac_out as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_11() {
        // assert_eq!(largest_area(), 50);
        let mut test = parse_input(11);
        test.dfs(
            "you".to_string(),
            "out".to_string(),
            &mut Vec::new(),
            &mut HashMap::new(),
        );
        assert_eq!(test.paths_so_far.len(), 5);
        println!("{:?},{:?}", parse_input(11), test.paths_so_far.len());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), 2);
    }
}
