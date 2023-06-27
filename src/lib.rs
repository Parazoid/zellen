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
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {}
        }
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
    pub fn new() -> Self {
        Cell { state: State::Dead }
    }
}

#[derive(Clone, Debug)]
enum State {
    Dead,
    Alive,
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
}
