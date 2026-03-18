use macroquad::rand::RandomRange;

use crate::{
    config::{GRID_HEIGHT, GRID_WIDTH},
    grid::{Grid, Material},
};

fn density(material: Material) -> u8 {
    match material {
        Material::Air => 0,
        Material::Smoke => 1,
        Material::Fire => 2,
        Material::Wood => 3,
        Material::Sand => 4,
        Material::Water => 5,
        Material::Stone => 6,
    }
}

fn can_displace(mover: Material, target: Material) -> bool {
    density(mover) > density(target)
}

pub fn update(grid: &mut Grid, x: usize, y: usize) {
    match grid.get(x, y).material {
        Material::Sand => {
            if y >= GRID_HEIGHT - 1 {
                return;
            }

            if can_displace(grid.get(x, y).material, grid.get(x, y + 1).material) {
                grid.swap(x, y, x, y + 1);
                return;
            }

            if x == 0 && can_displace(grid.get(x, y).material, grid.get(x + 1, y + 1).material) {
                grid.swap(x, y, x + 1, y + 1);
                return;
            }

            if x == GRID_WIDTH - 1
                && can_displace(grid.get(x, y).material, grid.get(x - 1, y + 1).material)
            {
                grid.swap(x, y, x + 1, y + 1);
                return;
            }

            let dx = RandomRange::gen_range(-1, 2) as isize;
            if can_displace(
                grid.get(x, y).material,
                grid.get((x as isize + dx) as usize, y + 1).material,
            ) {
                grid.swap(x, y, (x as isize + dx) as usize, y + 1);
            }
        }

        Material::Water => {}
        Material::Fire => {}
        Material::Smoke => {}
        Material::Wood => {}
        Material::Stone => {}
        Material::Air => {}
    };
}
