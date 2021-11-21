use crate::grid::Grid;

pub struct Simulation {
    pub grid: Grid,
    pub buffer: Grid,
}

impl Simulation {
    pub fn new(grid: Grid, timestep: f32) -> Simulation {
        let cloned_grid = grid.clone();

        Simulation {
            grid,
            buffer: cloned_grid,
        }
    }

    pub fn advance_by_timestep(&mut self, timestep: f32) {
        for cell in &self.grid.cells {

        }
    }
}