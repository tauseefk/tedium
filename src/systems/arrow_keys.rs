use crate::prelude::*;

pub fn arrow_keys(
    arrow_keys_input: Res<ButtonInput<KeyCode>>,
    mut player_move_event: EventWriter<PlayerMoveEvent>,
) {
    if arrow_keys_input.just_pressed(KeyCode::ArrowLeft) {
        player_move_event.send(PlayerMoveEvent {
            direction: PlayerMoveDirection::Left,
        });
    } else if arrow_keys_input.just_pressed(KeyCode::ArrowRight) {
        player_move_event.send(PlayerMoveEvent {
            direction: PlayerMoveDirection::Right,
        });
    } else if arrow_keys_input.just_pressed(KeyCode::ArrowUp) {
        player_move_event.send(PlayerMoveEvent {
            direction: PlayerMoveDirection::Up,
        });
    } else if arrow_keys_input.just_pressed(KeyCode::ArrowDown) {
        player_move_event.send(PlayerMoveEvent {
            direction: PlayerMoveDirection::Down,
        });
    }
}
