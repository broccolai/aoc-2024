use grid::Grid;

#[derive(Debug, Copy, Clone)]
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

pub trait PositonGrid<T> {
    fn get_with_position(&self, positon: Position) -> Option<&T>;
}

impl<T> PositonGrid<T> for Grid<T> {
    fn get_with_position(&self, positon: Position) -> Option<&T> {
        if positon.column.is_negative() || positon.row.is_negative() {
            return None;
        }

        self.get(positon.row as usize, positon.column as usize)
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
