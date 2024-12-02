use crate::prelude::*;

pub fn animate_player(
    time: Res<Time>,
    mut frame_timer: ResMut<FrameTimer>,
    mut animation_state_with_texture_query: Query<
        (&mut PlayerAnimationState, &mut TextureAtlas),
        With<Player>,
    >,
) {
    if animation_state_with_texture_query.get_single().is_err() {
        return;
    }

    let (mut animation_state, mut texture_sprite) = animation_state_with_texture_query.single_mut();

    frame_timer.0.tick(time.delta());
    if frame_timer.0.finished() {
        texture_sprite.index = animation_state.wrapping_next_idx();
    }
}
