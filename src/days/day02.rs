use std::ops::RangeInclusive;

use itertools::Itertools;
use yaah::{aoc, aoc_test};

const PART_ONE_VALID_RANGE: RangeInclusive<u32> = 1..=3;

#[aoc(day2, part1)]
fn part_one(data: &'static str) -> u32 {
    data.lines()
        .map(parse_line)
        .filter(numbers_are_valid)
        .count() as u32
}

#[aoc_test(day2, part1)]
fn test_part_one() -> u32 {
    2
}

#[aoc(day2, part2)]
fn part_two(data: &'static str) -> u32 {
    data.lines()
        .map(parse_line)
        .filter(|numbers| {
            if numbers_are_valid(numbers) {
                return true;
            }

            (0..numbers.len())
                .map(|i| skipping_index_of_vec(numbers, i))
                .any(|filtered| numbers_are_valid(&filtered))
        })
        .count() as u32
}

#[aoc_test(day2, part2)]
fn test_part_two() -> u32 {
    4
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect_vec()
}

fn skipping_index_of_vec(vec: &Vec<u32>, index: usize) -> Vec<u32> {
    vec.iter()
        .enumerate()
        .filter(|&(idx, _)| idx != index)
        .map(|(_, &num)| num)
        .collect_vec()
}

fn numbers_are_valid(numbers: &Vec<u32>) -> bool {
    if !numbers.is_sorted() && !numbers.is_sorted_by(|a, b| a > b) {
        return false;
    }

    numbers
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(&a, &b)| u32::abs_diff(a, b))
        .all(|diff| PART_ONE_VALID_RANGE.contains(&diff))
}
