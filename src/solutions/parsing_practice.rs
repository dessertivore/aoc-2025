use crate::utils::{
    get_input::get_aoc_input,
    parsing::{split_lines, split_string_by_specified_char},
};

pub fn practice() -> Vec<Vec<String>> {
    let input = split_lines(get_aoc_input(2024, 1));
    let mut all_split: Vec<Vec<String>> = vec![];
    for item in input.iter() {
        all_split.push(split_string_by_specified_char(item.to_string(), "   "));
    }
    return all_split;
}
