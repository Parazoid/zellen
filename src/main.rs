use std::{thread::sleep, time::Duration};

use macroquad::prelude::*;
use zellen::{grid::Grid, input_handler};

#[macroquad::main("ZELLEN: Cellular Automata")]
async fn main() {
    let mut grid = Grid::new(50, 50);
    request_new_screen_size(500.0, 500.0);
    let mut manual_mode = true;
    loop {
        clear_background(WHITE);
        grid.show();
        manual_mode = input_handler(&mut grid, manual_mode);
        if !manual_mode {
            grid.update();
            sleep(Duration::from_millis(200));
        }
        next_frame().await
    }
}
