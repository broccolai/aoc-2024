use grid::Grid;
use yaah::{aoc, aoc_generator, aoc_test};

use crate::utilities::{
    grid::{Direction, Position, PositonGrid, DIRECTIONS},
    string::parse_grid,
};

type Search = Grid<Token>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    X,
    M,
    A,
    S,
}

#[aoc_generator(day4)]
fn generator(input: &'static str) -> Search {
    parse_grid(input, |char| match char {
        'X' => Token::X,
        'M' => Token::M,
        'A' => Token::A,
        'S' => Token::S,
        _ => unreachable!(),
    })
}

#[aoc(day4, part1)]
fn part_one(search: &Search) -> usize {
    search
        .indexed_iter()
        .filter(|&(_, token)| token == &Token::X)
        .map(|(position, _)| position)
        .flat_map(|position| {
            DIRECTIONS
                .iter()
                .map(move |&direction| (position, direction))
        })
        .filter(|&(position, direction)| matches_word(search, position, direction))
        .count()
}

#[aoc_test(day4, part1)]
fn test_part_one() -> usize {
    18
}

#[aoc(day4, part2)]
fn part_two(search: &Search) -> usize {
    const ASCENDING: (Direction, Direction) = (Direction::UpRight, Direction::DownLeft);
    const DESCENDING: (Direction, Direction) = (Direction::UpLeft, Direction::DownRight);

    search
        .indexed_iter()
        .filter(|&(_, token)| token == &Token::A)
        .map(|(pos, _)| Position::from_tuple(pos))
        .filter(|&pos| {
            matches_x_mas(search, pos, ASCENDING) && matches_x_mas(search, pos, DESCENDING)
        })
        .count()
}

#[aoc_test(day4, part2)]
fn test_part_two() -> usize {
    9
}

fn matches_word(grid: &Search, temp: (usize, usize), direction: Direction) -> bool {
    const WORD: &[Token] = &[Token::X, Token::M, Token::A, Token::S];

    let position = Position::from_tuple(temp);
    let direction_position = direction.to_position();

    WORD.iter().enumerate().all(|(index, expected)| {
        let adjusted_direction = direction_position * index;

        position
            .add(adjusted_direction)
            .and_then(|pos| grid.get_with_position(pos))
            .map_or(false, |token| token == expected)
    })
}

fn matches_x_mas(grid: &Search, position: Position, directions: (Direction, Direction)) -> bool {
    const VALID_SEQUENCES: [(&Token, &Token); 2] = [(&Token::M, &Token::S), (&Token::S, &Token::M)];

    let (up_direction, down_direction) = directions;
    let found_positions = Option::zip(
        grid.get_with_direction(position, up_direction),
        grid.get_with_direction(position, down_direction),
    );

    if found_positions.is_none() {
        return false;
    }

    let positions = found_positions.unwrap();

    VALID_SEQUENCES.contains(&positions)
}
