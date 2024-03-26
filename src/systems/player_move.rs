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
        match event.direction {
            PlayerMoveDirection::Up => {
                player.translation.y = player.translation.y + 1.;
            }
            PlayerMoveDirection::Down => {
                player.translation.y = player.translation.y - 1.;
            }
            PlayerMoveDirection::Left => {
                player.translation.x = player.translation.x - 1.;
            }
            PlayerMoveDirection::Right => {
                player.translation.x = player.translation.x + 1.;
            }
        }
    }
}
