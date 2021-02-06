use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GridError {
    #[error("Could not parse grid with 0 width or height")]
    EmptyGrid,

    #[error("Could not parse grid with different length rows")]
    RowsNotSameLength,
}

pub type Result<T> = std::result::Result<T, GridError>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid<T: Clone> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(width: usize, height: usize, default: T) -> Self {
        let row: Vec<_> = std::iter::repeat(default).take(width).collect();
        let data = std::iter::repeat(row).take(height).collect();
        Self {
            data,
            width,
            height,
        }
    }

    pub fn with_data(data: Vec<Vec<T>>) -> Result<Self> {
        use GridError::*;

        if data.len() == 0 || data[0].len() == 0 {
            return Err(EmptyGrid);
        }

        let width = data[0].len();
        if !data.iter().map(|row| row.len()).all(|w| w == width) {
            return Err(RowsNotSameLength);
        }

        let width = data[0].len();
        let height = data.len();

        Ok(Self {
            data,
            width,
            height,
        })
    }

    pub fn data(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if !self.is_in_bounds(x, y) {
            return None;
        }

        Some(&self.data[y][x])
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if self.is_in_bounds(x, y) {
            self.data[y][x] = value;
        }
    }

    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// Get adjacent neighbors for the given position. Neighbors are returned in
    /// order of up-left, up, up-right, left, right, down-left, down,
    /// down-right.
    pub fn neighbors(&self, x: usize, y: usize) -> Option<Vec<&T>> {
        if !self.is_in_bounds(x, y) {
            return None;
        }

        let mut neighbors = Vec::new();

        let up = y > 0;
        let down = y + 1 < self.height;
        let left = x > 0;
        let right = x + 1 < self.width;

        if up && left {
            neighbors.push(&self.data[y - 1][x - 1]);
        }
        if up {
            neighbors.push(&self.data[y - 1][x]);
        }
        if up && right {
            neighbors.push(&self.data[y - 1][x + 1])
        }

        if left {
            neighbors.push(&self.data[y][x - 1]);
        }
        if right {
            neighbors.push(&self.data[y][x + 1]);
        }

        if down && left {
            neighbors.push(&self.data[y + 1][x - 1]);
        }
        if down {
            neighbors.push(&self.data[y + 1][x]);
        }
        if down && right {
            neighbors.push(&self.data[y + 1][x + 1]);
        }

        Some(neighbors)
    }
}

impl<T> TryFrom<Vec<T>> for Grid<char>
where
    T: ToString,
{
    type Error = GridError;

    fn try_from(value: Vec<T>) -> Result<Self> {
        let data: Vec<Vec<_>> = value
            .into_iter()
            .map(|s| s.to_string().chars().collect())
            .collect();

        Grid::with_data(data)
    }
}

impl std::str::FromStr for Grid<char> {
    type Err = GridError;

    fn from_str(s: &str) -> Result<Self> {
        let data = s.lines().map(|s| s.chars().collect()).collect();

        Grid::with_data(data)
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    use std::convert::TryFrom;

    #[test]
    fn it_converts_from_vec_of_string() {
        let input = vec![".#", "#."];
        let grid = Grid::try_from(input).unwrap();

        assert_eq!(grid.get(0, 0), Some(&'.'));
        assert_eq!(grid.get(0, 1), Some(&'#'));
        assert_eq!(grid.get(1, 0), Some(&'#'));
        assert_eq!(grid.get(1, 1), Some(&'.'));
    }

    #[test]
    fn it_gets_adjacent_neighbors() {
        let input = vec!["abc", "def", "ghi"];
        let grid = Grid::try_from(input).unwrap();

        assert_eq!(grid.neighbors(0, 0), Some(vec![&'b', &'d', &'e']));
        assert_eq!(
            grid.neighbors(1, 1),
            Some(vec![&'a', &'b', &'c', &'d', &'f', &'g', &'h', &'i'])
        );
        assert_eq!(grid.neighbors(2, 2), Some(vec![&'e', &'f', &'h']));
        assert_eq!(grid.neighbors(3, 3), None);
    }
}
