use std::path::PathBuf;
use suwaru::grid::Grid;
use suwaru::plotting::plot_depths;
use suwaru::rastercell::RasterCell;
use suwaru::simulation::Simulation;

fn main() {
    // Parameters
    let n_cols = 400;
    let n_rows = 20;
    let cell_size = 1.0; // in meters
    let starting_depth = 0.5; // in meters
    let timestep = 0.001; // in seconds
    let n_timesteps = 100000;


    // Setting up everything
    let cells = setup_cells(n_cols, n_rows, starting_depth);
    let grid = setup_grid(n_cols, n_rows, cell_size, cells);
    let mut simulation = setup_simulation(grid);

    let img_name = format!("./temp/depths_{:08}.jpg", 0);
    plot_depths(&simulation.grid, &PathBuf::from(img_name));

    println!("\nRunning {} minute simulation for {} cells.",
             timestep*(n_timesteps as f32)/60.0,
             n_rows*n_cols
    );

    // Running the simulation
    for i in 0..n_timesteps {
        if (i+1) % 1000 == 0 {
            println!("Step {} of {}", i+1, n_timesteps);
            println!("Water level of cell ({}, {}): {}m",
                     35,
                     10,
                     simulation.grid.get_cell(350, 10).depth
            );
            let img_name = format!("./temp/depths_{:08}.jpg", i+1);
            plot_depths(&simulation.grid, &PathBuf::from(img_name));

            // TODO: Write result as an image!
        }
        simulation.advance_by_timestep(timestep)
    }
}

fn setup_cells(n_cols: usize, n_rows: usize, starting_depth: f32) -> Vec<RasterCell> {
    let mut cells = Vec::with_capacity(n_cols*n_rows);
    for y in 0..n_rows {
        for x in 0..n_cols {
            let elevation = match x {
                0..=100 => 160.0 - (x as f32),
                100..=200 => 110.0 - (x as f32)/2.0,
                200..=300 => 30.0 - (x as f32)/10.0,
                300..=400 => 0.0,
                _ => 0.0,
            };
            cells.push(
                RasterCell::new_with_xyzh(
                    x,
                    y,
                    elevation,
                    starting_depth
                )
            );
        }
    }
    cells
}

fn setup_grid(n_cols: usize, n_rows: usize, cell_size: f32, cells: Vec<RasterCell>) -> Grid {
    let mut grid = Grid::new(n_cols, n_rows, cell_size);
    grid.cells = cells;
    grid
}

fn setup_simulation(grid: Grid) -> Simulation {
    Simulation::new(grid)
}