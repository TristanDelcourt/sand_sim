mod config;
mod grid;
mod render;
mod rules;

use crate::{
    config::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH},
    grid::{Cell, Grid, Material},
    render::Texturable,
};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sand Simulation".to_owned(),
        window_width: (GRID_WIDTH * CELL_SIZE) as i32,
        window_height: (GRID_HEIGHT * CELL_SIZE) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid = Grid::new();
    grid.set(
        10,
        10,
        Cell {
            material: Material::Sand,
            updated: false,
            lifetime: 0,
        },
    );

    loop {
        grid.draw();
        grid.update();

        next_frame().await
    }
}
