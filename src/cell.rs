use crate::{grid::Grid, State};

#[derive(Clone, Debug)]
pub struct Cell {
    pub state: State,
}

impl Cell {
    
    pub fn living_neighbors(&self, cell_x: usize, cell_y: usize, grid: &Grid) -> usize {
        let mut count = 0;
        for col_mod in -1..=1 {
            for row_mod in -1..=1 {
                let neighbor_x = cell_x as isize + col_mod;
                let neighbor_y = cell_y as isize + row_mod;
                if neighbor_x < 0
                    || neighbor_y < 0
                    || neighbor_x >= grid.width as isize
                    || neighbor_y >= grid.height as isize
                    || (col_mod == 0 && row_mod == 0)
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
