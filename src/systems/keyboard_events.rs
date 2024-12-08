use crate::prelude::*;

pub fn keyboard_events_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_move_event: EventWriter<PlayerMoveEvent>,
    mut update_debug_config_event: EventWriter<UpdateDebugConfigEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        player_move_event.send(PlayerMoveEvent {
            direction: PlayerMoveDirection::Left,
        });
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        player_move_event.send(PlayerMoveEvent {
            direction: PlayerMoveDirection::Right,
        });
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        player_move_event.send(PlayerMoveEvent {
            direction: PlayerMoveDirection::Up,
        });
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        player_move_event.send(PlayerMoveEvent {
            direction: PlayerMoveDirection::Down,
        });
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
        if keyboard_input.just_pressed(KeyCode::KeyL) {
            update_debug_config_event.send(UpdateDebugConfigEvent {
                config_key: DebugConfigKey::LightIntensityDecrease,
            });
        } else if keyboard_input.just_pressed(KeyCode::KeyH) {
            update_debug_config_event.send(UpdateDebugConfigEvent {
                config_key: DebugConfigKey::LightHeightDecrease,
            });
        }
    } else {
        if keyboard_input.just_pressed(KeyCode::KeyL) {
            update_debug_config_event.send(UpdateDebugConfigEvent {
                config_key: DebugConfigKey::LightIntensityIncrease,
            });
        } else if keyboard_input.just_pressed(KeyCode::KeyH) {
            update_debug_config_event.send(UpdateDebugConfigEvent {
                config_key: DebugConfigKey::LightHeightIncrease,
            });
        }
    }
}
