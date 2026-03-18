mod config;
mod grid;
mod render;

use crate::{
    grid::{Cell, Grid, Material},
    render::Texturable,
};
use macroquad::prelude::*;

#[macroquad::main("Sand Simulation")]
async fn main() {
    let mut grid = Grid::new();
    grid.set(
        10,
        10,
        Cell {
            material: Material::Sand,
            updated: 0,
            lifetime: 0,
        },
    );

    loop {
        clear_background(WHITE);

        grid.draw();
        next_frame().await
    }
}
