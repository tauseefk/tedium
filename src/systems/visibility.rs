use crate::prelude::*;

pub fn visibility_calc(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    visibile_blocks: Query<Entity, With<Visible>>,
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

        // // the grid is inverted on the x axis
        // #[rustfmt::skip]
        // let debug_tiles = vec![
        //     '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
        //     'o', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
        //     'o', 'o', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
        //     'o', 'o', 'o', '_', '_', '_', '_', 'o', '_', '_', '_', '_', '_', '_', '_', '_',
        //     'o', 'o', 'o', 'o', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
        //     'o', 'o', 'o', 'o', 'o', 'o', '_', '_', '_', 'o', '_', '_', '_', '_', '_', '_',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', '_', '_', 'o', '_', '_', '_', '_', '_', '_',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', '_', '_', 'o', '_', '_', '_', '_', '_', '_',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', '_', '_', '_', '_', '_', '_', '_', '_',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', '_', '_', '_', '_', '_', '_', '_',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', '_', '_', '_', '_', '_', '_',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', '_', '_', '_', '_', '_',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', '_', '_', '_', '_',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', '_', '_', '_',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', '_', '_',
        //     'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', '_'
        // ];

        // let tiles: Vec<TileType> = debug_tiles.iter().map(|value| value.into()).collect();

        let tiles: Vec<TileType> = layer_instance
            .int_grid_csv
            .iter()
            .map(|int_grid_tile_value| match int_grid_tile_value {
                1 => TileType::Opaque,
                2 => TileType::Transparent,
                _ => TileType::Opaque,
            })
            .collect();

        let world = crate::field_of_view::World {
            tiles,
            width: 16,
            height: 16,
        };

        for entity in visibile_blocks.iter() {
            visibility.drain_visible_tiles();
            commands.entity(entity).despawn_recursive();
        }

        let result = visibility.compute_visible_tiles(&world);

        for grid_pos in result {
            // TOOD: fix hardcoded offset for x axis
            let grid_pos = GridPosition::try_new(grid_pos.x, grid_pos.y);
            if let Some(grid_pos) = grid_pos {
                let grid_pos = grid_to_translation(grid_pos);
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(24.)),
                            color: BLUE_TRANSPARENT,
                            ..Default::default()
                        },
                        transform: Transform {
                            translation: Vec3::new(grid_pos.x, grid_pos.y, 3.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Visible);
            }
        }
    });
}
