use macroquad::prelude::*;
use zellen::{input_handler, Grid};

#[macroquad::main("ZELLEN: Cellular Automata")]
async fn main() {
    let mut grid = Grid::new(20, 20);
    println!("{:?}", grid);
    loop {
        clear_background(GRAY);
        grid.show();
        input_handler(&mut grid);
        next_frame().await
    }
}
