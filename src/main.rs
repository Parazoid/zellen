use macroquad::prelude::*;
use zellen::{grid::Grid, input_handler};

#[macroquad::main("ZELLEN: Cellular Automata")]
async fn main() {
    let mut grid = Grid::new(50, 50);
    request_new_screen_size(500.0, 500.0);
    loop {
        clear_background(WHITE);
        grid.show();
        input_handler(&mut grid);
        next_frame().await
    }
}
