use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

/// Runs the solution for Advent of Code Day 6.
pub fn day_6() -> u64 {
    let part_1 = solve_all_lines();
    println!("Day 6! Part 1: {:?}", part_1);
    return part_1;
}

#[derive(Debug)]
struct MathsSheet {
    lines_of_numbers: Vec<Vec<u64>>,
    instructions: Vec<String>,
}

impl MathsSheet {
    fn solve_line(&self, line: Vec<u64>, operator_pos: usize) -> u64 {
        let mut line_total: u64 = 0;
        let operator = self.instructions[operator_pos].as_str();
        match operator {
            "+" => line_total = line.iter().sum(), // Calculate the sum of the line
            "*" => line_total = line.iter().product(), // Calculate the product of the line
            _ => println!(
                "No function found for input: {}",
                self.instructions[operator_pos]
            ),
        }
        return line_total;
    }

    fn solve_all_lines(&self) -> u64 {
        let mut grand_total: u64 = 0;
        for pos in 0..self.lines_of_numbers.len() {
            grand_total += self.solve_line(self.lines_of_numbers[pos].clone(), pos);
        }
        return grand_total;
    }
}

fn solve_all_lines() -> u64 {
    let mut input: Vec<String> = split_lines(get_aoc_input(2025, 6));
    let mut sheet: MathsSheet = MathsSheet {
        lines_of_numbers: Vec::new(),
        instructions: Vec::new(),
    };
    sheet.instructions = input
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .map(String::from)
        .collect(); // instructions are on last line
    let mut vertical_lines: Vec<Vec<u64>> = Vec::new();
    for line in input {
        let mut pos = 0;
        let chars = line.split_whitespace();
        if vertical_lines.is_empty() {
            let char_count = chars.clone().count(); // Clone the iterator to avoid consuming it
            vertical_lines = vec![Vec::new(); char_count]; // Instantiate vertical lines
        }
        for char in line.split_whitespace() {
            vertical_lines[pos].push(char.parse::<u64>().unwrap());
            pos += 1;
        }
    }
    sheet.lines_of_numbers = vertical_lines;

    if cfg!(test) {
        println!("{:?}", sheet);
    }

    return sheet.solve_all_lines();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_6() {
        assert_eq!(solve_all_lines(), 4277556);
    }
}
