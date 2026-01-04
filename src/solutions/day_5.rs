use std::{cmp::max, collections::HashSet, ops::RangeInclusive};

use crate::utils::{
    get_input::get_aoc_input, parsing::split_lines, parsing::split_string_by_specified_char,
};

/// Runs the solution for Advent of Code Day 5.
pub fn day_5() {
    let part_1 = num_valid_ingredients();
    let part_2 = parse_input();
    println!(
        "Day 5! Part 1: {:?}, Part 2: {:?}",
        part_1, part_2.current_num_valid_ids
    );
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

    fn add_range(&mut self, new_range: RangeInclusive<u64>) {
        self.current_num_valid_ids += new_range.end().abs_diff(*new_range.start());
        self.valid_ranges.push(new_range);
    }

    fn aggregate_ranges(&mut self) {
        self.valid_ranges.sort_by(|x, y| x.start().cmp(y.start()));
        let mut idx_a = 0;
        let mut idx_b = 1;

        while idx_a < self.valid_ranges.len() - 1 {
            // let's go through every single range and aggregate them!

            let range_a = &self.valid_ranges[idx_a];
            let range_b = &self.valid_ranges[idx_b];
            if range_a.contains(range_b.start()) {
                // if start of range B is within range A
                if range_a.contains(range_b.end()) {
                    // if range b is totally within range a
                    self.current_num_valid_ids -= range_b.end().abs_diff(*range_b.start()); // remove old range_b sum
                    self.valid_ranges.remove(idx_b); // range b is totally within range a, just remove
                                                     // idx_b stays the same
                    continue;
                } else {
                    let new_end = max(*range_a.end(), *range_b.end());
                    let new_range = *range_a.start()..=new_end;

                    self.valid_ranges.remove(idx_b); // remove 2nd range, which is now part of new
                    self.valid_ranges.remove(idx_a); // remove 1st range, which is now part of new
                    self.valid_ranges.insert(idx_a, new_range);
                }
                if idx_b == self.valid_ranges.len() {
                    idx_a += 1;
                    idx_b = idx_a + 1; // if we've checked through all later ranges, then time to increment idx_a
                } else {
                    // we deleted item at idx_b so no need to change idx_b
                }
            } else {
                idx_a += 1;
                idx_b = idx_a + 1;
            }
        }

        let new_valid_ids: u64 = self
            .valid_ranges
            .iter()
            .map(|x| x.end().abs_diff(*x.start()) + 1) // +1 so we take into account upper bound!
            .sum();
        self.current_num_valid_ids = new_valid_ids;
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
    }

    ingredients_parsed.aggregate_ranges(); // this is key for part 2!

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
    }

    #[test]
    fn test_day_5_part_2() {
        assert_eq!(parse_input().current_num_valid_ids, 14);
    }
    #[test]
    fn test_day_5_part_2_extra_test() {
        let mut test = parse_input();
        test.add_range(RangeInclusive::new(20, 30));
        test.aggregate_ranges();
        assert_eq!(test.current_num_valid_ids, 24);
    }
}
