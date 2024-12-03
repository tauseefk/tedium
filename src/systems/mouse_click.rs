use crate::prelude::*;

/// Listens to mouse events and triggers appropriate in game events
pub fn mouse_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut cycle_point_of_interest: EventWriter<CyclePOIEvent>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        cycle_point_of_interest.send(CyclePOIEvent {});
    }
}
