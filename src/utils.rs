use crate::prelude::*;
/// Grid coordinates to world coordinates
pub fn grid_to_translation(grid_pos: GridPosition) -> Vec3 {
    Vec3::new(
        (grid_pos.x * GRID_BLOCK_SIZE - GRID_BLOCK_SIZE / 2) as f32,
        (grid_pos.y * GRID_BLOCK_SIZE - GRID_BLOCK_SIZE / 2) as f32,
        2.,
    )
}

/// World coordinates to grid coordinates
pub fn translation_to_grid_pos(translation: Vec3) -> Option<GridPosition> {
    let x = (translation.x as i32) / GRID_BLOCK_SIZE + 1;
    let y = (translation.y as i32) / GRID_BLOCK_SIZE + 1;

    GridPosition::try_new(x, y)
}

/// Snaps arbitrary coordinates to align with the in-game grid
pub fn snap_to_grid(translation: Vec3) -> Vec3 {
    grid_to_translation(translation_to_grid_pos(translation).unwrap())
}

pub fn idx_to_grid_pos(idx: usize, world_width: i32, world_height: i32) -> GridPosition {
    if idx > world_width as usize * world_height as usize || world_width < 1 || world_height < 1 {
        panic!("World width, height, and idx are in consistent");
    }
    let y = idx as i32 / world_width;
    let x = idx as i32 % world_width;

    GridPosition { x, y }
}

pub fn grid_pos_to_idx(
    tile_coords: &GridPosition,
    world_width: i32,
    world_height: i32,
) -> Option<usize> {
    if !is_in_bounds(tile_coords, world_width, world_height) {
        // println!("Tile not in bounds: {} {}", tile_coords.x, tile_coords.y);
        return None;
    }

    let w = world_width;

    Some((tile_coords.y * w + tile_coords.x) as usize)
}

/// Check if the tile in inside the bounds of the world
/// returns true for [0, WORLD_SIZE)
pub fn is_in_bounds(tile_coords: &GridPosition, world_width: i32, world_height: i32) -> bool {
    let x = tile_coords.x;
    let y = tile_coords.y;

    x >= 0 && y >= 0 && x < world_width && y < world_height
}
