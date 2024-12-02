use std::{fmt::Debug, str::FromStr};

use grid::Grid;
use itertools::Itertools;

pub fn split_once_and_trim(source: &str, delimiter: char) -> Option<(&str, &str)> {
    source
        .split_once(delimiter)
        .map(|(first, second)| (first.trim(), second.trim()))
}

pub fn remove_before_once_and_trim(source: &str, delimiter: char) -> &str {
    source.split_once(delimiter).unwrap().1.trim()
}

pub fn parse_vec_of_numbers<N: FromStr>(source: &str) -> Vec<N>
where
    N::Err: Debug,
{
    source
        .split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

pub fn parse_grid<T>(source: &str, mapper: fn(char) -> T) -> Grid<T> {
    let values: Vec<Vec<T>> = source
        .lines()
        .map(|line| line.chars().map(mapper).collect_vec())
        .collect_vec();

    let width = values.first().unwrap().len();
    let flattened_values = values.into_iter().flatten().collect_vec();

    Grid::from_vec(flattened_values, width)
}
