use macroquad::{color::{BLACK, WHITE}, shapes::draw_rectangle, window::{clear_background, next_frame}};

const CELL_SIZE: f32 = 10.0;
const WIDTH: usize = 80;
const HEIGHT: usize = 60;

struct Game {
    cells: Vec<bool>,
    next_cells: Vec<bool>,
}

impl Game {
    fn new() -> Self {
        let cells = (0..WIDTH * HEIGHT).map(|_| rand::random()).collect();
        let next_cells = vec![false; WIDTH * HEIGHT];
        Self { cells, next_cells }
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        ((y as usize) % HEIGHT) * WIDTH + ((x as usize) % WIDTH)
    }

    fn count_neighbors(&self, x: i32, y: i32) -> usize {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx != 0 || dy != 0 {
                    let nx = x + dx;
                    let ny = y + dy;
                    let index = self.get_index(nx, ny);
                    if self.cells[index] {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn update(&mut self) {
        for y in 0..HEIGHT as i32 {
            for x in 0..WIDTH as i32 {
                let index = self.get_index(x, y);
                let cell = self.cells[index];
                let neighbors = self.count_neighbors(x, y);

                self.next_cells[index] = match (cell, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    fn draw(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = y * WIDTH + x;
                if self.cells[index] {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        WHITE,
                    );
                }
            }
        }
    }
}

pub async fn start() {
    let mut game = Game::new();
    loop {
        clear_background(BLACK);

        game.update();
        game.draw();

        next_frame().await
    }
}
