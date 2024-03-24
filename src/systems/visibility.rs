use crate::prelude::*;

pub fn visibility_calc(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    visibile_blocks: Query<Entity, With<Visible>>,
    hidden_blocks: Query<Entity, With<Hidden>>,
    mut visibility: ResMut<crate::field_of_view::Visibility>,
    levels: Res<Assets<LdtkLevel>>,
    level_query: Query<&Handle<LdtkLevel>>,
) {
    player.iter().for_each(|transform| {
        if let Some(player_grid_pos) = translation_to_grid_pos(transform.translation) {
            visibility.observer = player_grid_pos;
        }
    });

    level_query.for_each(|level_handle| {
        let ldtk_level = levels
            .get(level_handle)
            .expect("Level should be loaded by this point");

        let layer_instance = &ldtk_level
            .level
            .layer_instances
            .clone()
            .expect("Level asset should have layers")[2];

        // the grid is inverted on the x axis
        // #[rustfmt::skip]
        // let debug_tiles = vec![
        //     '_', 'o', '_', '_', '_', '_', '_', '_', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     'o', 'o', '_', '_', '_', '_', '_', '_', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     '_', '_', '_', '_', '_', '_', '_', '_', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     '_', '_', '_', '_', '_', '_', '_', '_', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     '_', '_', '_', '_', '_', '_', '_', '_', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     '_', '_', '_', '_', '_', '_', '_', '_', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     '_', '_', '_', '_', '_', '_', '_', '_', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     '_', '_', '_', '_', '_', '_', '_', '_', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o'
        // ];

        // let tiles = debug_tiles
        //     .iter()
        //     .map(|int_grid_tile_value| match int_grid_tile_value {
        //         '_' => TileType::Transparent,
        //         _ => TileType::Opaque,
        //     })
        //     .collect();

        let mut stuff: Vec<char> = vec![];
        let tiles = layer_instance
            .int_grid_csv
            // let tiles = debug_tiles
            .iter()
            .map(|int_grid_tile_value| match int_grid_tile_value {
                2 => {
                    stuff.push('_');
                    TileType::Transparent
                }
                _ => {
                    stuff.push('o');
                    TileType::Opaque
                }
            })
            .collect();

        // println!("{stuff:?}");

        let world = crate::field_of_view::World {
            tiles,
            width: GRID_BLOCK_SIZE,
            height: GRID_BLOCK_SIZE,
        };

        for entity in visibile_blocks.iter() {
            visibility.drain_visible_tiles();
            commands.entity(entity).despawn_recursive();
        }

        for entity in hidden_blocks.iter() {
            commands.entity(entity).despawn_recursive();
        }

        let result = visibility.compute_visible_tiles(&world);

        for x in 0..=GRID_CELL_COUNT {
            for y in 0..=GRID_CELL_COUNT {
                let grid_pos = GridPosition::try_new(x, y);
                if let Some(grid_pos) = grid_pos {
                    if !result.contains(&grid_pos) {
                        let world_pos = grid_to_translation(grid_pos);
                        commands
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(VISIBILITY_DEBUG_SIZE)),
                                    color: DARK_OVERLAY,
                                    ..Default::default()
                                },
                                transform: Transform {
                                    translation: Vec3::new(world_pos.x, world_pos.y, 3.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(Visible);
                    }
                }
                {}
            }
        }
    });
}
