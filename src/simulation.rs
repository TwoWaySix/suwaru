use crate::grid::Grid;
use crate::hydraulics::gauckler_manning_stricker as gms;

pub struct Simulation {
    pub grid: Grid,
    pub buffer_grid: Grid,
}

impl Simulation {
    pub fn new(grid: Grid) -> Simulation {
        let cloned_grid = grid.clone();

        Simulation {
            grid,
            buffer_grid: cloned_grid,
        }
    }

    pub fn advance_by_timestep(&mut self, timestep: f32) {
        self.buffer_grid = self.grid.clone();

        for cell in &self.buffer_grid.cells {
            let neighbours = self.buffer_grid.get_cell_neighbours_with_lower_water_level(
                cell.x,
                cell.y
            );

            // Calculate the maximum possible flows, if flow was just to one neighbour cell
            let maximum_flows: Vec<f32> = neighbours.iter()
                .map(|other| gms::flow(&cell, &other, self.buffer_grid.cell_size))
                .collect();

            // Calculate the slopes to each neighbour cell
            let slopes: Vec<f32> = neighbours.iter()
                .map(|other| gms::hydraulic_slope(
                    &cell,
                    &other,
                    self.buffer_grid.cell_size
                ))
                .collect();

            // Normalize the slopes
            let slope_sum: f32 = slopes.iter().sum();
            if slope_sum == 0.0 { continue; } // No slope == No flow
            let slopes_normalized: Vec<f32> = slopes.iter()
                .map(|s| s/slope_sum)
                .collect();

            // Scale the maximum flows by the normalized slopes and calculate the water volumes to be transported
            let mut volumes: Vec<f32> = maximum_flows.iter()
                .zip(&slopes_normalized)
                .map(|(f, s)| f*s*timestep)
                .collect();

            // Make sure, the water volume to be distributed is actually inside the cell. Otherwise, scale the volumes down
            let mut volumes_sum: f32 = volumes.iter().sum();
            let cell_volume = self.buffer_grid.water_volume_of_cell(cell.x, cell.y);
            if volumes_sum > cell_volume {
                let scale_factor = cell_volume/volumes_sum;
                volumes = volumes.iter()
                    .map(|v| v*scale_factor)
                    .collect();
                volumes_sum = cell_volume;
            }

            // Redistribute the water in the original grid
            let area = self.grid.cell_size * self.grid.cell_size;
            let delta_depth = volumes_sum / area;
            self.grid.get_cell_mut(cell.x, cell.y).depth -= delta_depth;
            for (i, neighbour) in neighbours.iter().enumerate() {
                let delta_depth = volumes[i] / area;
                self.grid.get_cell_mut(neighbour.x, neighbour.y).depth += delta_depth;
            }
        }
    }
}