use crate::prelude::*;

pub fn visibility_calc(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    visibile_blocks: Query<Entity, With<Hidden>>,
    mut visibility: ResMut<crate::field_of_view::Visibility>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    player.iter().for_each(|transform| {
        if let Some(player_grid_pos) = translation_to_grid_pos(transform.translation) {
            visibility.observer = player_grid_pos;
        }
    });

    level_query.iter().for_each(|(_level_handle, level_iid)| {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("Project should be loaded if level has spawned");

        let ldtk_level = ldtk_project
            .as_standalone()
            .get_loaded_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");

        // Wall layer also stores info about floors
        let layer_instance = &ldtk_level
            .layer_instances()
            .iter()
            .find(|layer| layer.identifier == "Walls")
            .expect("IntGrid layer not found");

        // int_grid_csv returns y_flipped tiles for some reason
        let rows_y_flipped: Vec<&[i32]> = layer_instance
            .int_grid_csv
            .chunks(GRID_CELL_COUNT as usize)
            .rev()
            .collect();

        // translate rows in x and y direction by 1 position

        let row_length = rows_y_flipped[0].len();
        let x_translated_rows = rows_y_flipped
            .iter()
            .map(|row| {
                let mut new_row = Vec::with_capacity(row_length);

                // Add zero at the beginning
                new_row.push(WALKABLE_INT_GRID_VALUE);

                new_row.extend_from_slice(&row[0..row.len() - 1]);
                new_row
            })
            .collect();

        let zero_row = vec![WALKABLE_INT_GRID_VALUE; row_length];
        let xy_translated_rows = [vec![zero_row], x_translated_rows].concat();
        let xy_translated_rows = xy_translated_rows[0..xy_translated_rows.len() - 1].iter();

        let tiles_y_flipped: Vec<TileType> = xy_translated_rows
            .flatten()
            .map(|int_grid_tile_value| match int_grid_tile_value {
                1 => TileType::Opaque,
                _ => TileType::Transparent,
            })
            .collect();

        let world = crate::field_of_view::World {
            tiles: tiles_y_flipped,
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
                    if !result.contains(&grid_pos) {
                        let world_pos = grid_to_translation(grid_pos);
                        commands
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(VISIBILITY_DEBUG_SIZE)),
                                    color: DARK_OVERLAY.with_alpha(0.4),
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
                        let world_pos = grid_to_translation(grid_pos);
                        commands
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(VISIBILITY_DEBUG_SIZE)),
                                    color: DARK_OVERLAY.with_alpha(0.2),
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
    });
}
