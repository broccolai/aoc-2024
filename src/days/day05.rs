use std::collections::{HashMap, HashSet};
use yaah::{aoc, aoc_generator, aoc_test};

use crate::utilities::tuple::MapTuple;

pub struct Manual {
    prerequisites: HashMap<u32, HashSet<u32>>,
    pages: Vec<Vec<u32>>,
}

#[aoc(day5, part1)]
fn part_one(manual: &Manual) -> u32 {
    let Manual {
        prerequisites,
        pages,
    } = manual;

    pages
        .iter()
        .filter(|page| is_ordered(prerequisites, page))
        .map(|page| middle_entry(page))
        .sum()
}

#[aoc(day5, part2)]
fn part_two(manual: &Manual) -> u32 {
    let Manual {
        prerequisites,
        pages,
    } = manual;

    pages
        .iter()
        .filter(|page| !is_ordered(prerequisites, page))
        .map(|page| {
            let reorded_page = reorder_page(prerequisites, page);
            middle_entry(&reorded_page)
        })
        .sum()
}

#[aoc_test(day5, part1)]
fn test_part_one() -> u32 {
    143
}

#[aoc_test(day5, part2)]
fn test_part_two() -> u32 {
    123
}

#[aoc_generator(day5)]
fn generator(input: &'static str) -> Option<Manual> {
    let (rules_input, pages_input) = input.split_once("\n\n")?;

    let prerequisites: HashMap<u32, HashSet<u32>> = rules_input
        .lines()
        .filter_map(|line| line.split_once('|'))
        .map_tuple(|a| a.parse::<u32>().unwrap(), |b| b.parse::<u32>().unwrap())
        .fold(HashMap::new(), |mut acc, (x, y)| {
            acc.entry(y).or_default().insert(x);
            acc
        });

    let pages: Vec<Vec<u32>> = pages_input
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|num| num.parse::<u32>().ok())
                .collect()
        })
        .collect();

    Some(Manual {
        prerequisites,
        pages,
    })
}

fn is_ordered(prerequisites: &HashMap<u32, HashSet<u32>>, page: &[u32]) -> bool {
    page.is_sorted_by(|a, b| {
        // For each consecutive pair (a, b), check if 'a' is a prerequisite of 'b'
        // If 'b' has no prerequisites or 'a' is among them, it's correctly ordered.
        prerequisites.get(b).map_or(false, |deps| deps.contains(a))
    })
}

fn reorder_page(prerequisites: &HashMap<u32, HashSet<u32>>, page: &[u32]) -> Vec<u32> {
    let mut reorder_page = page.to_owned();
    reorder_page.sort_by(|a, b| {
        // If 'a' must come before 'b', 'a' should be less than 'b'
        // We use `cmp` with boolean to prioritize correctly ordered pairs.
        prerequisites
            .get(b)
            .map_or(false, |deps| deps.contains(a))
            .cmp(&true)
    });
    reorder_page
}

const fn middle_entry(page: &[u32]) -> u32 {
    page[page.len() / 2]
}
