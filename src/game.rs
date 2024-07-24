use macroquad::{prelude::*, ui::root_ui};

const DEFAULT_SPEED: f32 = 10.0;

/// Represents the state of the Game of Life.
pub struct Game {
    ncols: usize,
    nrows: usize,
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
    /// * `height` - The height of the game grid in pixels.
    /// * `width` - The width of the game grid in pixels.
    /// * `cell_size` - The size of each cell in pixels.
    ///
    /// # Returns
    ///
    /// `Ok(Self)` if the parameters are valid, containing the new `Game` instance,
    /// or `Err(String)` with an error message if any of the parameters is invalid.
    pub fn new(height: usize, width: usize, cell_size: usize) -> Result<Self, String> {
        Self::validate_start_parameters(height, width, cell_size)?;

        let ncols = width / cell_size;
        let nrows = height / cell_size;
        let cells = vec![vec![false; nrows]; ncols];
        Ok(Self {
            ncols,
            nrows,
            cell_size,
            cells,
        })
    }

    /// Validates the start parameters for the game.
    ///
    /// # Arguments
    ///
    /// * `height` - The height of the game grid in pixels.
    /// * `width` - The width of the game grid in pixels.
    /// * `cell_size` - The size of each cell in pixels.
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
    /// This function manages the game's lifecycle, including displaying the start menu,
    /// allowing the player to choose the initial state of the cells, and continuously updating
    /// and rendering the game state. The game can be paused or unpaused by pressing the Space
    /// key, and the game speed can be adjusted using the Up and Down arrow keys.
    ///
    /// The `run` function performs the following actions in its main loop:
    /// - Checks for key presses to pause/unpause the game, adjust the game speed or show the menu.
    /// - Updates the game state if the game is not paused.
    /// - Draws the game grid and cells.
    /// - Displays the menu when requested and handles restarting the game if necessary.
    /// - Waits for the next frame to be drawn, allowing for smooth animation.
    ///
    /// The game speed controls the frequency of state updates. The `speed` variable is
    /// adjusted by multiplying or dividing it by 1.5 when the Up or Down arrow keys are pressed,
    /// respectively. The update timer ensures the game state updates according to the current speed.
    pub async fn run(&mut self) {
        self.show_start_menu().await;
        self.choose_initial_state().await;

        let mut paused = false;
        let mut show_menu = false;
        let mut restart = false;
        let mut speed = DEFAULT_SPEED;
        let mut update_timer = 0.0;
        loop {
            self.check_keys(&mut paused, &mut speed, &mut show_menu);

            if !paused {
                update_timer += get_frame_time();
                if update_timer >= 1.0 / speed {
                    self.update();
                    update_timer = 0.0;
                }
            }
            if show_menu {
                self.draw_menu(&mut paused, &mut restart).await;
            }
            if restart {
                self.restart().await;
                paused = false;
                show_menu = false;
                restart = false;
            }

            clear_background(BLACK);
            self.draw_grid();
            self.draw_cells();
            next_frame().await;
        }
    }

    /// Displays the start menu for the game.
    ///
    /// This function renders the start menu on the screen with instructions for opening and closing the menu,
    /// pausing the game and changing the game speed. It continuously displays
    /// the menu until the player presses the Enter key to start the game.
    async fn show_start_menu(&self) {
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
                "Press M to open and close the menu",
                screen_width() / 2.0 - 140.0,
                screen_height() / 2.0 + 20.0,
                20.0,
                GRAY,
            );
            draw_text(
                "Press Space to pause",
                screen_width() / 2.0 - 100.0,
                screen_height() / 2.0 + 50.0,
                20.0,
                GRAY,
            );
            draw_text(
                "Press Up/Down arrows to change speed",
                screen_width() / 2.0 - 160.0,
                screen_height() / 2.0 + 80.0,
                20.0,
                GRAY,
            );
            draw_text(
                "Press Enter to start",
                screen_width() / 2.0 - 100.0,
                screen_height() / 2.0 + 140.0,
                20.0,
                WHITE,
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

    /// Toggles the state of the cell at the mouse position.
    ///
    /// This function retrieves the mouse position and toggles the state of the cell
    /// at that position from alive to dead or vice versa.
    fn toggle_cell_state(&mut self) {
        let mouse_pos = mouse_position();
        let x = (mouse_pos.0 / self.cell_size as f32) as usize;
        let y = (mouse_pos.1 / self.cell_size as f32) as usize;

        if x < self.ncols && y < self.nrows {
            self.cells[x][y] = !self.cells[x][y];
        }
    }

    /// Randomizes the state of all cells in the grid.
    ///
    /// This function sets each cell in the grid to a random state (alive or dead).
    fn randomize(&mut self) {
        self.cells = (0..self.ncols)
            .map(|_| (0..self.nrows).map(|_| ::rand::random()).collect())
            .collect();
    }

    /// Checks for key presses to pause/unpause the game, adjust the speed and show the menu.
    ///
    /// # Arguments
    ///
    /// * `paused` - A mutable reference to a boolean that indicates whether the game is paused.
    /// * `speed` - A mutable reference to a float representing the current game speed.
    /// * `show_menu` - A mutable reference to a boolean that controls the visibility of the menu.
    fn check_keys(&mut self, paused: &mut bool, speed: &mut f32, show_menu: &mut bool) {
        if is_key_pressed(KeyCode::Space) {
            *paused = !*paused;
        }
        if is_key_pressed(KeyCode::Up) {
            *speed = (*speed * 1.5).min(1000.0);
        }
        if is_key_pressed(KeyCode::Down) {
            *speed = (*speed / 1.5).max(0.1);
        }
        if is_key_pressed(KeyCode::M) {
            *show_menu = !*show_menu;
        }
    }

    /// Updates the game state to the next generation.
    ///
    /// This function calculates the next state of the game based on the current state
    /// and updates the cells accordingly.
    fn update(&mut self) {
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

    /// Draws the grid lines for the Game of Life.
    ///
    /// This function renders the grid lines onto the screen to visually separate the cells.
    /// The lines are drawn based on the cell size and the dimensions of the game grid.
    fn draw_grid(&self) {
        let width = self.ncols * self.cell_size;
        let height = self.nrows * self.cell_size;

        for x in 0..=self.ncols {
            let x_pos = (x * self.cell_size) as f32;
            draw_line(x_pos, 0.0, x_pos, height as f32, 1.0, GRAY);
        }

        for y in 0..=self.nrows {
            let y_pos = (y * self.cell_size) as f32;
            draw_line(0.0, y_pos, width as f32, y_pos, 1.0, GRAY);
        }
    }

    /// Draws the current game state cells.
    ///
    /// This function renders the cells of the game onto the screen using the cell size
    /// to determine their position and dimensions. Only alive cells are drawn.
    fn draw_cells(&self) {
        for x in 0..self.ncols {
            for y in 0..self.nrows {
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

    /// Draws the game menu with options for restarting and pausing/unpausing the game.
    ///
    /// # Arguments
    ///
    /// * `paused` - A mutable reference to a boolean that indicates whether the game is currently paused.
    /// * `restart` - A mutable reference to a boolean that is set to `true` when the "Restart Game" button is pressed.
    async fn draw_menu(&mut self, paused: &mut bool, restart: &mut bool) {
        let menu_height = 200.0;
        let menu_width = 250.0;
        let menu_x = (screen_width() - menu_width) / 2.0;
        let menu_y = (screen_height() - menu_height) / 2.0;

        root_ui().window(
            1,
            vec2(menu_x, menu_y),
            vec2(menu_width, menu_height),
            |ui| {
                ui.label(None, "Game Menu");
                ui.separator();
                if ui.button(None, "Restart Game") {
                    *restart = true;
                }
                if ui.button(None, if *paused { "Unpause" } else { "Pause" }) {
                    *paused = !*paused;
                }
                ui.label(None, "Press 'M' to close the menu");
            },
        );
    }

    /// Resets the game state and restarts the game.
    ///
    /// This function clears the current state of the cells, effectively resetting the game board
    /// to its initial empty state. It then prompts the user to choose a new initial state for the cells.
    async fn restart(&mut self) {
        self.cells = vec![vec![false; self.nrows]; self.ncols];
        self.choose_initial_state().await;
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
