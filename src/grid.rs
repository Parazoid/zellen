use crate::apply_rules;
use crate::{cell::Cell, State};
use macroquad::shapes::draw_rectangle;
use macroquad::window::{screen_height, screen_width};

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid_row: Vec<Cell> = Vec::new();
        let mut cell_grid: Vec<Vec<Cell>> = Vec::new();
        for _i in 0..width {
            grid_row.push(Cell { state: State::Dead });
        }
        for _j in 0..height {
            cell_grid.push(grid_row.clone());
        }
        Grid {
            cells: cell_grid,
            width,
            height,
        }
    }

    pub fn show(&self) {
        let cell_width = screen_width() / self.width as f32;
        let cell_height = screen_height() / self.height as f32;

        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match cell.state {
                    State::Dead => draw_rectangle(
                        j as f32 * cell_width,
                        i as f32 * cell_height,
                        cell_width,
                        cell_height,
                        macroquad::color::BLACK,
                    ),
                    State::Alive => draw_rectangle(
                        j as f32 * cell_width,
                        i as f32 * cell_height,
                        cell_width,
                        cell_height,
                        macroquad::color::WHITE,
                    ),
                };
            }
        }
    }

    pub fn update(&mut self) {
        let current_grid_state = &self.cells;
        let mut new_grid_state = current_grid_state.clone();
        for (i, row) in current_grid_state.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let living_neighbors = cell.living_neighbors(j, i, self);
                new_grid_state[i][j].state = apply_rules(&cell.state, living_neighbors);
            }
        }
        self.cells = new_grid_state;
    }

    pub fn cell_at_mouse_position(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        let cell_width = screen_width() / self.width as f32;
        let cell_height = screen_height() / self.height as f32;
        if x < 0.0 || x > screen_width() || y < 0.0 || y > screen_height() {
            None
        } else {
            Some(((y / cell_height) as usize, (x / cell_width) as usize))
        }
    }

    pub fn toggle_cell(&mut self, row: usize, column: usize) {
        self.cells[row][column].state = match self.cells[row][column].state {
            State::Alive => State::Dead,
            State::Dead => State::Alive,
        }
    }
}
