use macroquad::prelude::*;

const DEFAULT_SPEED: f32 = 10.0;

/// Represents the state of the Game of Life.
pub struct Game {
    cell_size: usize,
    cells: Vec<Vec<bool>>,
}

impl Game {
    /// Creates a new Game of Life with all cells initially dead.
    ///
    /// The function will adjust the number of rows and columns based on the given
    /// `cell_size` and will initialize all cells to be dead (false).
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
        let cells = vec![vec![false; nrows]; ncols];
        Ok(Self { cell_size, cells })
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

    /// Runs the Game of Life.
    ///
    /// This function displays the start menu, allows the player to choose the initial
    /// state of the cells, and then enters an infinite loop to continuously update
    /// and draw the game state. The game can be paused and unpaused by pressing the
    /// Space key. The game speed can be increased by pressing the Up arrow key and
    /// decreased by pressing the Down arrow key.
    ///
    /// The main loop performs the following actions:
    /// - Checks for key presses to pause/unpause the game and adjust the game speed.
    /// - Updates the game state if the game is not paused.
    /// - Draws the game grid and cells.
    /// - Waits for the next frame to be drawn.
    ///
    /// The game speed controls how frequently the game state updates. The `speed` variable is
    /// multiplied or divided by 1.5 when the Up or Down arrow keys are pressed, respectively.
    pub async fn run(&mut self) {
        self.show_menu().await;
        self.choose_initial_state().await;

        let mut paused = false;
        let mut speed = DEFAULT_SPEED;
        let mut update_timer = 0.0;
        loop {
            Self::check_keys(&mut paused, &mut speed);

            if !paused {
                update_timer += get_frame_time();
                if update_timer >= 1.0 / speed {
                    self.update();
                    update_timer = 0.0;
                }
            }

            clear_background(BLACK);
            self.draw_grid();
            self.draw_cells();
            next_frame().await;
        }
    }

    /// Displays the start menu.
    ///
    /// This function shows the start menu with instructions to start the game,
    /// pause the game, and change the game speed.
    async fn show_menu(&self) {
        let mut show_menu = true;

        while show_menu {
            clear_background(BLACK);
            draw_text(
                "Conway's Game of Life",
                screen_width() / 2.0 - 150.0,
                screen_height() / 2.0 - 20.0,
                30.0,
                WHITE,
            );
            draw_text(
                "Press Enter to Start",
                screen_width() / 2.0 - 100.0,
                screen_height() / 2.0 + 20.0,
                20.0,
                GRAY,
            );
            draw_text(
                "Press Space to Pause",
                screen_width() / 2.0 - 100.0,
                screen_height() / 2.0 + 50.0,
                20.0,
                GRAY,
            );
            draw_text(
                "Press Up/Down arrows to Change Speed",
                screen_width() / 2.0 - 160.0,
                screen_height() / 2.0 + 80.0,
                20.0,
                GRAY,
            );
            next_frame().await;

            if is_key_pressed(KeyCode::Enter) {
                show_menu = false;
            }
        }
    }

    /// Allows the player to choose the initial alive cells through a GUI.
    ///
    /// This function allows the player to click on cells to toggle their state
    /// (alive or dead) before starting the game. The player can also randomize
    /// the initial state by pressing 'R'.
    async fn choose_initial_state(&mut self) {
        let mut choosing = true;

        while choosing {
            clear_background(BLACK);
            self.draw_grid();
            self.draw_cells();

            if is_mouse_button_pressed(MouseButton::Left) {
                self.toggle_cell_state();
            }
            if is_key_pressed(KeyCode::R) {
                self.randomize();
            }

            draw_text(
                "Select alive cells and press Enter to Start",
                screen_width() / 2.0 - 180.0,
                screen_height() - 40.0,
                20.0,
                WHITE,
            );
            draw_text(
                "Press 'R' to randomize",
                screen_width() / 2.0 - 100.0,
                screen_height() - 20.0,
                20.0,
                WHITE,
            );
            next_frame().await;

            if is_key_pressed(KeyCode::Enter) {
                choosing = false;
            }
        }
    }

    fn toggle_cell_state(&mut self) {
        let mouse_pos = mouse_position();
        let x = (mouse_pos.0 / self.cell_size as f32) as usize;
        let y = (mouse_pos.1 / self.cell_size as f32) as usize;

        if x < self.cells.len() && y < self.cells[0].len() {
            self.cells[x][y] = !self.cells[x][y];
        }
    }

    fn randomize(&mut self) {
        let nrows = self.cells.len();
        let ncols = self.cells[0].len();
        self.cells = (0..nrows)
            .map(|_| (0..ncols).map(|_| ::rand::random()).collect())
            .collect();
    }

    fn check_keys(paused: &mut bool, speed: &mut f32) {
        if is_key_pressed(KeyCode::Space) {
            *paused = !*paused;
        }
        if is_key_pressed(KeyCode::Up) {
            *speed *= 1.5;
        }
        if is_key_pressed(KeyCode::Down) {
            *speed /= 1.5;
        }
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

    /// Draws the grid lines for the Game of Life.
    ///
    /// This function renders the grid lines onto the screen to visually separate the cells.
    /// The lines are drawn based on the cell size and the dimensions of the game grid.
    fn draw_grid(&self) {
        let width = self.cells.len() * self.cell_size;
        let height = self.cells[0].len() * self.cell_size;

        for x in 0..=self.cells.len() {
            let x_pos = (x * self.cell_size) as f32;
            draw_line(x_pos, 0.0, x_pos, height as f32, 1.0, GRAY);
        }

        for y in 0..=self.cells[0].len() {
            let y_pos = (y * self.cell_size) as f32;
            draw_line(0.0, y_pos, width as f32, y_pos, 1.0, GRAY);
        }
    }

    /// Draws the current game state cells.
    ///
    /// This function renders the cells of the game onto the screen using the cell size
    /// to determine their position and dimensions. Only alive cells are drawn.
    fn draw_cells(&self) {
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
