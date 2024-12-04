use regex::{Captures, Regex};
use yaah::{aoc, aoc_test};

#[aoc(day3, part1)]
fn part_one(data: &'static str) -> u32 {
    let regex = Regex::new(r"mul\(\s*\d+\s*,\s*\d+\s*\)").unwrap();

    regex
        .captures_iter(data)
        .map(|capture| parse_instruction(&capture))
        .map(|(a, b)| a * b)
        .sum()
}

#[aoc_test(day3, part1)]
fn test_part_one() -> u32 {
    161
}

#[aoc(day3, part2)]
fn part_two(data: &'static str) -> u32 {
    let regex_mul = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let regex_control = Regex::new(r"(do\(\)|don't\(\))").unwrap();
    
    let mut enabled = true;   
    let mut total = 0;
    let mut pointer = 0;

    for i in 1..data.len() {
        let input = &data[pointer..i];
        let capture = regex_mul.captures_iter(input).last();

        if capture.is_none() {
            continue;
        }

        let potential_control = regex_control
            .captures_iter(input)
            .last()
            .map(|control| control.get(0).unwrap().as_str());

        if let Some(control) = potential_control {
            match control {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {}
            }
        }

        let (a, b) = parse_instruction(&capture.unwrap());

        if enabled {
            total += a * b;
        }

        pointer = i;
    }

    total
}

#[aoc_test(day3, part2)]
fn test_part_two() -> u32 {
    48
}

fn parse_instruction(capture: &Captures<'_>) -> (u32, u32) {
    let (a, b) = capture
        .get(0)
        .unwrap()
        .as_str()
        .trim_start_matches("mul(")
        .trim_end_matches(')')
        .split_once(',')
        .unwrap();

    (a.parse().unwrap(), b.parse().unwrap())
}
