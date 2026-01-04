use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::RangeInclusive,
};

use crate::utils::{
    get_input::get_aoc_input, parsing::split_lines, parsing::split_string_by_specified_char,
};

/// Runs the solution for Advent of Code Day 5.
pub fn day_5() -> usize {
    let part_1 = num_valid_ingredients();
    println!("Day 5! Part 1: {:?}", part_1);

    part_1
}

struct KitchenInventory {
    valid_ranges: Vec<RangeInclusive<u64>>,
    ingredient_ids: HashSet<u64>,
    validated_ingredients: HashSet<u64>,
    current_num_valid_ids: u64,
}

impl KitchenInventory {
    fn validate_ingredients(&mut self) {
        for ingredient in self.ingredient_ids.clone() {
            for range in self.valid_ranges.clone() {
                if range.contains(&ingredient) {
                    self.validated_ingredients.insert(ingredient);
                }
            }
        }
    }

    fn add_range(&mut self, mut new_range: RangeInclusive<u64>) {
        let mut new_ranges: Vec<RangeInclusive<u64>> = Vec::new();
        let mut no_overlap: bool = true;
        while let Some(range) = self.valid_ranges.pop() {
            if range.contains(&new_range.start()) || range.contains(&new_range.end()) {
                let new_start = min(*range.start(), *new_range.start());
                let new_end = max(*range.end(), *new_range.end());
                new_range = new_start..=new_end;
                new_ranges.push(new_range.clone()); // extend range
                self.current_num_valid_ids += new_range.end().abs_diff(*new_range.start());
                self.current_num_valid_ids -= range.end().abs_diff(*range.start()); // remove old range sum
                no_overlap = false;
                break;
            } else {
                new_ranges.push(range); // keep old range
            }
        }
        if no_overlap {
            new_ranges.push(new_range.clone()); // add new range and new range sum
            self.current_num_valid_ids += new_range.end().abs_diff(*new_range.start());
        } else {
            new_ranges.extend(self.valid_ranges.clone()) //add old ranges back in
        }
        self.valid_ranges = new_ranges;
    }

    fn add_ranges(&mut self, new_ranges: Vec<RangeInclusive<u64>>) {
        for range in new_ranges {
            self.add_range(range);
        }
    }
}

fn parse_input() -> KitchenInventory {
    let input: Vec<String> = split_string_by_specified_char(get_aoc_input(2025, 5), "\n\n");
    let mut ingredients_parsed = KitchenInventory {
        valid_ranges: Vec::new(),
        ingredient_ids: HashSet::new(),
        validated_ingredients: HashSet::new(),
        current_num_valid_ids: 0,
    };
    for range in split_lines(input[0].clone()).iter() {
        let nums: Vec<u64> = range
            .split("-")
            .map(|num| {
                num.parse::<u64>()
                    .expect("Failed to convert range to numbers")
            })
            .collect();
        ingredients_parsed.add_range(RangeInclusive::new(nums[0], nums[1]));
        println!("{:?}", ingredients_parsed.valid_ranges)
    }
    let ingredient_ids: Vec<u64> = split_lines(input[1].clone())
        .into_iter()
        .map(|num| {
            num.parse::<u64>()
                .expect("Failed to convert IDs to numbers")
        })
        .collect();
    for id in ingredient_ids {
        ingredients_parsed.ingredient_ids.insert(id);
    }

    ingredients_parsed
}

fn num_valid_ingredients() -> usize {
    let mut ingredients_parsed: KitchenInventory = parse_input();
    ingredients_parsed.validate_ingredients();

    ingredients_parsed.validated_ingredients.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_5() {
        assert_eq!(num_valid_ingredients(), 3);
        assert_eq!(parse_input().current_num_valid_ids, 14);
    }
}
