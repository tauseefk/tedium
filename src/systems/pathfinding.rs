use crate::prelude::*;

/// Find the shortest path between the player's current position and the active point of interest
pub fn pathfinding(
    player: Query<&Transform, With<Player>>,
    poi_with_transform: Query<(&PointOfInterest, &Transform)>,
    wall_blocks: Query<&Transform, With<Wall>>,
    path_blocks: Query<Entity, With<Path>>,
    mut commands: Commands,
) {
    if player.get_single().is_err() {
        return;
    }

    let world_dimensions = WorldDimensions {
        rows: GRID_CELL_COUNT,
        cols: GRID_CELL_COUNT,
        cell_width: GRID_BLOCK_SIZE,
    };
    let player = player.single();
    let chest = poi_with_transform.iter().find(|(chest, _)| chest.active);

    if chest.is_none() {
        println!("No points of interest found");
        return;
    }

    let (_, c_transform) = chest.unwrap();

    let start_grid_pos = translation_to_grid_pos(player.translation, &world_dimensions).unwrap();
    let end_grid_pos = translation_to_grid_pos(c_transform.translation, &world_dimensions).unwrap();

    let wall_blocks = wall_blocks
        .iter()
        .filter_map(|block| translation_to_grid_pos(block.translation, &world_dimensions))
        .collect::<Vec<_>>();

    let result = bfs(
        &start_grid_pos,
        |p| {
            let &IVec2 { x, y } = p;
            vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
                .into_iter()
                .filter_map(
                    |(x, y)| match is_in_bounds(&IVec2::new(x, y), &world_dimensions) {
                        true => Some(IVec2::new(x, y)),
                        false => None,
                    },
                )
                .filter(|grid_pos| wall_blocks.contains(grid_pos).not())
        },
        |p| *p == end_grid_pos,
    );

    for entity in path_blocks.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if let Some(path) = result {
        for grid_pos in path {
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(4.0, 4.0)),
                        color: BLUE,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: grid_position_to_translation(grid_pos, &world_dimensions),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Path);
        }
    }
}

pub fn path_traversal(
    time: Res<Time>,
    mut movement_timer: ResMut<MovementTimer>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Path>)>,
    path_query: Query<&Transform, (With<Path>, Without<Player>)>,
    mut animation_state: Query<&mut PlayerAnimationState, With<Player>>,
) {
    if player_query.get_single().is_err() {
        return;
    }
    let mut player = player_query.single_mut();
    let mut player_animation_state = animation_state.single_mut();
    let world_dimensions = WorldDimensions {
        rows: GRID_CELL_COUNT,
        cols: GRID_CELL_COUNT,
        cell_width: GRID_BLOCK_SIZE,
    };

    match path_query.iter().nth(1) {
        Some(path_block) => {
            let current_grid_position =
                translation_to_grid_pos(player.translation, &world_dimensions).unwrap();
            let next_grid_position =
                translation_to_grid_pos(path_block.translation, &world_dimensions).unwrap();

            let next_animation_variant = match (
                next_grid_position.x - current_grid_position.x,
                next_grid_position.y - current_grid_position.y,
            ) {
                (-1, 0) => PlayerAnimationVariant::WalkLeft,
                (1, 0) => PlayerAnimationVariant::WalkRight,
                (0, 1) => PlayerAnimationVariant::WalkUp,
                (0, -1) => PlayerAnimationVariant::WalkDown,
                _ => player_animation_state.variant,
            };

            // flip the character sprite horizontally
            if next_animation_variant == PlayerAnimationVariant::WalkRight {
                player.scale.x = player.scale.x.abs();
            } else if next_animation_variant == PlayerAnimationVariant::WalkLeft {
                player.scale.x = -1.0 * player.scale.x.abs();
            }

            player_animation_state.transition_variant(next_animation_variant);

            if movement_timer.0.tick(time.delta()).just_finished() {
                player.translation.x = path_block.translation.x;
                player.translation.y = path_block.translation.y;
            }
        }
        None => {
            if player_animation_state.variant != PlayerAnimationVariant::Idle {
                player_animation_state.transition_variant(PlayerAnimationVariant::Idle);
            }
        }
    }
}
