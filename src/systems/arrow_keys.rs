use crate::prelude::*;

pub fn arrow_keys(
    arrow_keys_input: Res<Input<KeyCode>>,
    mut player_move_event: EventWriter<PlayerMoveEvent>,
) {
    if arrow_keys_input.just_pressed(KeyCode::Left) {
        player_move_event.send(PlayerMoveEvent {
            direction: PlayerMoveDirection::Left,
        });
    } else if arrow_keys_input.just_pressed(KeyCode::Right) {
        player_move_event.send(PlayerMoveEvent {
            direction: PlayerMoveDirection::Right,
        });
    } else if arrow_keys_input.just_pressed(KeyCode::Up) {
        player_move_event.send(PlayerMoveEvent {
            direction: PlayerMoveDirection::Up,
        });
    } else if arrow_keys_input.just_pressed(KeyCode::Down) {
        player_move_event.send(PlayerMoveEvent {
            direction: PlayerMoveDirection::Down,
        });
    }
}
