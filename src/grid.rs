use crate::node::Node;

#[derive(Debug)]
pub struct Grid {
    pub n_cols: usize,
    pub n_rows: usize,
    pub cell_size: f32,
    pub cells: Vec<Node>,
}

impl Grid {
    pub fn new(n_cols: usize, n_rows: usize, cell_size: f32) -> Grid {
        Grid {
            n_cols,
            n_rows,
            cell_size,
            cells: Vec::with_capacity(n_cols*n_rows),
        }
    }

    pub fn clone(&self) -> Grid {
        let mut copied_cells = Vec::with_capacity(self.n_rows*self.n_cols);

        for node in &self.cells {
            copied_cells.push(node.clone())
        }

        Grid {
            n_cols: self.n_cols,
            n_rows: self.n_rows,
            cell_size: self.cell_size,
            cells: copied_cells
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Node {
        &self.cells[y*self.n_cols + x]
    }

    pub fn get_cell_neighbours(&self, x: usize, y: usize) -> Vec<&Node> {
        let mut cell_neighbours = Vec::new();

        let x_from = if x != 0 { x-1 } else { 0 };
        let y_from = if y != 0 { y-1 } else { 0 };

        let x_to =  if x != self.n_cols-1 { x+1 } else { self.n_cols-1 };
        let y_to =  if y != self.n_rows-1 { y+1 } else { self.n_rows-1 };

        for row_i in y_from..=y_to {
            for col_i in x_from..=x_to {
                if col_i == x && row_i == y { continue; }

                cell_neighbours.push(self.get_cell(col_i, row_i));
            }
        }
        cell_neighbours
    }

    pub fn get_water_volume(&self, x: usize, y: usize) -> f32 {
        let cell = self.get_cell(x, y);
        let cell_neighbours = self.get_cell_neighbours(x, y);

        // TODO: Implement water volume calculation
        0.0
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        for (n1, n2) in self.cells.iter().zip(&other.cells) {
            if n1.x != n2.x || n1.y != n2.y || n1.elevation != n2.elevation || n1.depth != n2.depth {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod grid_tests {
    use crate::grid::Grid;
    use crate::node::Node;

    fn create_test_grid_no1() -> Grid {
        let mut grid = Grid::new(4,3, 1.0);
        for i in 0..12 {
            let node = Node::new_with_xyz(0, 0, i as f32);
            grid.cells.push(node);
        }
        grid
    }

    #[test]
    fn test_get_cell() {
        let grid = create_test_grid_no1();

        assert_eq!(
            grid.get_cell(0, 0).elevation,
            0.0
        );
        assert_eq!(
            grid.get_cell(1, 0).elevation,
            1.0
        );
        assert_eq!(
            grid.get_cell(1, 2).elevation,
            9.0
        );
        assert_eq!(
            grid.get_cell(3, 2).elevation,
            11.0
        );
    }

    #[test]
    fn test_clone() {
        let grid = create_test_grid_no1();
        let mut cloned = grid.clone();

        assert_eq!(grid, cloned);

        cloned.cells[0].y = 31470;
        assert_ne!(grid, cloned);
    }

    #[test]
    fn test_get_cell_neighbours_order() {
        let grid = create_test_grid_no1();
        let cell_neighbours = grid.get_cell_neighbours(1, 1);

        assert_eq!(cell_neighbours[0].elevation, 0.0);
        assert_eq!(cell_neighbours[1].elevation, 1.0);
        assert_eq!(cell_neighbours[2].elevation, 2.0);

        assert_eq!(cell_neighbours[3].elevation, 4.0);
        assert_eq!(cell_neighbours[4].elevation, 6.0);

        assert_eq!(cell_neighbours[5].elevation, 8.0);
        assert_eq!(cell_neighbours[6].elevation, 9.0);
        assert_eq!(cell_neighbours[7].elevation, 10.0);
    }

    #[test]
    fn test_get_cell_neighbours_1_1() {
        let grid = create_test_grid_no1();
        let cell_neighbours = grid.get_cell_neighbours(1, 1);

        assert_eq!(cell_neighbours.len(), 8);
    }

    #[test]
    fn test_get_cell_neighbours_0_0() {
        let grid = create_test_grid_no1();
        let cell_neighbours = grid.get_cell_neighbours(0, 0);

        assert_eq!(cell_neighbours.len(), 3);
    }

    #[test]
    fn test_get_cell_neighbours_0_1() {
        let grid = create_test_grid_no1();
        let cell_neighbours = grid.get_cell_neighbours(0, 1);

        assert_eq!(cell_neighbours.len(), 5);
    }

    #[test]
    fn test_get_cell_neighbours_1_0() {
        let grid = create_test_grid_no1();
        let cell_neighbours = grid.get_cell_neighbours(1, 0);

        assert_eq!(cell_neighbours.len(), 5);
    }

    #[test]
    fn test_get_cell_neighbours_max_max() {
        let grid = create_test_grid_no1();
        let cell_neighbours = grid.get_cell_neighbours(grid.n_cols-1, grid.n_rows-1);

        assert_eq!(cell_neighbours.len(), 3);
    }

    #[test]
    fn test_get_cell_neighbours_0_max() {
        let grid = create_test_grid_no1();
        let cell_neighbours = grid.get_cell_neighbours(0, grid.n_rows-1);

        assert_eq!(cell_neighbours.len(), 3);
    }

    #[test]
    fn test_get_cell_neighbours_max_0() {
        let grid = create_test_grid_no1();
        let cell_neighbours = grid.get_cell_neighbours(grid.n_cols-1, 0);

        assert_eq!(cell_neighbours.len(), 3);
    }


    #[test]
    fn test_get_cell_neighbours_1_max() {
        let grid = create_test_grid_no1();
        let cell_neighbours = grid.get_cell_neighbours(1, grid.n_rows-1);

        assert_eq!(cell_neighbours.len(), 5);
    }

    #[test]
    fn test_get_cell_neighbours_max_1() {
        let grid = create_test_grid_no1();
        let cell_neighbours = grid.get_cell_neighbours(grid.n_cols-1, 1);

        assert_eq!(cell_neighbours.len(), 5);
    }
}