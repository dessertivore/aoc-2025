use crate::utils::{get_input::get_aoc_input, parsing::split_lines};

/// Runs the solution for Advent of Code Day 6.
pub fn day_6() -> u64 {
    let part_1 = solve_all_lines(true);
    println!("Day 6! Part 1: {:?}", part_1);

    part_1
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

        line_total
    }

    fn solve_all_lines(&self) -> u64 {
        let mut grand_total: u64 = 0;
        for pos in 0..self.lines_of_numbers.len() {
            grand_total += self.solve_line(self.lines_of_numbers[pos].clone(), pos);
        }

        grand_total
    }

    // fn part_2_line_parsing(&self) {
    //     for line in self.lines_of_numbers {}
    // }
}

fn solve_all_lines(part_1: bool) -> u64 {
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
    if part_1 {
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

        sheet.solve_all_lines()
    } else {
        // for line in input {
        //     let mut pos = 0;
        //     let nums = split_string_by_specified_char(line, " ");
        //     if vertical_lines.is_empty() {
        //         let char_count = nums.len(); // Clone the iterator to avoid consuming it
        //         vertical_lines = vec![Vec::new(); char_count]; // Instantiate vertical lines
        //     }
        //     for num in nums {
        //         let char_pos = 0;
        //         for char in num.chars() {
        //             vertical_lines[pos][char_pos]
        //         }
        //     }
        // I haven't finished this yet :(
        // }
        sheet.lines_of_numbers = vertical_lines;

        if cfg!(test) {
            println!("{:?}", sheet);
        }

        sheet.solve_all_lines()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_6() {
        assert_eq!(solve_all_lines(true), 4277556);
    }
}
