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
        Material::Water => 4,
        Material::Sand => 5,
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
                grid.swap(x, y, x - 1, y + 1);
                return;
            }

            let dx = (RandomRange::gen_range(0, 2) * 2 - 1) as isize;
            if grid.in_bounds(x as isize + dx, y as isize)
                && can_displace(
                    grid.get(x, y).material,
                    grid.get((x as isize + dx) as usize, y + 1).material,
                )
            {
                grid.swap(x, y, (x as isize + dx) as usize, y + 1);
            } else if grid.in_bounds(x as isize - dx, y as isize)
                && can_displace(
                    grid.get(x, y).material,
                    grid.get((x as isize - dx) as usize, y + 1).material,
                )
            {
                grid.swap(x, y, (x as isize - dx) as usize, y + 1);
            }
        }

        Material::Water => {
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
                grid.swap(x, y, x - 1, y + 1);
                return;
            }

            let dx = (RandomRange::gen_range(0, 2) * 2 - 1) as isize;
            if grid.in_bounds(x as isize + dx, y as isize)
                && can_displace(
                    grid.get(x, y).material,
                    grid.get((x as isize + dx) as usize, y + 1).material,
                )
            {
                grid.swap(x, y, (x as isize + dx) as usize, y + 1);
            } else if grid.in_bounds(x as isize - dx, y as isize)
                && can_displace(
                    grid.get(x, y).material,
                    grid.get((x as isize - dx) as usize, y + 1).material,
                )
            {
                grid.swap(x, y, (x as isize - dx) as usize, y + 1);
            }

            let spread_distance = RandomRange::gen_range(1, 10) as isize;
            let direction = (RandomRange::gen_range(0, 2) * 2 - 1) as isize;

            for dx in 1..=spread_distance {
                if grid.in_bounds(x as isize + dx * direction, y as isize)
                    && can_displace(
                        grid.get(x, y).material,
                        grid.get((x as isize + dx * direction) as usize, y).material,
                    )
                {
                    grid.swap(x, y, (x as isize + dx * direction) as usize, y);
                    return;
                } else if grid.in_bounds(x as isize - dx * direction, y as isize)
                    && can_displace(
                        grid.get(x, y).material,
                        grid.get((x as isize - dx * direction) as usize, y).material,
                    )
                {
                    grid.swap(x, y, (x as isize - dx * direction) as usize, y);
                    return;
                }
            }
        }

        Material::Fire => {}
        Material::Smoke => {}
        Material::Wood => {}
        Material::Stone => {}
        Material::Air => {}
    };
}
