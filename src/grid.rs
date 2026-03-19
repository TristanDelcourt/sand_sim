use crate::{config::*, render::Texturable, rules};
use macroquad::{
    color::WHITE,
    math::vec2,
    text::draw_text,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
};

#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub updated: bool,
    pub lifetime: u16,
}

struct Stats {
    sand_count: usize,
    water_count: usize,
    stone_count: usize,
    wood_count: usize,
    fire_count: usize,
    smoke_count: usize,
}

struct Config {
    draw_counts: bool,
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    texture: Texture2D,
    texture_bytes: Vec<u8>,
    stats: Stats,
    config: Config,
}

impl Grid {
    pub fn new() -> Self {
        let cells = vec![
            Cell {
                material: Material::Air,
                updated: false,
                lifetime: 0,
            };
            (GRID_WIDTH * GRID_HEIGHT) as usize
        ];

        let bytes = cells
            .iter()
            .flat_map(|cell| {
                let color = match cell.material {
                    Material::Air => [100, 100, 100, 255],
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

        let texture = Texture2D::from_rgba8(GRID_WIDTH as u16, GRID_HEIGHT as u16, &bytes);
        texture.set_filter(macroquad::texture::FilterMode::Nearest);

        Self {
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
            cells,
            texture,
            texture_bytes: bytes,
            stats: Stats {
                sand_count: 0,
                water_count: 0,
                stone_count: 0,
                wood_count: 0,
                fire_count: 0,
                smoke_count: 0,
            },
            config: Config { draw_counts: true },
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y * self.width + x] = cell;
    }

    // Written explicetly for performance reasons, as this is called very often
    pub fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let temp = self.get(x1, y1);

        self.set(
            x1,
            y1,
            Cell {
                material: self.get(x2, y2).material,
                updated: true,
                lifetime: self.get(x2, y2).lifetime,
            },
        );
        self.set(
            x2,
            y2,
            Cell {
                material: temp.material,
                updated: true,
                lifetime: temp.lifetime,
            },
        );
    }

    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        x < self.width as isize && y < self.height as isize && x >= 0 && y >= 0
    }

    fn clear(&mut self) {
        self.cells.iter_mut().for_each(|cell| {
            if cell.updated {
                cell.updated = false;
            }
        });

        self.stats = Stats {
            sand_count: 0,
            water_count: 0,
            stone_count: 0,
            wood_count: 0,
            fire_count: 0,
            smoke_count: 0,
        };
    }

    fn increment_stats(&mut self, material: Material) {
        match material {
            Material::Sand => self.stats.sand_count += 1,
            Material::Water => self.stats.water_count += 1,
            Material::Stone => self.stats.stone_count += 1,
            Material::Wood => self.stats.wood_count += 1,
            Material::Fire => self.stats.fire_count += 1,
            Material::Smoke => self.stats.smoke_count += 1,
            Material::Air => {}
        }
    }

    pub fn draw_stats(&self) {
        if self.config.draw_counts {
            let stats_text = format!(
                "| Sand: {} | Water: {} | Stone: {} | Wood: {} | Fire: {} | Smoke: {}",
                self.stats.sand_count,
                self.stats.water_count,
                self.stats.stone_count,
                self.stats.wood_count,
                self.stats.fire_count,
                self.stats.smoke_count
            );
            draw_text(&stats_text, 30., 15., 16., WHITE);
        }
    }

    pub fn update(&mut self) {
        self.clear();

        let mut left = false;

        for y in (0..self.height).rev() {
            if left {
                for x in (0..self.width).rev() {
                    let cell = self.get(x, y);
                    self.increment_stats(cell.material);
                    if !cell.updated {
                        rules::update(self, x, y);
                    }
                }
                left = !left;
            } else {
                for x in 0..self.width {
                    let cell = self.get(x, y);
                    self.increment_stats(cell.material);
                    if !cell.updated {
                        rules::update(self, x, y);
                    }
                }
                left = !left;
            }
        }
    }

    pub fn paint(&mut self, x: usize, y: usize, material: Material) {
        for dx in 0..DEFAULT_BRUSH_R {
            for dy in 0..DEFAULT_BRUSH_R {
                let offset_x = (dx as isize - DEFAULT_BRUSH_R as isize / 2) as isize;
                let offset_y = (dy as isize - DEFAULT_BRUSH_R as isize / 2) as isize;
                let px = (x as isize + offset_x) as usize;
                let py = (y as isize + offset_y) as usize;
                if self.in_bounds(x as isize + offset_x, y as isize + offset_y) {
                    if self.get(px, py).material == Material::Air {
                        self.set(
                            px,
                            py,
                            Cell {
                                material,
                                updated: false,
                                lifetime: if material == Material::Fire { 50 } else { 0 },
                            },
                        );
                    }
                }
            }
        }
    }
}

impl Texturable for Grid {
    fn draw(&mut self) {
        for (i, cell) in self.cells.iter().enumerate() {
            let color = match cell.material {
                Material::Air => [100, 100, 100, 255],
                Material::Sand => [194, 178, 128, 255],
                Material::Water => [64, 164, 223, 255],
                Material::Stone => [128, 128, 128, 255],
                Material::Wood => [139, 69, 19, 255],
                Material::Fire => [255, 69, 0, 255],
                Material::Smoke => [105, 105, 105, 128],
            };

            // Each cell takes up 4 bytes (R, G, B, A)
            let byte_idx = i * 4;
            self.texture_bytes[byte_idx] = color[0];
            self.texture_bytes[byte_idx + 1] = color[1];
            self.texture_bytes[byte_idx + 2] = color[2];
            self.texture_bytes[byte_idx + 3] = color[3];
        }

        self.texture
            .update_from_bytes(GRID_WIDTH as u32, GRID_HEIGHT as u32, &self.texture_bytes);

        draw_texture_ex(
            &self.texture,
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
