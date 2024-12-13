use crate::prelude::*;

#[derive(Resource)]
pub struct VisContainer {
    pub visibility: lumos::Visibility,
}

pub fn visibility_calculation_system(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    visibile_blocks: Query<Entity, With<Hidden>>,
    wall_blocks: Query<&Transform, With<Wall>>,
    mut vis_container: ResMut<VisContainer>,
    debug_config: Res<DebugConfig>,
) {
    let world_dimensions = WorldDimensions {
        rows: GRID_CELL_COUNT,
        cols: GRID_CELL_COUNT,
        cell_width: GRID_BLOCK_SIZE,
    };
    player.iter().for_each(|transform| {
        if let Some(player_grid_pos) = translation_to_grid_pos(
            Vec3::new(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
            ),
            &world_dimensions,
        ) {
            vis_container.visibility.observer = player_grid_pos;
        }
    });

    let mut wall_position_hash_set = HashSet::<usize>::new();

    wall_blocks.iter().for_each(|transform| {
        let grid_position = translation_to_grid_pos(
            Vec3::new(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
            ),
            &world_dimensions,
        );

        if let Some(grid_position) = grid_position {
            if let Some(idx) = grid_pos_to_idx(&grid_position, &world_dimensions) {
                wall_position_hash_set.insert(idx);
            }
        }
    });

    let mut tiles: Vec<TileType> =
        vec![TileType::Transparent; (GRID_CELL_COUNT * GRID_CELL_COUNT) as usize];
    for x in 0..=GRID_CELL_COUNT {
        for y in 0..=GRID_CELL_COUNT {
            let idx = grid_pos_to_idx(&GridPosition { x, y }, &world_dimensions);
            if let Some(idx) = idx {
                if wall_position_hash_set.contains(&idx) {
                    tiles[idx] = TileType::Opaque;
                }
            }
        }
    }

    // debug_tiles(&tiles);

    let world = TileGrid {
        tiles,
        grid_dimensions: world_dimensions,
    };

    for entity in visibile_blocks.iter() {
        vis_container.visibility.drain_visible_tiles();
        commands.entity(entity).despawn_recursive();
    }

    let result = vis_container.visibility.compute_visible_tiles(&world);

    // GRID cells start at 1,1 and end at GRID_CELL_COUNT, GRID_CELL_COUNT
    for x in 1..=GRID_CELL_COUNT {
        for y in 1..=GRID_CELL_COUNT {
            let grid_pos = GridPosition::try_new(x, y, &world_dimensions);
            if let Some(grid_pos) = grid_pos {
                if !result.contains_key(&grid_pos) {
                    let world_pos = grid_position_to_translation(grid_pos, &world.grid_dimensions);
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(VISIBILITY_DEBUG_SIZE)),
                                color: DARK_OVERLAY.with_alpha(
                                    (1 - (debug_config.light_intensity.value
                                        / (debug_config.light_height.value + MAX_VISIBLE_DISTANCE)))
                                        as f32,
                                ),
                                ..Default::default()
                            },
                            transform: Transform {
                                translation: bevy::prelude::Vec3::new(world_pos.x, world_pos.y, 4.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Hidden { visibility: 0 });
                } else {
                    let tile_dist = result.get(&grid_pos).unwrap();
                    let world_pos = grid_position_to_translation(grid_pos, &world.grid_dimensions);
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(VISIBILITY_DEBUG_SIZE)),
                                color: DARK_OVERLAY.with_alpha(
                                    (1. - (debug_config.light_intensity.value as f32
                                        / (debug_config.light_height.value + tile_dist) as f32))
                                        as f32,
                                ),
                                ..Default::default()
                            },
                            transform: Transform {
                                translation: bevy::prelude::Vec3::new(world_pos.x, world_pos.y, 4.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Hidden { visibility: 4 });
                }
            }
        }
    }
}
