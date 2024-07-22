use macroquad::{
    color::{BLACK, WHITE},
    shapes::draw_rectangle,
    window::{clear_background, next_frame},
};

/// Represents the state of the Game of Life.
pub struct Game {
    cell_size: usize,
    cells: Vec<Vec<bool>>,
}

impl Game {
    /// Creates a new Game of Life with a random initial state.
    ///
    /// The function will adjust the number of rows and columns based on the given
    /// `cell_size` and will initialize the cells with random boolean values.
    /// 
    /// # Arguments 
    ///
    /// * `height` - The height of the game grid.
    /// * `width` - The width of the game grid.
    /// * `cell_size` - The size of each cell.
    ///
    /// # Returns
    ///
    /// `Ok(Self)` if the parameters are valid, containing the new `Game` instance,
    /// or `Err(String)` with an error message if any of the parameters is invalid.
    pub fn new(height: usize, width: usize, cell_size: usize) -> Result<Self, String> {
        Self::validate_start_parameters(height, width, cell_size)?;

        let nrows = height / cell_size;
        let ncols = width / cell_size;
        let cells = (0..ncols)
            .map(|_| (0..nrows).map(|_| rand::random()).collect())
            .collect();
        Ok(Self {
            cell_size,
            cells,
        })
    }

    /// Validates the start parameters for the game.
    ///
    /// # Arguments
    ///
    /// * `height` - The height of the game grid.
    /// * `width` - The width of the game grid.
    /// * `cell_size` - The size of each cell.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the parameters are valid, otherwise an error message.
    /// 
    /// # Errors
    ///
    /// This function will return an error if `height`, `width` or `cell_size` is 0
    fn validate_start_parameters(
        height: usize,
        width: usize,
        cell_size: usize,
    ) -> Result<(), String> {
        if height == 0 || width == 0 || cell_size == 0 {
            return Err("Neither height, width nor cell size can be 0".to_string());
        }
        Ok(())
    }

    /// Starts the Game of Life.
    ///
    /// This function enters an infinite loop to continuously update and draw the game state.
    pub async fn start(&mut self) {
        loop {
            clear_background(BLACK);

            self.update();
            self.draw();

            next_frame().await
        }
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
            if nx < 0 || nx >= self.cells.len() as i32 {
                // checks if neighbor's x is out of bounds
                continue;
            }
            for dy in -1..=1 {
                let ny = y + dy;
                if ny < 0 || ny >= self.cells[0].len() as i32 {
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

    /// Updates the game state to the next generation.
    fn update(&mut self) {
        let nrows = self.cells.len();
        let ncols = self.cells[0].len();
        let mut next_cells = vec![vec![false; ncols]; nrows];
        
        for x in 0..next_cells.len() {
            for y in 0..next_cells[0].len() {
                let cell = self.cells[x][y];
                let neighbors = self.count_neighbors(x as i32, y as i32);

                next_cells[x][y] = matches!((cell, neighbors), (true, 2) | (true, 3) | (false, 3));
            }
        }
        self.cells = next_cells;
    }

    /// Draws the current game state.
    ///
    /// This function renders the cells of the game onto the screen using the cell size
    /// to determine their position and dimensions.
    fn draw(&self) {
        for x in 0..self.cells.len() {
            for y in 0..self.cells[0].len() {
                if self.cells[x][y] {
                    draw_rectangle(
                        (x * self.cell_size) as f32,
                        (y * self.cell_size) as f32,
                        self.cell_size as f32,
                        self.cell_size as f32,
                        WHITE,
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Game;

    #[test]
    fn test_valid_start_parameters() {
        assert!(Game::validate_start_parameters(600, 800, 10).is_ok())
    }

    #[test]
    fn test_invalid_start_parameters() {
        match Game::validate_start_parameters(0, 800, 10) {
            Err(e) => assert_eq!(e, "Neither height, width nor cell size can be 0"),
            _ => panic!("Expected an error"),
        }
        match Game::validate_start_parameters(600, 0, 10) {
            Err(e) => assert_eq!(e, "Neither height, width nor cell size can be 0"),
            _ => panic!("Expected an error"),
        }
        match Game::validate_start_parameters(600, 800, 0) {
            Err(e) => assert_eq!(e, "Neither height, width nor cell size can be 0"),
            _ => panic!("Expected an error"),
        }
    }

    #[test]
    fn test_create_new_valid_game() {
        let game = Game::new(600, 800, 10).unwrap();
        assert_eq!(game.cells.len(), 80);
        assert_eq!(game.cells[0].len(), 60);
    }

    #[rustfmt::skip]
    #[test]
    fn test_count_neighbors() {
        let mut game = Game::new(5, 5, 1).unwrap();
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
        let mut game = Game::new(5, 5, 1).unwrap();
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
