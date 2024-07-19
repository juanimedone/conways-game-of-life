use macroquad::{color::{BLACK, WHITE}, shapes::draw_rectangle, window::{clear_background, next_frame}};

struct Game {
    height: usize,
    width: usize,
    cells: Vec<bool>,
    next_cells: Vec<bool>,
}

impl Game {
    fn new(height: usize, width: usize) -> Self {
        let cells = (0..height * width).map(|_| rand::random()).collect();
        let next_cells = vec![false; height * width];
        Self { height, width, cells, next_cells }
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        ((y as usize) % self.height) * self.width + ((x as usize) % self.width)
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
        for x in 0..self.width as i32 {
            for y in 0..self.height as i32 {
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

    fn draw(&self, cell_size: f32) {
        for x in 0..self.width {
            for y in 0..self.height {
                let index = y * self.width + x;
                if self.cells[index] {
                    draw_rectangle(
                        x as f32 * cell_size,
                        y as f32 * cell_size,
                        cell_size,
                        cell_size,
                        WHITE,
                    );
                }
            }
        }
    }
}

pub async fn start(height: usize, width: usize, cell_size: f32) {
    let mut game = Game::new(height, width);
    loop {
        clear_background(BLACK);

        game.update();
        game.draw(cell_size);

        next_frame().await
    }
}
