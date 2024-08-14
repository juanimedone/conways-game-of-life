use conways_game_of_life::{board_renderer::BoardRenderer, game::Game};
use macroquad::window::*;
use std::num::NonZeroUsize;

const CELL_SIZE: usize = 20;
const NROWS: usize = 30;
const NCOLS: usize = 40;

/// Window configuration for Macroquad.
///
/// This function returns a `Conf` struct with the desired window settings for the game.
///
/// # Returns
///
/// * `Conf` - The configuration struct with the specified title, height and width.
fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_string(),
        window_height: (NROWS * CELL_SIZE) as i32,
        window_width: (NCOLS * CELL_SIZE) as i32,
        ..Default::default()
    }
}

/// The entry point of the Conway's Game of Life.
///
/// This function initializes the game with the specified cell size and dimensions.
/// It uses `NonZeroUsize` to ensure that the dimensions and cell size are non-zero.
/// If any of the parameters are invalid, it prints an error message.
///
/// # Constants
///
/// * `CELL_SIZE` - The size of each cell in pixels.
/// * `NROWS` - The number of rows in the game grid.
/// * `NCOLS` - The number of columns in the game grid.
///
/// # Usage
///
/// This function uses the `#[macroquad::main(window_conf)]` attribute to set the window configuration.
/// It then creates a new game instance with a board and runs it asynchronously.
#[macroquad::main(window_conf)]
async fn main() {
    match (
        NonZeroUsize::new(NROWS),
        NonZeroUsize::new(NCOLS),
        NonZeroUsize::new(CELL_SIZE),
    ) {
        (Some(nrows), Some(ncols), Some(cell_size)) => {
            let mut board = BoardRenderer {
                ncols: ncols.get(),
                nrows: nrows.get(),
                cell_size: cell_size.get(),
            };
            let mut game = Game::new(ncols.get(), nrows.get());
            board.run(&mut game).await;
        }
        _ => {
            println!("Error: Invalid dimensions or cell size");
        }
    }
}
