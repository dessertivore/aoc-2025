use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

// /// Runs the solution for Advent of Code Day 11.
pub fn day_11() -> usize {
    let mut server_rack = parse_input();
    server_rack.dfs("you".to_string(), &mut Vec::new());
    let part_1 = server_rack.paths_so_far.len();
    println!(
        "Day 11! Part 1: {:?}, Part 2: {:?}",
        part_1, "Not done yet!"
    );
    return part_1;
}

#[derive(Debug, Clone)]
struct ServerRack {
    connections: HashMap<String, HashSet<String>>,
    node_to_id: HashMap<String, u32>,
    id_to_node: HashMap<u32, String>,
    paths_so_far: Vec<Vec<u32>>,
}

impl ServerRack {
    fn new() -> Self {
        ServerRack {
            connections: HashMap::new(),
            node_to_id: HashMap::new(),
            id_to_node: HashMap::new(),
            paths_so_far: Vec::new(),
        }
    }

    fn dfs(&mut self, source: String, path: &mut Vec<u32>) {
        println!("{:?}", source);

        if source == "out" {
            self.paths_so_far.push(path.clone())
        } else {
            path.push(self.node_to_id[&source]);
            let neighbors: Vec<String> = self.connections[&source].iter().cloned().collect();
            for x in neighbors {
                self.dfs(x, path);
            }
            path.pop();
        }
    }
}

fn parse_input() -> ServerRack {
    let raw_input: Vec<String> = split_lines(get_aoc_input(2025, 11));
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
    println!("{:?}", server_rack);
    server_rack
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_10() {
        // assert_eq!(largest_area(), 50);
        let mut test = parse_input();
        test.dfs("you".to_string(), &mut Vec::new());
        assert_eq!(test.paths_so_far.len(), 5);
        println!("{:?},{:?}", parse_input(), test.paths_so_far.len());
    }
}
