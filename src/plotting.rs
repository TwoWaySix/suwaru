use std::path::{Path, PathBuf};
use image::{ImageBuffer, Rgb, RgbImage};
use crate::grid::Grid;

pub fn plot_depths(grid: &Grid, path: &Path) {
    let mut img = RgbImage::new(
        grid.n_cols as u32,
        grid.n_rows as u32
    );

    for y in 0..grid.n_rows {
        for x in 0..grid.n_cols {
            let scale = scale_color(grid.get_cell(x, y).depth, 0.5);
            let color = Rgb([255-scale, 255-scale, 255]);
            img.put_pixel(x as u32, y as u32, color);
        }
    }
    img.save(path).unwrap();
}

fn scale_color(val: f32, max_val: f32) -> u8 {
    if val >= max_val {
        return 255;
    }
    (val*255.0/max_val) as u8
}