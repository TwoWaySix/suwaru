use crate::grid::Grid;

pub struct Simulation {
    pub grid: Grid,
    pub buffer: Grid,
    pub timestep: f32,
}

impl Simulation {
    pub fn new(grid: Grid, timestep: f32) -> Simulation {
        let cloned_grid = grid.clone();

        Simulation {
            grid,
            buffer: cloned_grid,
            timestep,
        }
    }

    pub fn advance_by_timestep(&mut self) {

    }
}