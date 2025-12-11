use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

// /// Runs the solution for Advent of Code Day 11.
pub fn day_11() -> usize {
    let mut server_rack = parse_input(11);
    server_rack.dfs("you".to_string(), &mut Vec::new());
    let part_1 = server_rack.paths_so_far.len();
    let part_2 = part_2().len();
    println!("Day 11! Part 1: {:?}, Part 2: {:?}", part_1, part_2);
    return part_1;
}

#[derive(Debug, Clone)]
struct ServerRack {
    connections: HashMap<String, HashSet<String>>,
    node_to_id: HashMap<String, u32>,
    id_to_node: HashMap<u32, String>,
    paths_so_far: HashSet<Vec<u32>>,
}

impl ServerRack {
    fn new() -> Self {
        ServerRack {
            connections: HashMap::new(),
            node_to_id: HashMap::new(),
            id_to_node: HashMap::new(),
            paths_so_far: HashSet::new(),
        }
    }

    fn dfs(&mut self, source: String, path: &mut Vec<u32>) {
        if source == "out" {
            self.paths_so_far.insert(path.clone());
        } else {
            if let Some(&node_id) = self.node_to_id.get(&source) {
                path.push(node_id);
                let neighbors: Vec<String> = self
                    .connections
                    .get(&source)
                    .unwrap_or(&HashSet::new())
                    .iter()
                    .cloned()
                    .collect::<Vec<String>>();
                for x in neighbors {
                    self.dfs(x, path);
                }
                path.pop();
            }
        }
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

fn part_2() -> HashSet<Vec<u32>> {
    let testing: bool = cfg!(test);
    let mut no_dac: ServerRack;
    if testing {
        no_dac = parse_input(25); // saved under day 25 because part 2 has diff test input
    } else {
        no_dac = parse_input(11);
    }
    let mut no_fft = no_dac.clone();
    let mut all_items = no_dac.clone();
    all_items.dfs("svr".to_string(), &mut Vec::new());
    println!("Got all items");

    no_fft.node_to_id.remove("fft");
    no_fft.dfs("svr".to_string(), &mut Vec::new());

    println!("Got no_fft");
    no_dac.node_to_id.remove("dac");
    no_dac.dfs("svr".to_string(), &mut Vec::new());
    println!("Got no_dac");

    let no_dac_overlap_no_fft = no_dac
        .paths_so_far
        .union(&no_fft.paths_so_far)
        .cloned()
        .collect::<HashSet<Vec<u32>>>();
    let no_dac_overlap_no_fft_diff_from_all_routes = all_items
        .paths_so_far
        .difference(&no_dac_overlap_no_fft)
        .cloned()
        .collect::<HashSet<Vec<u32>>>();

    return no_dac_overlap_no_fft_diff_from_all_routes;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_11() {
        // assert_eq!(largest_area(), 50);
        let mut test = parse_input(11);
        test.dfs("you".to_string(), &mut Vec::new());
        assert_eq!(test.paths_so_far.len(), 5);
        println!("{:?},{:?}", parse_input(11), test.paths_so_far.len());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2().len(), 2);
    }
}
