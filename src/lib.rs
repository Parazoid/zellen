use macroquad::input;
use macroquad::shapes::{draw_rectangle, draw_rectangle_lines};
use macroquad::window::{screen_height, screen_width};

#[derive(Debug)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid_row: Vec<Cell> = Vec::new();
        let mut cell_grid: Vec<Vec<Cell>> = Vec::new();
        for _i in 0..width {
            grid_row.push(Cell::new())
        }
        for _j in 0..height {
            cell_grid.push(grid_row.clone())
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
                    State::Dead => draw_rectangle_lines(
                        j as f32 * cell_width,
                        i as f32 * cell_height,
                        cell_width,
                        cell_height,
                        3.0,
                        macroquad::color::BLACK,
                    ),
                    State::Alive => draw_rectangle(
                        j as f32 * cell_width,
                        i as f32 * cell_height,
                        cell_width,
                        cell_height,
                        macroquad::color::BLACK,
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

    fn cell_at_mouse_position(&self, x: f32, y: f32) -> Option<(usize, usize)> {
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

#[derive(Clone, Debug)]
pub struct Cell {
    state: State,
}

impl Cell {
    fn new() -> Self {
        Cell { state: State::Dead }
    }
    fn living_neighbors(&self, cell_x: usize, cell_y: usize, grid: &Grid) -> usize {
        let mut count = 0;
        for colMod in (-1 as isize)..=1 {
            for rowMod in (-1 as isize)..=1 {
                let neighbor_x = cell_x as isize + colMod;
                let neighbor_y = cell_y as isize + rowMod;
                if neighbor_x < 0
                    || neighbor_y < 0
                    || neighbor_x >= grid.width as isize
                    || neighbor_y >= grid.height as isize
                    || (colMod == 0 && rowMod == 0)
                {
                    continue;
                } else {
                    match grid.cells[neighbor_y as usize][neighbor_x as usize].state {
                        State::Alive => count += 1,
                        State::Dead => (),
                    }
                }
            }
        }
        count
    }
}

#[derive(Clone, Debug)]
enum State {
    Dead,
    Alive,
}

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
            if living_neighbors == 3 {
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
