use conways_game_of_life::game::Game;
use macroquad::window::*;
use std::num::NonZeroUsize;

const CELL_SIZE: usize = 20;
const WINDOW_HEIGHT: i32 = 600;
const WINDOW_WIDTH: i32 = 800;

/// Window configuration for Macroquad.
///
/// This function returns a `Conf` struct with the desired window settings for the game.
///
/// # Returns
///
/// * `Conf` - The configuration struct with the specified title, width, and height.
fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_string(),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        ..Default::default()
    }
}

/// The entry point of the Conway's Game of Life.
///
/// This function initializes the game with the specified cell size and the window dimensions.
/// It uses `NonZeroUsize` to ensure that the dimensions and cell size are non-zero.
/// If any of the parameters are invalid, it prints an error message.
///
/// # Constants
///
/// * `CELL_SIZE` - The size of each cell in pixels.
/// * `WINDOW_HEIGHT` - The height of the game window in pixels.
/// * `WINDOW_WIDTH` - The width of the game window in pixels.
///
/// # Usage
///
/// This function uses the `#[macroquad::main(window_conf)]` attribute to set the window configuration.
/// It then creates a new game instance and runs it asynchronously.
#[macroquad::main(window_conf)]
async fn main() {
    match (
        NonZeroUsize::new(screen_height() as usize),
        NonZeroUsize::new(screen_width() as usize),
        NonZeroUsize::new(CELL_SIZE),
    ) {
        (Some(height), Some(width), Some(cell_size)) => {
            let mut game = Game::new(height.get(), width.get(), cell_size.get());
            game.run().await;
        }
        _ => {
            println!("Error: Invalid dimensions or cell size");
        }
    }
}
