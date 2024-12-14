use crate::prelude::*;

/// Cycle through points of interest in order
pub fn player_move(
    mut player_move_events: EventReader<PlayerMoveEvent>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if player_query.get_single().is_err() {
        return;
    }

    let mut player = player_query.single_mut();

    for event in player_move_events.read() {
        let translation_delta: bevy::prelude::Vec3 = match event.direction {
            PlayerMoveDirection::Up => shadowcaster::Vec3 {
                x: 0.,
                y: GRID_BLOCK_SIZE as f32,
                z: 0.,
            },
            PlayerMoveDirection::Down => shadowcaster::Vec3 {
                x: 0.,
                y: -1. * GRID_BLOCK_SIZE as f32,
                z: 0.,
            },
            PlayerMoveDirection::Left => shadowcaster::Vec3 {
                x: -1. * GRID_BLOCK_SIZE as f32,
                y: 0.,
                z: 0.,
            },
            PlayerMoveDirection::Right => shadowcaster::Vec3 {
                x: GRID_BLOCK_SIZE as f32,
                y: 0.,
                z: 0.,
            },
        };
        let updated_position = player.translation + translation_delta;
        let updated_position = shadowcaster::snap_to_grid(
            updated_position,
            &WorldDimensions {
                rows: GRID_CELL_COUNT,
                cols: GRID_CELL_COUNT,
                cell_width: GRID_BLOCK_SIZE,
            },
        );
        if let Some(updated_position) = updated_position {
            player.translation = updated_position;
        }
    }
}
