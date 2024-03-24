use crate::prelude::*;

/// Toggle the wall block at the location described in the event
pub fn toggle_wall(
    mut toggle_wall_block: EventReader<ToggleWallBlockEvent>,
    blocks: Query<(Entity, &Transform), With<Wall>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("wall.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(GRID_BLOCK_SIZE as f32, GRID_BLOCK_SIZE as f32),
        8,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for event in toggle_wall_block.iter() {
        let event: &ToggleWallBlockEvent = event;
        match blocks.iter().find(|(_, transform)| {
            translation_to_grid_pos(transform.translation).unwrap()
                == translation_to_grid_pos(event.translation).unwrap()
        }) {
            None => {
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: Transform {
                            translation: event.translation,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Wall);
            }
            Some((entity, _)) => {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
