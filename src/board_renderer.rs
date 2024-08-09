use macroquad::{prelude::*, ui::root_ui};

/// Responsible for rendering the game board and handling UI elements.
pub struct BoardRenderer {
    pub(crate) ncols: usize,
    pub(crate) nrows: usize,
    pub(crate) cell_size: usize,
}

impl BoardRenderer {
    /// Displays the start menu for the game.
    ///
    /// This function renders the start menu on the screen with instructions for opening and closing the menu,
    /// pausing the game and changing the game speed. It continuously displays
    /// the menu until the player presses the Enter key to start the game.
    pub async fn show_start_menu() {
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
}
