use crate::board_renderer::BoardRenderer;
use macroquad::prelude::*;

const DEFAULT_SPEED: f32 = 10.0;

/// Represents the state of the Game of Life.
pub struct Game {
    board: BoardRenderer,
    cells: Vec<Vec<bool>>,
}

impl Game {
    /// Creates a new Game of Life with all cells initially dead.
    ///
    /// # Arguments
    ///
    /// * `nrows` - The number of rows in the game grid.
    /// * `ncols` - The number of columns in the game grid.
    /// * `cell_size` - The size of each cell in pixels.
    ///
    /// # Returns
    ///
    /// A new `Game` instance with the specified dimensions and cell size.
    pub fn new(nrows: usize, ncols: usize, cell_size: usize) -> Self {
        let board = BoardRenderer {
            ncols,
            nrows,
            cell_size,
        };
        let cells = vec![vec![false; nrows]; ncols];
        Self { board, cells }
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
        BoardRenderer::show_start_menu().await;
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
                BoardRenderer::draw_menu(&mut paused, &mut restart).await;
            }
            if restart {
                self.restart().await;
                paused = false;
                show_menu = false;
                restart = false;
            }

            clear_background(BLACK);
            self.board.draw_grid();
            self.board.draw_cells(&self.cells);
            next_frame().await;
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
            self.board.draw_grid();
            self.board.draw_cells(&self.cells);

            if is_mouse_button_pressed(MouseButton::Left) {
                self.toggle_cell_state();
            }
            if is_key_pressed(KeyCode::R) {
                self.randomize();
            }

            BoardRenderer::show_initial_instructions();
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
        let x = (mouse_pos.0 / self.board.cell_size as f32) as usize;
        let y = (mouse_pos.1 / self.board.cell_size as f32) as usize;

        if x < self.board.ncols && y < self.board.nrows {
            self.cells[x][y] = !self.cells[x][y];
        }
    }

    /// Randomizes the state of all cells in the grid.
    ///
    /// This function sets each cell in the grid to a random state (alive or dead).
    fn randomize(&mut self) {
        self.cells = (0..self.board.ncols)
            .map(|_| (0..self.board.nrows).map(|_| ::rand::random()).collect())
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
        let mut next_cells = vec![vec![false; self.board.nrows]; self.board.ncols];

        #[allow(clippy::needless_range_loop)] // this way is clearer than how Clippy suggests
        for x in 0..self.board.ncols {
            for y in 0..self.board.nrows {
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
            if nx < 0 || nx >= self.board.ncols as i32 {
                // checks if neighbor's x is out of bounds
                continue;
            }
            for dy in -1..=1 {
                let ny = y + dy;
                if ny < 0 || ny >= self.board.nrows as i32 {
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

    /// Resets the game state and restarts the game.
    ///
    /// This function clears the current state of the cells, effectively resetting the game board
    /// to its initial empty state. It then prompts the user to choose a new initial state for the cells.
    async fn restart(&mut self) {
        self.cells = vec![vec![false; self.board.nrows]; self.board.ncols];
        self.choose_initial_state().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_game() {
        let game = Game::new(30, 40, 10);
        assert_eq!(game.cells.len(), 40);
        assert_eq!(game.cells[0].len(), 30);
    }

    #[rustfmt::skip]
    #[test]
    fn test_count_neighbors() {
        let mut game = Game::new(5, 5, 1);
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
        let mut game = Game::new(5, 5, 1);
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
