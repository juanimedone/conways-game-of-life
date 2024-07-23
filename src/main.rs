use conways_game_of_life::game::Game;

const CELL_SIZE: usize = 20;
const HEIGHT: usize = 600;
const WIDTH: usize = 800;

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
        Ok(mut game) => game.run().await,
        Err(e) => println!("Error: {}", e),
    }
}
