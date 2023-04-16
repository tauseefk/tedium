use crate::prelude::*;

pub fn play_speed(
    keyboard_input: Res<Input<KeyCode>>,
    mut play_speed_query: Query<&mut PlaySpeed>,
) {
    let mut play_speed = play_speed_query.single_mut();

    if keyboard_input.pressed(KeyCode::P) {
        play_speed.multiplier = 2.0;
    }
    if keyboard_input.pressed(KeyCode::O) {
        play_speed.multiplier = 1.0;
    }
}
