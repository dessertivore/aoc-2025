use aoc_2025::utils::{get_input::get_aoc_input, parsing::split_lines};

/// Runs the solution for Advent of Code Day 6.
pub fn main() {
    println!(
        "Day 6! Part 1: {:?}, Part 2: {:?}",
        solve_all_lines(true),
        solve_all_lines(false)
    );
}

/// Represents a maths sheet containing lines of numbers and a list of instructions.
///
/// # Fields
///
/// * `lines_of_numbers` - A vector of vectors, where each inner vector contains the numbers for a line.
/// * `instructions` - A vector of strings representing the operation to apply to each line.
#[derive(Debug)]
struct MathsSheet {
    lines_of_numbers: Vec<Vec<u64>>,
    instructions: Vec<String>,
}

/// Processes the input for Part 1 by splitting each line into numbers and collecting them row-wise.
///
/// # Arguments
///
/// * `input` - A vector of strings, each representing a line of numbers separated by whitespace.
///
/// # Returns
///
/// A vector of vectors of `u64`, where each inner vector contains the numbers from a line.
fn process_input_part_1(input: Vec<String>) -> Vec<Vec<u64>> {
    let mut output_lines = Vec::new();
    for (pos, line) in input.iter().enumerate() {
        let chars = line.split_whitespace();
        if output_lines.is_empty() {
            let char_count = chars.count();
            output_lines = vec![Vec::new(); char_count]; // Instantiate vertical lines
        }
        for char in line.split_whitespace() {
            output_lines[pos].push(char.parse::<u64>().unwrap());
        }
    }
    output_lines
}

/// Processes the input for Part 1 by splitting each line into numbers and collecting them row-wise.
///
/// # Arguments
///
/// * `input` - A vector of strings, each representing a line of numbers separated by whitespace.
///
/// # Returns
///
/// A vector of vectors of `u64`, where each inner vector contains the numbers from a line.
fn process_input_part_2(input: Vec<String>) -> Vec<Vec<u64>> {
    println!("Finding whitespaces in input for part 2.");
    let line_len = input[0].len();
    let mut columns: Vec<Option<u64>> = vec![Some(0); line_len];

    for (i, column) in columns.iter_mut().enumerate() {
        if input
            .iter()
            .all(|line| line.chars().nth(i).is_some_and(|c| c.is_whitespace()))
        {
            *column = None;
        }
    }

    // Build numbers column-wise, keeping x_coord and y_coord
    for (y_coord, line) in input.iter().enumerate() {
        println!("Line {:?} being parsed.", y_coord);
        for (x_coord, ch) in line.chars().enumerate() {
            if columns[x_coord].is_none() {
                continue;
            } else if let Some(num) = ch.to_digit(10) {
                columns[x_coord] = columns[x_coord].map(|current| current * 10 + num as u64);
            }
        }
    }

    let mut fully_parsed = Vec::new();
    let mut current_nums: Vec<u64> = Vec::new();
    for item in columns.iter().take(line_len) {
        if item.is_none() {
            fully_parsed.push(current_nums);
            current_nums = Vec::new();
        } else {
            current_nums.push(item.unwrap());
        }
    }
    fully_parsed.push(current_nums); // push last vec

    fully_parsed
}

impl MathsSheet {
    /// Processes the input for Part 1 by splitting each line into numbers and collecting them row-wise.
    ///
    /// # Arguments
    ///
    /// * `input` - A vector of strings, each representing a line of numbers separated by whitespace.
    ///
    /// # Returns
    ///
    /// A vector of vectors of `u64`, where each inner vector contains the numbers from a line.
    fn solve_line(&self, line: &Vec<u64>, operator_pos: usize) -> u64 {
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

    /// Solves all lines in the maths sheet by applying the corresponding operator to each line and summing the results.
    ///
    /// # Returns
    ///
    /// The grand total after processing all lines.
    fn solve_all_lines(&self) -> u64 {
        let mut grand_total: u64 = 0;
        for pos in 0..self.lines_of_numbers.len() {
            grand_total += self.solve_line(&self.lines_of_numbers[pos], pos);
        }

        grand_total
    }
}

/// Processes the input and solves all lines for either Part 1 or Part 2.
///
/// # Arguments
///
/// * `part_1` - A boolean indicating whether to use Part 1 (`true`) or Part 2 (`false`) processing.
///
/// # Returns
///
/// The final result as a `u64` after processing all lines and applying the instructions.
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
    if part_1 {
        sheet.lines_of_numbers = process_input_part_1(input);

        if cfg!(test) {
            println!("{:?}", sheet);
        }

        sheet.solve_all_lines()
    } else {
        sheet.lines_of_numbers = process_input_part_2(input);

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
    #[test]
    fn test_day_6_part_2() {
        assert_eq!(solve_all_lines(false), 3263827);
    }
}
