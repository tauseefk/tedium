pub fn animate_player_system(
    time: Res<Time>,
    mut frame_timer: ResMut<FrameTimer>,
    mut texture_query: Query<&mut TextureAtlasSprite, With<Player>>,
    mut animation_state_query: Query<&mut PlayerAnimationState, With<Player>>,
    slo_mo_query: Query<&SloMo>,
) {
    let inverse_speed_multiplier = slo_mo_query.single().inverse_speed_multiplier;
    let mut animation_state = animation_state_query.single_mut();

    for mut sprite in texture_query.iter_mut() {
        frame_timer
            .0
            .tick(time.delta() / (inverse_speed_multiplier as u32));
        if frame_timer.0.finished() {
            sprite.index = animation_state.wrapping_next_idx();
        }
    }
}
