use macroquad::rand::RandomRange;

use crate::{
    config::{GRID_HEIGHT, GRID_WIDTH},
    grid::{Cell, Grid, Material},
};

fn density(material: Material) -> u8 {
    match material {
        Material::Air => 0,
        Material::Smoke => 1,
        Material::Fire => 2,
        Material::Water => 3,
        Material::Sand => 4,
        Material::Wood => 5,
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
            if grid.in_bounds(x as isize + dx, y as isize + 1)
                && can_displace(
                    grid.get(x, y).material,
                    grid.get((x as isize + dx) as usize, y + 1).material,
                )
            {
                grid.swap(x, y, (x as isize + dx) as usize, y + 1);
            } else if grid.in_bounds(x as isize - dx, y as isize + 1)
                && can_displace(
                    grid.get(x, y).material,
                    grid.get((x as isize - dx) as usize, y + 1).material,
                )
            {
                grid.swap(x, y, (x as isize - dx) as usize, y + 1);
            }

            let spread_distance = RandomRange::gen_range(1, 5) as isize;
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

        Material::Fire => {
            grid.set(
                x,
                y,
                Cell {
                    material: Material::Fire,
                    updated: true,
                    lifetime: grid.get(x, y).lifetime - 1,
                },
            );

            if grid.get(x, y).lifetime == 0 {
                grid.set(
                    x,
                    y,
                    Cell {
                        material: Material::Smoke,
                        updated: true,
                        lifetime: 150,
                    },
                );
                return;
            }

            for dx in -1..=1 {
                for dy in -1..=1 {
                    if grid.in_bounds(x as isize + dx, y as isize + dy) {
                        let target_material = grid
                            .get((x as isize + dx) as usize, (y as isize + dy) as usize)
                            .material;
                        if target_material == Material::Wood {
                            grid.set(
                                (x as isize + dx) as usize,
                                (y as isize + dy) as usize,
                                Cell {
                                    material: Material::Fire,
                                    updated: true,
                                    lifetime: 20,
                                },
                            );
                        }
                    }
                }
            }

            if grid.in_bounds(x as isize, y as isize - 1)
                && can_displace(grid.get(x, y).material, grid.get(x, y - 1).material)
            {
                grid.swap(x, y, x, y - 1);
                return;
            }
        }

        Material::Smoke => {
            //Smoke dissipation logic, not prefered
            //grid.set(
            //    x,
            //    y,
            //    Cell {
            //        material: Material::Smoke,
            //        updated: true,
            //        lifetime: grid.get(x, y).lifetime - 1,
            //    },
            //);
            //
            //if grid.get(x, y).lifetime == 0 {
            //    grid.set(
            //        x,
            //        y,
            //        Cell {
            //            material: Material::Air,
            //            updated: true,
            //            lifetime: 0,
            //        },
            //    );
            //    return;
            //}

            if y <= 1 {
                return;
            }

            let horizontal_move = RandomRange::gen_range(0., 1.) < 0.3;
            let dx = if horizontal_move {
                (RandomRange::gen_range(0, 2) * 2 - 1) as isize
            } else {
                0
            };
            if grid.in_bounds(x as isize + dx, y as isize - 1)
                && can_displace(
                    grid.get(x, y).material,
                    grid.get((x as isize + dx) as usize, y - 1).material,
                )
            {
                grid.swap(x, y, (x as isize + dx) as usize, y - 1);
                return;
            }

            if x == 0 && can_displace(grid.get(x, y).material, grid.get(x + 1, y - 1).material) {
                grid.swap(x, y, x + 1, y - 1);
                return;
            }

            if x == GRID_WIDTH - 1
                && can_displace(grid.get(x, y).material, grid.get(x - 1, y - 1).material)
            {
                grid.swap(x, y, x - 1, y - 1);
                return;
            }

            let dx = (RandomRange::gen_range(0, 2) * 2 - 1) as isize;
            if grid.in_bounds(x as isize + dx, y as isize - 1)
                && can_displace(
                    grid.get(x, y).material,
                    grid.get((x as isize + dx) as usize, y - 1).material,
                )
            {
                grid.swap(x, y, (x as isize + dx) as usize, y - 1);
            } else if grid.in_bounds(x as isize - dx, y as isize - 1)
                && can_displace(
                    grid.get(x, y).material,
                    grid.get((x as isize - dx) as usize, y - 1).material,
                )
            {
                grid.swap(x, y, (x as isize - dx) as usize, y - 1);
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

        Material::Wood => {}
        Material::Stone => {}
        Material::Air => {}
    };
}
