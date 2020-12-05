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

#[derive(Debug, Clone)]
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
}
