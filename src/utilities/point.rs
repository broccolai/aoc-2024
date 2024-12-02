#[derive(Debug, Clone)]
pub struct Point<N> {
    pub x: N,
    pub y: N,
}

impl<N: Ord + Copy> Point<N> {
    pub const fn from_tuple(source: (N, N)) -> Self {
        Self {
            x: source.0,
            y: source.1,
        }
    }

    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: N::min(self.x, other.x),
            y: N::min(self.y, other.y),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: N::max(self.x, other.x),
            y: N::max(self.y, other.y),
        }
    }
}

impl Point<usize> {
    pub const fn manhatten_distance(&self, other: &Self) -> usize {
        usize::abs_diff(self.x, other.x) + usize::abs_diff(self.y, other.y)
    }
}
