use crate::prelude::*;

// Initial setup, spawns the camera, and LDTK World
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new((WINDOW_WIDTH / 2) as f32, (WINDOW_HEIGHT / 2) as f32, 3.),
            ..Default::default()
        },
        ..Default::default()
    });

    let ldtk_handle = asset_server.load("basic_map.ldtk");
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}

/// Grid coordinates to world coordinates
fn grid_to_translation(grid_pos: GridPosition) -> Vec3 {
    Vec3::new(
        (grid_pos.x as i32 * GRID_BLOCK_SIZE - GRID_BLOCK_SIZE / 2) as f32,
        (grid_pos.y as i32 * GRID_BLOCK_SIZE - GRID_BLOCK_SIZE / 2) as f32,
        2.,
    )
}

/// World coordinates to grid coordinates
fn translation_to_grid_pos(translation: Vec3) -> Option<GridPosition> {
    let x = (translation.x as i32) / GRID_BLOCK_SIZE + 1;
    let y = (translation.y as i32) / GRID_BLOCK_SIZE + 1;

    GridPosition::try_new(x, y)
}

/// Snaps arbitrary coordinates to align with the in-game grid
fn snap_to_grid(translation: Vec3) -> Vec3 {
    grid_to_translation(translation_to_grid_pos(translation).unwrap())
}

/// Listens to mouse events and triggers appropriate in game events
pub fn mouse_click(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut toggle_wall: EventWriter<ToggleWallBlockEvent>,
    mut cycle_point_of_interest: EventWriter<CyclePOIEvent>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(window) = windows.get_primary() {
            if let Some(cursor_pos) = window.cursor_position() {
                toggle_wall.send(ToggleWallBlockEvent {
                    translation: snap_to_grid(Vec3::new(cursor_pos.x, cursor_pos.y, 1.)),
                });
            }
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        cycle_point_of_interest.send(CyclePOIEvent {});
    }
}

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

/// Cycle through points of interest in order
pub fn cycle_point_of_interest(
    time: Res<Time>,
    mut cycle_timer: ResMut<CycleTimer>,
    mut my_events: EventReader<CyclePOIEvent>,
    mut poi_query: Query<&mut PointOfInterest>,
) {
    for _ in my_events.iter() {
        let active_idx = poi_query
            .iter()
            .enumerate()
            .find_map(|(idx, point_of_interest)| {
                if point_of_interest.active {
                    Some(idx)
                } else {
                    None
                }
            });

        if let Some(active_idx) = active_idx {
            let mut active_poi = poi_query.iter_mut().nth(active_idx).unwrap();
            active_poi.active = false;

            let next_idx = (active_idx + 1) % poi_query.iter().len();
            let mut next_poi = poi_query.iter_mut().nth(next_idx).unwrap();
            next_poi.active = true;
        }
    }

    cycle_timer.0.tick(time.delta());
    if cycle_timer.0.finished() {
        let active_idx = poi_query
            .iter()
            .enumerate()
            .find_map(|(idx, point_of_interest)| {
                if point_of_interest.active {
                    Some(idx)
                } else {
                    None
                }
            });

        if let Some(active_idx) = active_idx {
            let mut active_poi = poi_query.iter_mut().nth(active_idx).unwrap();
            active_poi.active = false;

            let next_idx = (active_idx + 1) % poi_query.iter().len();
            let mut next_poi = poi_query.iter_mut().nth(next_idx).unwrap();
            next_poi.active = true;
        }
    }
}

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

    let blocks = wall_blocks
        .iter()
        .map(|block| translation_to_grid_pos(block.translation).unwrap())
        .collect::<Vec<_>>();

    let result = bfs(
        &start_grid_pos,
        |p| {
            let &GridPosition { x, y } = p;
            vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
                .into_iter()
                .filter_map(|(x, y)| GridPosition::try_new(x, y))
                .filter(|grid_pos| blocks.contains(&grid_pos).not())
        },
        |p| *p == end_grid_pos,
    );

    for entity in path_blocks.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if let Some(path) = result {
        for grid_pos in path {
            commands
                .spawn_bundle(SpriteBundle {
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

pub fn visibility_calc(
    visibile_blocks: Query<Entity, With<Visible>>,
    mut visibility: ResMut<crate::field_of_view::Visibility>,
    mut commands: Commands,
) {
    #[rustfmt::skip]
        let tiles = vec![
            '_', '_', '_', '_', '_', '_', '_', '_',
            '_', '_', '_', '_', '_', '_', '_', '_',
            '_', '_', '_', '_', '_', '_', '_', '_',
            '_', '_', '_', '_', '_', '_', '_', '_',
            '_', '_', '_', '_', '_', '_', '_', '_',
            '_', 'o', 'o', 'o', '_', '_', '_', '_',
            '_', 'o', 'o', 'o', '_', '_', '_', '_',
            '_', 'o', 'o', 'o', '_', '_', '_', '_'
        ];

    let tiles = tiles.iter().map(|value| value.into()).collect();

    let world = crate::field_of_view::World {
        tiles,
        width: 8,
        height: 8,
    };

    for entity in visibile_blocks.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let result = visibility.compute_visible_tiles(&world);
    for grid_pos in result {
        let grid_pos = GridPosition::try_new(grid_pos.x, grid_pos.y);
        if let Some(grid_pos) = grid_pos {
            let grid_pos = grid_to_translation(grid_pos);
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(
                            GRID_BLOCK_SIZE as f32,
                            GRID_BLOCK_SIZE as f32,
                        )),
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

pub fn animate_player(
    time: Res<Time>,
    mut frame_timer: ResMut<FrameTimer>,
    mut animation_state_with_texture_query: Query<
        (&mut PlayerAnimationState, &mut TextureAtlasSprite),
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

pub fn play_speed(
    keyboard_input: Res<Input<KeyCode>>,
    mut play_speed_query: Query<&mut PlaySpeed>,
) {
    let mut play_speed = play_speed_query.single_mut();

    if keyboard_input.pressed(KeyCode::P) {
        play_speed.multiplier = 2.0;
    }
    if keyboard_input.pressed(KeyCode::O) {
        play_speed.multiplier = 1.0;
    }
}
