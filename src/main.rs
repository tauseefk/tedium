mod components;
mod debug;
mod events;
mod field_of_view;
mod player_animation;
mod systems;
mod utils;

mod prelude {
    pub use std::ops::Not;

    pub use animation_transition::{AnimationLoop, AnimationTransition, AnimationTransitionMacro};
    pub use bevy::asset::AssetMetaCheck;
    pub use bevy::prelude::*;
    pub use bevy::utils::{HashMap, HashSet};
    pub use bevy_ecs_ldtk::prelude::*;
    pub use pathfinding::prelude::*;

    pub use crate::components::*;
    pub use crate::debug::*;
    pub use crate::events::*;
    pub use crate::field_of_view::*;
    pub use crate::player_animation::*;
    pub use crate::systems::{
        animate_player::*, camera_follow::*, cycle_poi::*, keyboard_events::*, mouse_click::*,
        pathfinding::*, player_move::*, spawn_camera::*, spawn_map::*, visibility::*,
    };
    pub use crate::utils::*;

    #[derive(Resource)]
    pub struct FrameTimer(pub Timer);

    #[derive(Resource)]
    pub struct CycleTimer(pub Timer);

    #[derive(Resource)]
    pub struct MovementTimer(pub Timer);

    pub const TIME_STEP: f32 = 1.0 / 60.0;

    // It's a square grid so rows == col
    pub const GRID_CELL_COUNT: i32 = 32;
    pub const GRID_BLOCK_SIZE: i32 = 16;
    pub const WINDOW_HEIGHT: i32 = GRID_CELL_COUNT * GRID_BLOCK_SIZE;
    pub const WINDOW_WIDTH: i32 = WINDOW_HEIGHT + WINDOW_HEIGHT / 2;

    pub const CAMERA_BB_HEIGHT: f32 = (WINDOW_HEIGHT / 8) as f32;
    pub const CAMERA_BB_WIDTH: f32 = (WINDOW_WIDTH / 8) as f32;
    pub const CAMERA_FOLLOW_SPEED: f32 = 0.05;

    pub const WALLS_LAYER_IDX: i32 = 1;
    pub const POI_CYCLE_INTERVAL: f32 = 8.0;

    pub const MAX_VISIBLE_DISTANCE: i32 = 8;
    pub const LIGHT_INTENSITY: i32 = 8;
    pub const LIGHT_HEIGHT: i32 = 2;

    pub const VISIBILITY_DEBUG_SIZE: f32 = 16.;
    pub const YELLOW: Color = Color::hsl(53.0, 0.99, 0.50);
    pub const PALE: Color = Color::hsla(237.0, 0.45, 0.9, 0.1);
    pub const BLUE_TRANSPARENT: Color = Color::hsla(232.0, 0.62, 0.57, 0.5);
    pub const BLUE: Color = Color::hsl(232.0, 0.62, 0.57);
    pub const WHITE: Color = Color::hsl(0., 0., 1.);
    pub const DARK_OVERLAY: Color = Color::hsla(0., 0., 0., 1.0);
    pub const CLEAR_COLOR: Color = Color::hsl(1., 0., 0.);
}

use prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins
        .set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32).into(),
                title: "Tedium".to_string(),
                ..default()
            }),
            close_when_requested: true,
            ..default()
        }),))
        .add_plugins(LdtkPlugin)
        .insert_resource(DebugConfig {
            light_height: DebugConfigValue {
                min: 1,
                max: 20,
                value: LIGHT_HEIGHT,
            },
            light_intensity: DebugConfigValue {
                min: 0,
                max: 9,
                value: LIGHT_INTENSITY,
            },
        })
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(LevelSelection::index(0))
        .insert_resource(FrameTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert_resource(CycleTimer(Timer::from_seconds(
            POI_CYCLE_INTERVAL,
            TimerMode::Repeating,
        )))
        .insert_resource(MovementTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )))
        .insert_resource(field_of_view::Visibility::new(false, MAX_VISIBLE_DISTANCE))
        .add_event::<UpdateDebugConfigEvent>()
        .add_event::<ToggleWallBlockEvent>()
        .add_event::<PlayerMoveEvent>()
        .add_event::<CyclePOIEvent>()
        .register_ldtk_int_cell::<components::WallBundle>(WALLS_LAYER_IDX)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::ChestBundle>("Chest")
        .add_systems(Startup, (spawn_map_system, spawn_camera_system))
        .add_systems(
            Update,
            (
                cycle_point_of_interest,
                mouse_click,
                visibility_calculation_system,
                pathfinding,
                keyboard_events_system,
                // play_speed,
                player_move,
                camera_follow_system,
                camera_transform_system,
                update_debug_config_system,
                bevy::window::close_when_requested,
            ),
        )
        // .add_systems(FixedUpdate, animate_player);
        .add_systems(FixedUpdate, (path_traversal, animate_player));

    app.run();
}
