use conways_game_of_life::game::Game;

const CELL_SIZE: f32 = 10.0;
const HEIGHT: usize = 60;
const WIDTH: usize = 80;

/// The entry point of the Conway's Game of Life.
///
/// This function initializes the game with the specified height, width,
/// and cell size. If there is an error in the parameters, it prints the error.
///
/// # Constants
///
/// * `CELL_SIZE` - The size of each cell.
/// * `HEIGHT` - The height of the game grid.
/// * `WIDTH` - The width of the game grid.
#[macroquad::main("Conway's Game of Life")]
async fn main() {
    match Game::new(HEIGHT, WIDTH, CELL_SIZE) {
        Ok(mut game) => game.start().await,
        Err(e) => println!("Error: {}", e),
    }
}
