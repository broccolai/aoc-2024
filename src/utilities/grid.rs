use std::ops::Mul;

use grid::Grid;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Position {
    pub row: i32,
    pub column: i32,
}

impl Position {
    pub const fn from_tuple(source: (usize, usize)) -> Self {
        Self {
            row: source.0 as i32,
            column: source.1 as i32,
        }
    }

    pub const fn add(self, delta: Self) -> Option<Self> {
        let next_row = self.row + delta.row;
        let next_column = self.column + delta.column;

        if next_row.is_negative() || next_column.is_negative() {
            return None;
        }

        Some(Self {
            row: next_row,
            column: next_column,
        })
    }
}

impl Mul<usize> for Position {
    type Output = Self;

    fn mul(self, scalar: usize) -> Self {
        Self {
            row: self.row * scalar as i32,
            column: self.column * scalar as i32,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum Direction {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

pub const DIRECTIONS: [Direction; 8] = [
    Direction::UpLeft,
    Direction::Up,
    Direction::UpRight,
    Direction::Left,
    Direction::Right,
    Direction::DownLeft,
    Direction::Down,
    Direction::DownRight,
];

impl Direction {
    pub const fn to_position(self) -> Position {
        match self {
            Self::UpLeft => Position {
                row: -1,
                column: -1,
            },
            Self::Up => Position { row: -1, column: 0 },
            Self::UpRight => Position { row: -1, column: 1 },
            Self::Left => Position { row: 0, column: -1 },
            Self::Right => Position { row: 0, column: 1 },
            Self::DownLeft => Position { row: 1, column: -1 },
            Self::Down => Position { row: 1, column: 0 },
            Self::DownRight => Position { row: 1, column: 1 },
        }
    }
}

pub trait PositonGrid<T> {
    fn get_with_position(&self, positon: Position) -> Option<&T>;

    fn set_with_position(&mut self, position: Position, value: T);

    fn get_with_direction(&self, position: Position, direction: Direction) -> Option<&T>;

    fn get_with_direction_indexed(
        &self,
        position: Position,
        direction: Direction,
    ) -> Option<(Position, &T)>;
}

impl<T> PositonGrid<T> for Grid<T> {
    fn get_with_position(&self, positon: Position) -> Option<&T> {
        if positon.column.is_negative() || positon.row.is_negative() {
            return None;
        }

        self.get(positon.row as usize, positon.column as usize)
    }

    fn set_with_position(&mut self, position: Position, value: T) {
        self.set(position.row as usize, position.column as usize, value);
    }

    fn get_with_direction(&self, position: Position, direction: Direction) -> Option<&T> {
        self.get_with_direction_indexed(position, direction)
            .map(|(_, value)| value)
    }

    fn get_with_direction_indexed(
        &self,
        position: Position,
        direction: Direction,
    ) -> Option<(Position, &T)> {
        let adjusted_position = position.add(direction.to_position())?;

        self.get_with_position(adjusted_position)
            .map(|value| (adjusted_position, value))
    }
}

pub trait UtilityGrid<T> {
    fn set(&mut self, row: usize, column: usize, value: T);
}

impl<T> UtilityGrid<T> for Grid<T> {
    fn set(&mut self, row: usize, column: usize, value: T) {
        *self.get_mut(row, column).unwrap() = value;
    }
}
