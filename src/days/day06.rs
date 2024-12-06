use std::collections::HashSet;

use grid::Grid;
use yaah::{aoc, aoc_generator, aoc_test};

use crate::utilities::{
    grid::{Direction, Position, PositonGrid},
    string::parse_grid,
};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Token {
    Empty,
    Obstacle,
    Visited,
}

#[derive(Clone)]
pub struct Map {
    position: Position,
    direction: Direction,
    layout: Grid<Token>,
}

#[aoc(day6, part1)]
fn part_one(input: &Map) -> usize {
    let Map {
        mut position,
        mut direction,
        mut layout,
    } = input.clone();

    while let Some((next_position, &token)) = layout.get_with_direction_indexed(position, direction)
    {
        if token == Token::Obstacle {
            direction = next_direction(direction);
            continue;
        }

        layout.set_with_position(next_position, Token::Visited);
        position = next_position;
    }

    layout
        .iter()
        .filter(|&token| *token == Token::Visited)
        .count()
}

#[aoc(day6, part2)]
fn part_two(input: &Map) -> usize {
    let Map {
        position,
        direction,
        layout,
    } = input;

    layout
        .indexed_iter()
        .filter(|(_, &token)| token == Token::Empty)
        .map(|(position, _)| Position::from_tuple(position))
        .filter(|potential_obstacle_position| {
            let mut test_layout = layout.clone();
            test_layout.set_with_position(*potential_obstacle_position, Token::Obstacle);

            simulate_and_check_loop(*position, *direction, &test_layout)
        })
        .count()
}

#[aoc_test(day6, part1)]
fn test_part_one() -> usize {
    41
}

#[aoc_test(day6, part2)]
fn test_part_two() -> usize {
    6
}

#[aoc_generator(day6)]
fn generator(input: &'static str) -> Option<Map> {
    let layout = parse_grid(input, |char| match char {
        '.' => Token::Empty,
        '#' => Token::Obstacle,
        '^' => Token::Visited,
        _ => unreachable!(),
    });

    let position = layout
        .indexed_iter()
        .find(|(_, &token)| token == Token::Visited)
        .map(|(coords, _)| Position::from_tuple(coords))?;

    Some(Map {
        position,
        direction: Direction::Up,
        layout,
    })
}

fn next_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        _ => unreachable!(),
    }
}

fn simulate_and_check_loop(
    mut position: Position,
    mut direction: Direction,
    layout: &Grid<Token>,
) -> bool {
    let mut visited_states = HashSet::new();
    visited_states.insert((position, direction));

    while let Some((next_pos, &token)) = layout.get_with_direction_indexed(position, direction) {
        let (new_position, new_direction) = if token == Token::Obstacle {
            (position, next_direction(direction))
        } else {
            (next_pos, direction)
        };

        if !visited_states.insert((new_position, new_direction)) {
            return true;
        }

        position = new_position;
        direction = new_direction;
    }

    false
}
