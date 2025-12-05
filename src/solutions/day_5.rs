use std::collections::{btree_map::Range, HashSet};

use crate::utils::{
    get_input::get_aoc_input, parsing::split_lines, parsing::split_string_by_specified_char,
};

/// Runs the solution for Advent of Code Day 5.
pub fn day_5() -> usize {
    let part_1 = num_valid_ingredients();
    println!("Day 5! Part 1: {:?}", part_1);
    return part_1;
}

struct KitchenInventory {
    valid_ranges: HashSet<(u64, u64)>,
    ingredient_ids: HashSet<u64>,
    validated_ingredients: HashSet<u64>,
}

impl KitchenInventory {
    fn validate_ingredients(&mut self) {
        for ingredient in self.ingredient_ids.clone() {
            for (lower, upper) in self.valid_ranges.clone() {
                if (lower..upper).contains(&ingredient) {
                    self.validated_ingredients.insert(ingredient);
                }
            }
        }
    }
}

fn num_valid_ingredients() -> usize {
    let input: Vec<String> = split_string_by_specified_char(get_aoc_input(2025, 5), "\n\n");
    let mut ingredients_parsed = KitchenInventory {
        valid_ranges: HashSet::new(),
        ingredient_ids: HashSet::new(),
        validated_ingredients: HashSet::new(),
    };
    for range in split_lines(input[0].clone()).iter() {
        let nums: Vec<u64> = range
            .split("-")
            .map(|num| {
                num.parse::<u64>()
                    .expect("Failed to convert range to numbers")
            })
            .collect();
        ingredients_parsed
            .valid_ranges
            .insert((nums[0], nums[1] + 1));
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
    ingredients_parsed.validate_ingredients();
    return ingredients_parsed.validated_ingredients.len();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_5() {
        assert_eq!(num_valid_ingredients(), 3);
    }
}
