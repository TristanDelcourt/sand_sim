mod config;
mod grid;
mod render;
mod rules;

use crate::{
    config::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH},
    grid::{Grid, Material},
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
    let mut is_drawing = false;
    let mut drawing_material = Material::Sand;

    let mut grid = Grid::new();

    loop {
        if is_mouse_button_pressed(MouseButton::Left) {
            is_drawing = true;
        }
        if is_mouse_button_released(MouseButton::Left) {
            is_drawing = false;
        }

        while let Some(c) = get_char_pressed() {
            match c {
                '&' | '1' => drawing_material = Material::Sand,
                'é' | '2' => drawing_material = Material::Water,
                //'"' | '3' => drawing_material = Material::Stone,
                //'\'' | '4' => drawing_material = Material::Wood,
                //'(' | '5' => drawing_material = Material::Fire,
                //'-' | '6' => drawing_material = Material::Smoke,
                _ => {} // Ignore any other keys
            }
        }

        if is_drawing {
            let (mouse_x, mouse_y) = mouse_position();
            let grid_x = (mouse_x / CELL_SIZE as f32) as usize;
            let grid_y = (mouse_y / CELL_SIZE as f32) as usize;
            grid.paint(grid_x, grid_y, drawing_material);
        }

        grid.draw();
        grid.update();

        next_frame().await
    }
}
