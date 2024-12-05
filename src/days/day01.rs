use std::iter::zip;

use crate::utilities::tuple::MapTuple;
use yaah::{aoc, aoc_generator, aoc_test};

type Lists = (Vec<usize>, Vec<usize>);

#[aoc_generator(day1)]
fn generator(data: &'static str) -> Lists {
    data.lines()
        .filter_map(|line| line.split_once("   "))
        .map_tuple(
            |left| left.parse::<usize>().unwrap(),
            |right| right.parse::<usize>().unwrap(),
        )
        .unzip()
}

#[aoc(day1, part1)]
fn part_one(lists: &Lists) -> usize {
    let (mut left, mut right) = lists.clone();

    left.sort_unstable();
    right.sort_unstable();

    zip(left, right).map(|(l, r)| usize::abs_diff(l, r)).sum()
}

#[aoc_test(day1, part1)]
fn test_part_one() -> usize {
    11
}

#[aoc(day1, part2)]
fn part_two(lists: &Lists) -> usize {
    let (left, right) = lists;

    left.iter()
        .map(|&left_num| {
            let right_count = right
                .iter()
                .filter(|&&right_num| left_num == right_num)
                .count();

            left_num * right_count
        })
        .sum()
}

#[aoc_test(day1, part2)]
fn test_part_two() -> usize {
    31
}
