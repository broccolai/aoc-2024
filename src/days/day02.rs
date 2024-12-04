use std::ops::RangeInclusive;

use itertools::Itertools;
use yaah::{aoc, aoc_generator, aoc_test};

const VALID_RANGE: RangeInclusive<u32> = 1..=3;

#[aoc_generator(day2)]
fn generator(data: &'static str) -> Vec<Vec<u32>> {
    data.lines()
        .map(|line| line.split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect()
        )
        .collect_vec()
}

#[aoc(day2, part1)]
fn part_one(data: &[Vec<u32>]) -> usize {
    data.iter()
        .filter(|numbers| numbers_are_valid(numbers))
        .count()
}

#[aoc_test(day2, part1)]
fn test_part_one() -> usize {
    2
}

#[aoc(day2, part2)]
fn part_two(data: &[Vec<u32>]) -> usize {
    data.iter()
        .filter(|numbers| {
            if numbers_are_valid(numbers) {
                return true;
            }

            (0..numbers.len())
                .map(|i| skipping_index_of_vec(numbers, i))
                .any(|filtered| numbers_are_valid(&filtered))
        })
        .count()
}

#[aoc_test(day2, part2)]
fn test_part_two() -> usize {
    4
}

fn skipping_index_of_vec(vec: &[u32], index: usize) -> Vec<u32> {
    vec.iter()
        .enumerate()
        .filter(|&(idx, _)| idx != index)
        .map(|(_, &num)| num)
        .collect_vec()
}

fn numbers_are_valid(numbers: &[u32]) -> bool {
    if !numbers.is_sorted() && !numbers.is_sorted_by(|a, b| a > b) {
        return false;
    }

    numbers
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(&a, &b)| u32::abs_diff(a, b))
        .all(|diff| VALID_RANGE.contains(&diff))
}
