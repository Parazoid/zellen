use macroquad::prelude::*;
use zellen::Grid;

#[macroquad::main("ZELLEN: Cellular Automata")]
async fn main() {
    let grid = Grid::new(10, 10);
    println!("{:?}", grid);
    loop {
        clear_background(GRAY);
        grid.show(screen_width(), screen_height());
        next_frame().await
    }
}
