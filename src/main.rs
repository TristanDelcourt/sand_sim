mod config;
mod grid;
mod render;
mod rules;

use std::{thread, time::Duration};

use crate::{
    config::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH, TARGET_FPS},
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
    let target_frame_time = 1. / TARGET_FPS as f64;

    let mut grid = Grid::new();

    loop {
        let frame_start_time = get_time();

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
                '"' | '3' => drawing_material = Material::Stone,
                '\'' | '4' => drawing_material = Material::Wood,
                '(' | '5' => drawing_material = Material::Fire,
                '-' | '6' => drawing_material = Material::Smoke,
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

        let fps_text = format!("{}", get_fps());
        draw_text(&fps_text, 10.0, 25.0, 30.0, WHITE);

        let frame_time = get_time() - frame_start_time;
        if frame_time < target_frame_time {
            let sleep_time = target_frame_time - frame_time;
            thread::sleep(Duration::from_secs_f64(sleep_time));
        }

        next_frame().await
    }
}
