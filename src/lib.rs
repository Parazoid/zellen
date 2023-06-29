use crate::grid::Grid;
use macroquad::input;
pub mod cell;
pub mod grid;

fn apply_rules(state: &State, living_neighbors: usize) -> State {
    match state {
        State::Alive => {
            if living_neighbors == 2 || living_neighbors == 3 {
                State::Alive
            } else {
                State::Dead
            }
        }
        State::Dead => {
            if living_neighbors == 2 {
                State::Alive
            } else {
                State::Dead
            }
        }
    }
}

pub fn input_handler(grid: &mut Grid) {
    if input::is_mouse_button_pressed(input::MouseButton::Left) {
        if let Some((i, j)) =
            grid.cell_at_mouse_position(input::mouse_position().0, input::mouse_position().1)
        {
            grid.toggle_cell(i, j);
            println!("TOGGLED at {} {}", i, j);
        }
    }
    if input::is_key_pressed(macroquad::prelude::KeyCode::Space) {
        grid.update();
    }
}

#[derive(Clone, Debug)]
pub enum State {
    Dead,
    Alive,
}
