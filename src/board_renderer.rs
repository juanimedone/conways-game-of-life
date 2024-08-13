use crate::game::Game;
use macroquad::{prelude::*, ui::root_ui};

const DEFAULT_SPEED: f32 = 10.0;

/// Responsible for rendering the game board and handling UI elements.
pub struct BoardRenderer {
    pub ncols: usize,
    pub nrows: usize,
    pub cell_size: usize,
}

impl BoardRenderer {
    /// Displays the start menu for the game.
    ///
    /// This function renders the start menu on the screen with instructions for opening and closing the menu,
    /// pausing the game and changing the game speed. It continuously displays
    /// the menu until the player presses the Enter key to start the game.
    async fn show_start_menu() {
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

    /// Displays initial instructions for the game.
    ///
    /// This function renders initial instructions for selecting alive cells and
    /// randomizing the board.
    pub fn show_initial_instructions() {
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
    }

    /// Draws the grid lines for the Game of Life.
    ///
    /// This function renders the grid lines onto the screen to visually separate the cells.
    /// The lines are drawn based on the cell size and the dimensions of the game grid.
    pub fn draw_grid(&self) {
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
    ///
    /// # Arguments
    ///
    /// * `cells` - A reference to a 2D vector representing the game grid. Each element
    ///   is a boolean indicating whether the cell is alive (`true`) or dead (`false`).
    pub fn draw_cells(&self, cells: &[Vec<bool>]) {
        #[allow(clippy::needless_range_loop)] // this way is clearer than how Clippy suggests
        for x in 0..self.ncols {
            for y in 0..self.nrows {
                if cells[x][y] {
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
    pub async fn draw_menu(paused: &mut bool, restart: &mut bool) {
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

    /// Allows the player to choose the initial alive cells through a GUI.
    ///
    /// This function allows the player to click on cells to toggle their state
    /// (alive or dead) before starting the game. The player can also randomize
    /// the initial state by pressing 'R'.
    async fn choose_initial_state(&mut self, game: &mut Game) {
        let mut choosing = true;
        
        while choosing {
            clear_background(BLACK);
            self.draw_grid();
            self.draw_cells(&game.cells);
            
            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_pos = mouse_position();
                let x = (mouse_pos.0 / self.cell_size as f32) as usize;
                let y = (mouse_pos.1 / self.cell_size as f32) as usize;
                game.toggle_cell_state(x, y);
            }
            if is_key_pressed(KeyCode::R) {
                game.randomize();
            }
            
            Self::show_initial_instructions();
            next_frame().await;
            
            if is_key_pressed(KeyCode::Enter) {
                choosing = false;
            }
        }
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
    
    /// Resets the game state and restarts the game.
    ///
    /// This function clears the current state of the cells, effectively resetting the game board
    /// to its initial empty state. It then prompts the user to choose a new initial state for the cells.
    pub async fn restart(&mut self, game: &mut Game) {
        *game = Game::new(self.ncols, self.nrows);
        self.choose_initial_state(game).await;
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
    pub async fn run(&mut self, game: &mut Game) {
        Self::show_start_menu().await;
        self.choose_initial_state(game).await;

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
                    game.update();
                    update_timer = 0.0;
                }
            }
            if show_menu {
                Self::draw_menu(&mut paused, &mut restart).await;
            }
            if restart {
                self.restart(game).await;
                paused = false;
                show_menu = false;
                restart = false;
            }

            clear_background(BLACK);
            self.draw_grid();
            self.draw_cells(&game.cells);
            next_frame().await;
        }
    }
}
