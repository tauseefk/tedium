use crate::prelude::*;
/// Grid coordinates to world coordinates
pub fn grid_to_translation(grid_pos: GridPosition) -> Vec3 {
    Vec3::new(
        (grid_pos.x as i32 * GRID_BLOCK_SIZE - GRID_BLOCK_SIZE / 2) as f32,
        (grid_pos.y as i32 * GRID_BLOCK_SIZE - GRID_BLOCK_SIZE / 2) as f32,
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
