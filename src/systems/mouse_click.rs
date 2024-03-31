use crate::prelude::*;

/// Listens to mouse events and triggers appropriate in game events
pub fn mouse_click(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut update_debug_values: EventWriter<UpdateDebugValuesEvent>,
    // mut cycle_point_of_interest: EventWriter<CyclePOIEvent>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(window) = windows.get_primary() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Some(translation) = snap_to_grid(Vec3::new(cursor_pos.x, cursor_pos.y, 1.)) {
                    update_debug_values.send(UpdateDebugValuesEvent { translation });
                }
            }
        }
    }

    // if mouse_button_input.just_pressed(MouseButton::Right) {
    //     cycle_point_of_interest.send(CyclePOIEvent {});
    // }
}
