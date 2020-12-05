use super::grid::Grid;

/// A grid that wraps in the positive X direction
#[derive(Debug, Clone)]
pub struct WrapGrid<T: Clone> {
    grid: Grid<T>,
}

impl<T> WrapGrid<T>
where
    T: Clone,
{
    pub fn new(grid: Grid<T>) -> Self {
        Self { grid }
    }

    pub fn height(&self) -> usize {
        self.grid.height()
    }

    pub fn get(&self, mut x: usize, y: usize) -> Option<&T> {
        if y >= self.grid.height() {
            return None;
        }

        if x >= self.grid.width() {
            x %= self.grid.width();
        }

        self.grid.get(x, y)
    }
}
