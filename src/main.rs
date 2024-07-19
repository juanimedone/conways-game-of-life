use conways_game_of_life::game;

const CELL_SIZE: f32 = 10.0;
const HEIGHT: usize = 60;
const WIDTH: usize = 80;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    game::start(HEIGHT, WIDTH, CELL_SIZE).await
}
