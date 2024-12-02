use std::iter::zip;

use crate::utilities::tuple::MapTuple;
use yaah::{aoc, aoc_test};

#[aoc(day1, part1)]
fn part_one(data: &'static str) -> u32 {
    let (mut left, mut right) = parse(data);

    left.sort_unstable();
    right.sort_unstable();

    zip(left, right).map(|(l, r)| u32::abs_diff(l, r)).sum()
}

#[aoc_test(day1, part1)]
fn test_part_one() -> u32 {
    11
}

#[aoc(day1, part2)]
fn part_two(data: &'static str) -> u32 {
    let (left, right) = parse(data);

    left.iter()
        .map(|&left_num| {
            let right_count = right
                .iter()
                .filter(|&&right_num| left_num == right_num)
                .count() as u32;

            left_num * right_count
        })
        .sum()
}

#[aoc_test(day1, part2)]
fn test_part_two() -> u32 {
    31
}

fn parse(data: &'static str) -> (Vec<u32>, Vec<u32>) {
    data.lines()
        .filter_map(|line| line.split_once("   "))
        .map_tuple(
            |left| left.parse::<u32>().unwrap(),
            |right| right.parse::<u32>().unwrap(),
        )
        .unzip()
}
