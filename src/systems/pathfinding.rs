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

    let player = player.single();
    let chest = poi_with_transform.iter().find(|(chest, _)| chest.active);

    if chest.is_none() {
        println!("No points of interest found");
        return;
    }

    let (_, c_transform) = chest.unwrap();

    let start_grid_pos = translation_to_grid_pos(player.translation).unwrap();
    let end_grid_pos = translation_to_grid_pos(c_transform.translation).unwrap();

    let wall_blocks = wall_blocks
        .iter()
        .map(|block| translation_to_grid_pos(block.translation).unwrap())
        .collect::<Vec<_>>();

    // let row_length = rows_y_flipped[0].len();
    // let x_translated_rows = rows_y_flipped
    //     .iter()
    //     .map(|row| {
    //         let mut new_row = Vec::with_capacity(row_length);

    //         // Add zero at the beginning
    //         new_row.push(WALKABLE_INT_GRID_VALUE);

    //         new_row.extend_from_slice(&row[0..row.len() - 1]);
    //         new_row
    //     })
    //     .collect();

    // let zero_row = vec![WALKABLE_INT_GRID_VALUE; row_length];
    // let xy_translated_rows = [vec![zero_row], x_translated_rows].concat();
    // let xy_translated_rows = xy_translated_rows[0..xy_translated_rows.len() - 1].iter();

    let result = bfs(
        &start_grid_pos,
        |p| {
            let &GridPosition { x, y } = p;
            vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
                .into_iter()
                .filter_map(|(x, y)| GridPosition::try_new(x, y))
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
                        translation: grid_to_translation(grid_pos),
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

    match path_query.iter().nth(1) {
        Some(path_block) => {
            let current_grid_position = translation_to_grid_pos(player.translation).unwrap();
            let next_grid_position = translation_to_grid_pos(path_block.translation).unwrap();

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
