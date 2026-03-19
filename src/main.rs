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
use macroquad::{
    prelude::*,
    ui::{Skin, hash, root_ui},
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Sand Simulation".to_owned(),
        window_width: (GRID_WIDTH * CELL_SIZE) as i32,
        window_height: (GRID_HEIGHT * CELL_SIZE) as i32,
        platform: miniquad::conf::Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut is_drawing = false;
    let mut drawing_material = Material::Sand;
    let target_frame_time = 1. / TARGET_FPS as f64;

    let mut grid = Grid::new();

    let window_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(10.0, 10.0, 10.0, 10.0))
        .color(Color::from_rgba(30, 30, 30, 240))
        .text_color(WHITE)
        .build();

    let button_style = root_ui()
        .style_builder()
        .margin(RectOffset::new(10.0, 10.0, 5.0, 5.0))
        .color(Color::from_rgba(60, 60, 60, 255))
        .color_hovered(Color::from_rgba(90, 90, 90, 255))
        .color_clicked(Color::from_rgba(120, 120, 120, 255))
        .color_selected(Color::from_rgba(120, 120, 120, 255))
        .text_color(WHITE)
        .build();

    let label_style = root_ui()
        .style_builder()
        .text_color(Color::from_rgba(200, 200, 200, 255))
        .build();

    // 2. Combine them into a new Skin
    let custom_skin = Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin() // Inherit default settings for things we didn't customize (like checkboxes)
    };

    loop {
        let frame_start_time = get_time();

        let ui_x = 10.;
        let ui_y = 30.;
        let ui_w = 80.;
        let ui_h = 190.;

        root_ui().push_skin(&custom_skin);
        root_ui().window(hash!(), vec2(ui_x, ui_y), vec2(ui_w, ui_h), |ui| {
            if ui.button(None, "Sand ") {
                drawing_material = Material::Sand;
            }
            if ui.button(None, "Water") {
                drawing_material = Material::Water;
            }
            if ui.button(None, "Stone") {
                drawing_material = Material::Stone;
            }
            if ui.button(None, "Wood ") {
                drawing_material = Material::Wood;
            }
            if ui.button(None, "Fire ") {
                drawing_material = Material::Fire;
            }
            if ui.button(None, "Smoke") {
                drawing_material = Material::Smoke;
            }
        });

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

        let (mouse_x, mouse_y) = mouse_position();
        let ui_rect = Rect::new(ui_x, ui_y, ui_w, ui_h);
        let mouse_over_ui = ui_rect.contains(vec2(mouse_x, mouse_y));
        if is_drawing && !mouse_over_ui {
            let grid_x = (mouse_x / CELL_SIZE as f32) as usize;
            let grid_y = (mouse_y / CELL_SIZE as f32) as usize;
            grid.paint(grid_x, grid_y, drawing_material);
        }

        grid.draw();
        grid.update();
        grid.draw_stats();

        let fps_text = format!("{}", get_fps());
        draw_text(&fps_text, 5., 15., 16., WHITE);

        let frame_time = get_time() - frame_start_time;
        if frame_time < target_frame_time {
            let sleep_time = target_frame_time - frame_time;
            thread::sleep(Duration::from_secs_f64(sleep_time));
        }

        next_frame().await
    }
}
