use conways_game_of_life::game;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    game::start().await
}
