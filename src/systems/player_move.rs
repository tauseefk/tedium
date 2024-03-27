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

    for event in player_move_events.iter() {
        let translation_delta: Vec3 = match event.direction {
            PlayerMoveDirection::Up => Vec3 {
                x: 0.,
                y: GRID_BLOCK_SIZE as f32,
                z: 0.,
            },
            PlayerMoveDirection::Down => Vec3 {
                x: 0.,
                y: -1. * GRID_BLOCK_SIZE as f32,
                z: 0.,
            },
            PlayerMoveDirection::Left => Vec3 {
                x: -1. * GRID_BLOCK_SIZE as f32,
                y: 0.,
                z: 0.,
            },
            PlayerMoveDirection::Right => Vec3 {
                x: GRID_BLOCK_SIZE as f32,
                y: 0.,
                z: 0.,
            },
        };
        let updated_position = snap_to_grid(player.translation + translation_delta);
        if let Some(translation) = updated_position {
            player.translation = translation;
        }
    }
}
