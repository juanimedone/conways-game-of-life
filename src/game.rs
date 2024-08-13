/// Represents the state of the Game of Life.
pub struct Game {
    pub cells: Vec<Vec<bool>>,
    ncols: usize,
    nrows: usize,
}

impl Game {
    /// Creates a new `Game` instance with all cells initially dead.
    ///
    /// This function initializes a new `Game` with the given dimensions and prepares a grid where all cells are dead.
    ///
    /// # Arguments
    ///
    /// * `ncols` - The number of columns in the game grid.
    /// * `nrows` - The number of rows in the game grid.
    ///
    /// # Returns
    ///
    /// A new `Game` instance with the specified dimensions, and with all cells initially set to `false` (dead).
    pub fn new(ncols: usize, nrows: usize) -> Self {
        let cells = vec![vec![false; nrows]; ncols];
        Self {
            cells,
            ncols,
            nrows,
        }
    }

    /// This function changes the state of the cell at the given `(x, y)` coordinates from alive to dead or vice versa.
    ///
    /// # Arguments
    ///
    /// * `x` - The column index of the cell to be toggled. It must be within the range `[0, ncols)`.
    /// * `y` - The row index of the cell to be toggled. It must be within the range `[0, nrows)`.
    pub fn toggle_cell_state(&mut self, x: usize, y: usize) {
        if x < self.ncols && y < self.nrows {
            self.cells[x][y] = !self.cells[x][y];
        }
    }

    /// Randomizes the state of all cells in the grid.
    ///
    /// This function sets each cell in the grid to a random state (alive or dead).
    pub fn randomize(&mut self) {
        self.cells = (0..self.cells.len())
            .map(|_| (0..self.cells[0].len()).map(|_| ::rand::random()).collect())
            .collect();
    }

    /// Updates the game state to the next generation.
    ///
    /// This function calculates the next state of the game based on the current state
    /// and updates the cells accordingly.
    pub fn update(&mut self) {
        let mut next_cells = vec![vec![false; self.nrows]; self.ncols];

        #[allow(clippy::needless_range_loop)] // this way is clearer than how Clippy suggests
        for x in 0..self.ncols {
            for y in 0..self.nrows {
                let cell = self.cells[x][y];
                let neighbors = self.count_neighbors(x as i32, y as i32);

                next_cells[x][y] = matches!((cell, neighbors), (true, 2) | (true, 3) | (false, 3));
            }
        }
        self.cells = next_cells;
    }

    /// Counts the number of alive neighbors for the given cell.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the cell.
    /// * `y` - The y coordinate of the cell.
    ///
    /// # Returns
    ///
    /// The number of alive neighbors.
    fn count_neighbors(&self, x: i32, y: i32) -> usize {
        let mut count = 0;
        for dx in -1..=1 {
            let nx = x + dx;
            if nx < 0 || nx >= self.ncols as i32 {
                // checks if neighbor's x is out of bounds
                continue;
            }
            for dy in -1..=1 {
                let ny = y + dy;
                if ny < 0 || ny >= self.nrows as i32 {
                    // checks if neighbor's y is out of bounds
                    continue;
                }
                if nx == x && ny == y {
                    continue;
                }
                if self.cells[nx as usize][ny as usize] {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_game() {
        let game = Game::new(40, 30);
        assert_eq!(game.cells.len(), 40);
        assert_eq!(game.cells[0].len(), 30);
    }

    #[rustfmt::skip]
    #[test]
    fn test_count_neighbors() {
        let mut game = Game::new(5, 5);
        game.cells = vec![
            vec![false, true, false, true, false],
            vec![true, true, true, false, true],
            vec![false, false, true, false, false],
            vec![true, false, false, true, true],
            vec![false, true, false, true, false],
        ];
        assert_eq!(game.count_neighbors(0, 0), 3);
        assert_eq!(game.count_neighbors(0, 4), 2);
        assert_eq!(game.count_neighbors(2, 2), 3);
        assert_eq!(game.count_neighbors(4, 4), 3);
    }

    #[rustfmt::skip]
    #[test]
    fn test_update() {
        let mut game = Game::new(5, 5);
        game.cells = vec![
            vec![false, true, false, true, false],
            vec![true, true, true, false, true],
            vec![false, false, true, false, false],
            vec![true, false, false, true, true],
            vec![false, true, false, true, false],
        ];
        game.update();
        let expected = vec![
            vec![true, true, false, true, false],
            vec![true, false, false, false, false],
            vec![true, false, true, false, true],
            vec![false, true, false, true, true],
            vec![false, false, true, true, true],
        ];
        assert_eq!(game.cells, expected);
    }
}
