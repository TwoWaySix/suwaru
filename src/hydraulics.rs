pub mod gauckler_manning_stricker {
    use crate::rastercell::RasterCell;

    pub fn flow(cell1: &RasterCell, cell2: &RasterCell, cell_size: f32) -> f32 {
        area(cell1, cell2, cell_size) * velocity(cell1, cell2, cell_size)
    }

    pub fn area(cell1: &RasterCell, cell2: &RasterCell, cell_size: f32) -> f32 {
        average_depth(cell1, cell2) * cell_size
    }

    pub fn average_depth(cell1: &RasterCell, cell2: &RasterCell) -> f32 {
        0.5*(cell1.depth + cell2.depth)
    }

    pub fn velocity(cell1: &RasterCell, cell2: &RasterCell, cell_size: f32) -> f32 {
        average_kst(cell1, cell2) *
            hydraulic_radius(cell1, cell2, cell_size).powf(2.0/3.0) *
            hydraulic_slope(cell1, cell2, cell_size).sqrt()
    }

    pub fn average_kst(cell1: &RasterCell, cell2: &RasterCell) -> f32 {
        0.5*(cell1.kst + cell2.kst)
    }

    pub fn hydraulic_radius(cell1: &RasterCell, cell2: &RasterCell, cell_size: f32) -> f32 {
        if cell1.depth == 0.0 && cell2.depth == 0.0 {
            return 0.0;
        }
        area(cell1, cell2, cell_size) / wetted_perimeter(cell1, cell2, cell_size)
    }

    pub fn wetted_perimeter(cell1: &RasterCell, cell2: &RasterCell, cell_size: f32) -> f32 {
        2.0*average_depth(cell1, cell2) + cell_size
    }

    pub fn hydraulic_slope(cell1: &RasterCell, cell2: &RasterCell, cell_size: f32) -> f32 {
        (cell1.water_level() - cell2.water_level()) / cell_size
    }
}

#[cfg(test)]
mod gms_tests {
    use crate::rastercell::RasterCell;
    use super::gauckler_manning_stricker::*;

    #[test]
    fn test_hydraulic_slope() {
        let cell_size = 1.0;
        let cell1 = RasterCell::new_with_xyzh(0, 0, 1.5, 0.25);
        let cell2 = RasterCell::new_with_xyzh(0, 0, 1.0, 0.5);

        assert_eq!(
            hydraulic_slope(&cell1, &cell2, cell_size),
            0.25
        );
    }

    #[test]
    fn test_wetted_perimeter() {
        let cell_size = 1.0;
        let cell1 = RasterCell::new_with_xyzh(0, 0, 1.5, 0.25);
        let cell2 = RasterCell::new_with_xyzh(0, 0, 1.0, 0.5);

        assert_eq!(
            wetted_perimeter(&cell1, &cell2, cell_size),
            1.0 + 0.25 + 0.5
        );
    }

    #[test]
    fn test_area() {
        let cell_size = 1.0;
        let cell1 = RasterCell::new_with_xyzh(0, 0, 1.5, 0.25);
        let cell2 = RasterCell::new_with_xyzh(0, 0, 1.0, 0.5);

        assert_eq!(
            area(&cell1, &cell2, cell_size),
            1.0 * 0.5*(0.25 + 0.5)
        );
    }

    #[test]
    fn test_hydraulic_radius() {
        let cell_size = 1.0;
        let mut cell1 = RasterCell::new_with_xyzh(0, 0, 1.5, 0.25);
        let mut cell2 = RasterCell::new_with_xyzh(0, 0, 1.0, 0.5);

        assert_eq!(
            hydraulic_radius(&cell1, &cell2, cell_size),
            (1.0 * 0.5*(0.25 + 0.5)) / (1.0 + 0.25 + 0.5)
        );

        cell1.depth = 0.0;
        cell2.depth = 0.0;

        assert_eq!(
            hydraulic_radius(&cell1, &cell2, cell_size),
            0.0
        )
    }

    #[test]
    fn test_average_kst() {
        let cell_size = 1.0;
        let cell1 = RasterCell::new_with_xyzh(0, 0, 1.5, 0.25);
        let mut cell2 = RasterCell::new_with_xyzh(0, 0, 1.0, 0.5);
        cell2.kst = 10.0;

        assert_eq!(
            average_kst(&cell1, &cell2),
            0.5 * (10.0 + 30.0)
        );
    }

    #[test]
    fn test_average_depth() {
        let cell_size = 1.0;
        let cell1 = RasterCell::new_with_xyzh(0, 0, 1.5, 0.25);
        let mut cell2 = RasterCell::new_with_xyzh(0, 0, 1.0, 0.5);
        cell2.kst = 10.0;

        assert_eq!(
            average_depth(&cell1, &cell2),
            0.5 * (0.25 + 0.5)
        );
    }

    #[test]
    fn test_velocity() {
        let cell_size = 1.0;
        let cell1 = RasterCell::new_with_xyzh(0, 0, 1.5, 0.25);
        let cell2 = RasterCell::new_with_xyzh(0, 0, 1.0, 0.5);

        let calc_vel = velocity(&cell1, &cell2, cell_size);
        let corr_vel = 5.37139064460491;

        assert_eq!(
            (calc_vel - corr_vel).abs() < 0.0001,
            true
        );
    }

    #[test]
    fn test_flow() {
        let cell_size = 1.0;
        let cell1 = RasterCell::new_with_xyzh(0, 0, 1.5, 0.25);
        let cell2 = RasterCell::new_with_xyzh(0, 0, 1.0, 0.5);

        let calc_flow = flow(&cell1, &cell2, cell_size);
        let corr_flow = 2.01427149172684;

        assert_eq!(
            (calc_flow - corr_flow).abs() < 0.0001,
            true
        );
    }
}