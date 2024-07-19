use macroquad::{
    color::{BLACK, WHITE},
    shapes::draw_rectangle,
    window::{clear_background, next_frame},
};

/// Represents the state of the Game of Life.
struct Game {
    height: usize,
    width: usize,
    cells: Vec<bool>,
}

impl Game {
    /// Creates a new Game of Life with random initial state.
    ///
    /// # Arguments
    ///
    /// * `height` - The height of the game grid.
    /// * `width` - The width of the game grid.
    ///
    /// # Returns
    ///
    /// A new `Game` instance.
    fn new(height: usize, width: usize) -> Self {
        let cells = (0..height * width).map(|_| rand::random()).collect();
        Self {
            height,
            width,
            cells,
        }
    }

    /// Gets the index in the cells vector for the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate.
    /// * `y` - The y coordinate.
    ///
    /// # Returns
    ///
    /// The index in the cells vector.
    fn get_index(&self, x: i32, y: i32) -> usize {
        y as usize * self.width + x as usize
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
            if nx < 0 || nx >= self.width as i32 {
                // checks if neighbor's x is out of bounds
                continue;
            }
            for dy in -1..=1 {
                let ny = y + dy;
                if ny < 0 || ny >= self.height as i32 {
                    // checks if neighbor's y is out of bounds
                    continue;
                }
                if nx == x && ny == y {
                    continue;
                }
                let index = self.get_index(nx, ny);
                if self.cells[index] {
                    count += 1;
                }
            }
        }
        count
    }

    /// Updates the game state to the next generation.
    fn update(&mut self) {
        let mut next_cells = vec![false; self.height * self.width];
        for x in 0..self.width as i32 {
            for y in 0..self.height as i32 {
                let index = self.get_index(x, y);
                let cell = self.cells[index];
                let neighbors = self.count_neighbors(x, y);

                next_cells[index] = matches!((cell, neighbors), (true, 2) | (true, 3) | (false, 3));
            }
        }
        self.cells = next_cells
    }

    /// Draws the current game state.
    ///
    /// # Arguments
    ///
    /// * `cell_size` - The size of each cell.
    fn draw(&self, cell_size: f32) {
        for x in 0..self.width {
            for y in 0..self.height {
                let index = y * self.width + x;
                if self.cells[index] {
                    draw_rectangle(
                        x as f32 * cell_size,
                        y as f32 * cell_size,
                        cell_size,
                        cell_size,
                        WHITE,
                    );
                }
            }
        }
    }
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
fn validate_start_parameters(height: usize, width: usize, cell_size: f32) -> Result<(), String> {
    if height == 0 || width == 0 {
        return Err("Neither height nor width can be 0".to_string());
    }
    if cell_size <= 0.0 {
        return Err("Cell size can't be negative or 0".to_string());
    }
    Ok(())
}

/// Starts the Game of Life.
///
/// # Arguments
///
/// * `height` - The height of the game grid.
/// * `width` - The width of the game grid.
/// * `cell_size` - The size of each cell.
///
/// # Returns
///
/// A message if an error ocurrs while validating the parameters, otherwise it loops infinitely.
pub async fn start(height: usize, width: usize, cell_size: f32) -> Result<(), String> {
    validate_start_parameters(height, width, cell_size)?;

    let mut game = Game::new(height, width);
    loop {
        clear_background(BLACK);

        game.update();
        game.draw(cell_size);

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_start_parameters() {
        assert!(validate_start_parameters(60, 80, 10.0).is_ok())
    }

    #[test]
    fn test_invalid_start_parameters() {
        match validate_start_parameters(0, 80, 10.0) {
            Err(e) => assert_eq!(e, "Neither height nor width can be 0"),
            _ => panic!("Expected an error"),
        }
        match validate_start_parameters(60, 0, 10.0) {
            Err(e) => assert_eq!(e, "Neither height nor width can be 0"),
            _ => panic!("Expected an error"),
        }
        match validate_start_parameters(60, 80, 0.0) {
            Err(e) => assert_eq!(e, "Cell size can't be negative or 0"),
            _ => panic!("Expected an error"),
        }
        match validate_start_parameters(60, 80, -1.0) {
            Err(e) => assert_eq!(e, "Cell size can't be negative or 0"),
            _ => panic!("Expected an error"),
        }
    }

    #[test]
    fn test_create_new_game() {
        let game = Game::new(5, 6);
        assert_eq!(game.cells.len(), 30)
    }

    #[test]
    fn test_get_index() {
        let game = Game::new(5, 5);
        assert_eq!(game.get_index(0, 0), 0);
        assert_eq!(game.get_index(4, 0), 4);
        assert_eq!(game.get_index(0, 4), 20);
        assert_eq!(game.get_index(4, 4), 24);
    }

    #[rustfmt::skip]
    #[test]
    fn test_count_neighbors() {
        let mut game = Game::new(5, 5);
        game.cells = vec![
            false, true,  false, true,  false,
            true,  true,  true,  false, true,
            false, false, true,  false, false,
            true,  false, false, true,  true,
            false, true,  false, true,  false,
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
            false, true,  false, true,  false,
            true,  true,  true,  false, true,
            false, false, true,  false, false,
            true,  false, false, true,  true,
            false, true,  false, true,  false,
        ];
        game.update();
        let expected = vec![
            true,  true,  false,  true,  false,
            true,  false, false, false, false,
            true,  false, true, false, true,
            false, true, false, true, true,
            false, false, true, true, true,
        ];
        assert_eq!(game.cells, expected);
    }
}
