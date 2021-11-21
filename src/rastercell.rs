#[derive(Debug, Clone)]
pub struct RasterCell {
    pub x: usize,
    pub y: usize,
    pub elevation: f32,
    pub depth: f32,
    pub kst: f32,
}

impl RasterCell {
    pub fn new() -> RasterCell {
        RasterCell {
            x: 0,
            y: 0,
            elevation: 0.0,
            depth: 0.0,
            kst: 30.0,
        }
    }

    pub fn new_with_xy(x: usize, y: usize) -> RasterCell {
        RasterCell {
            x,
            y,
            elevation: 0.0,
            depth: 0.0,
            kst: 30.0,
        }
    }

    pub fn new_with_xyz(x: usize, y: usize, elevation: f32) -> RasterCell {
        RasterCell {
            x,
            y,
            elevation,
            depth: 0.0,
            kst: 30.0,
        }
    }

    pub fn new_with_xyzh(x: usize, y: usize, elevation: f32, depth: f32) -> RasterCell {
        RasterCell {
            x,
            y,
            elevation,
            depth,
            kst: 30.0,
        }
    }

    pub fn water_level(&self) -> f32 {
        self.elevation + self.depth
    }
}

#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn test_water_level() {
        let mut cell = RasterCell::new_with_xyz(0, 0, 10.5);
        cell.depth = 0.5;

        assert_eq!(cell.water_level(), 11.0);
    }
}
