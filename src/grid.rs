use crate::{config::*, render::Texturable};
use macroquad::{
    color::WHITE,
    math::vec2,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
};

#[derive(Clone, Copy)]
pub enum Material {
    Air,
    Sand,
    Water,
    Stone,
    Wood,
    Fire,
    Smoke,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub material: Material,
    pub updated: u8,
    pub lifetime: u8,
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new() -> Self {
        let cells = vec![
            Cell {
                material: Material::Air,
                updated: 0,
                lifetime: 0,
            };
            (GRID_WIDTH * GRID_HEIGHT) as usize
        ];
        Self {
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
            cells,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y * self.width + x] = cell;
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    // Written exlicetly for performance reasons, as this is called very often
    pub fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let idx1 = y1 * self.width + x1;
        let idx2 = y2 * self.width + x2;
        let temp = self.cells[idx1];
        self.cells[idx1] = self.cells[idx2];
        self.cells[idx2] = temp;
    }
}

impl Texturable for Grid {
    fn draw(&self) {
        let bytes = self
            .cells
            .iter()
            .flat_map(|cell| {
                let color = match cell.material {
                    Material::Air => [0, 0, 0, 0],
                    Material::Sand => [194, 178, 128, 255],
                    Material::Water => [64, 164, 223, 255],
                    Material::Stone => [128, 128, 128, 255],
                    Material::Wood => [139, 69, 19, 255],
                    Material::Fire => [255, 69, 0, 255],
                    Material::Smoke => [105, 105, 105, 128],
                };
                color
            })
            .collect::<Vec<u8>>();

        let texture = Texture2D::from_rgba8(self.width as u16, self.height as u16, &bytes);
        texture.set_filter(macroquad::texture::FilterMode::Nearest);

        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    (self.width * CELL_SIZE) as f32,
                    (self.height * CELL_SIZE) as f32,
                )),
                ..Default::default()
            },
        );
    }
}
