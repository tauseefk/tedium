use crate::prelude::*;

pub fn visibility_calculation_system(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    visibile_blocks: Query<Entity, With<Hidden>>,
    wall_blocks: Query<&Transform, With<Wall>>,
    mut visibility: ResMut<crate::field_of_view::Visibility>,
) {
    player.iter().for_each(|transform| {
        if let Some(player_grid_pos) = translation_to_grid_pos(transform.translation) {
            visibility.observer = player_grid_pos;
        }
    });

    let mut wall_position_hash_set = HashSet::<usize>::new();

    wall_blocks.iter().for_each(|transform| {
        let grid_position = translation_to_grid_pos(transform.translation);

        if let Some(grid_position) = grid_position {
            if let Some(idx) = grid_pos_to_idx(&grid_position, GRID_CELL_COUNT, GRID_CELL_COUNT) {
                wall_position_hash_set.insert(idx);
            }
        }
    });

    let mut tiles: Vec<TileType> =
        vec![TileType::Transparent; (GRID_CELL_COUNT * GRID_CELL_COUNT) as usize];
    for x in 0..=GRID_CELL_COUNT {
        for y in 0..=GRID_CELL_COUNT {
            let idx = grid_pos_to_idx(&GridPosition { x, y }, GRID_CELL_COUNT, GRID_CELL_COUNT);
            if let Some(idx) = idx {
                if wall_position_hash_set.contains(&idx) {
                    tiles[idx] = TileType::Opaque;
                }
            }
        }
    }

    // debug_tiles(&tiles);

    let world = crate::field_of_view::World {
        tiles,
        width: GRID_CELL_COUNT,
        height: GRID_CELL_COUNT,
    };

    for entity in visibile_blocks.iter() {
        visibility.drain_visible_tiles();
        commands.entity(entity).despawn_recursive();
    }

    let result = visibility.compute_visible_tiles(&world);

    // GRID cells start at 1,1 and end at GRID_CELL_COUNT, GRID_CELL_COUNT
    for x in 1..=GRID_CELL_COUNT {
        for y in 1..=GRID_CELL_COUNT {
            let grid_pos = GridPosition::try_new(x, y);
            if let Some(grid_pos) = grid_pos {
                if !result.contains_key(&grid_pos) {
                    let world_pos = grid_to_translation(grid_pos);
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(VISIBILITY_DEBUG_SIZE)),
                                color: DARK_OVERLAY.with_alpha(
                                    (1 - (LIGHT_INTENSITY / (LIGHT_HEIGHT + MAX_VISIBLE_DISTANCE)))
                                        as f32,
                                ),
                                ..Default::default()
                            },
                            transform: Transform {
                                translation: Vec3::new(world_pos.x, world_pos.y, 4.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Hidden { visibility: 0 });
                } else {
                    let tile_dist = result.get(&grid_pos).unwrap();
                    let world_pos = grid_to_translation(grid_pos);
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(VISIBILITY_DEBUG_SIZE)),
                                color: DARK_OVERLAY.with_alpha(
                                    (1. - (LIGHT_INTENSITY as f32
                                        / (LIGHT_HEIGHT + tile_dist) as f32))
                                        as f32,
                                ),
                                ..Default::default()
                            },
                            transform: Transform {
                                translation: Vec3::new(world_pos.x, world_pos.y, 4.),
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
