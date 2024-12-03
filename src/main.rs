mod components;
mod events;
mod field_of_view;
mod player_animation;
mod systems;
mod utils;

mod prelude {
    pub use std::ops::Not;

    pub use animation_transition::{AnimationLoop, AnimationTransition, AnimationTransitionMacro};
    pub use bevy::prelude::*;
    pub use bevy::utils::HashSet;
    pub use bevy::window::PrimaryWindow;
    pub use bevy_ecs_ldtk::prelude::*;
    pub use pathfinding::prelude::*;

    pub use crate::components::*;
    pub use crate::events::*;
    pub use crate::field_of_view::*;
    pub use crate::player_animation::*;
    pub use crate::systems::{
        animate_player::*, arrow_keys::*, cycle_poi::*, mouse_click::*, pathfinding::*,
        player_move::*, setup::*, visibility::*,
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
    pub const GRID_CELL_COUNT: i32 = 16;
    pub const GRID_BLOCK_SIZE: i32 = 16;
    pub const WINDOW_HEIGHT: i32 = 256;
    pub const WINDOW_WIDTH: i32 = 256;

    pub const MAX_VISIBLE_DISTANCE: i32 = 8;

    pub const VISIBILITY_DEBUG_SIZE: f32 = 16.;
    pub const YELLOW: Color = Color::hsl(53.0, 0.99, 0.50);
    pub const PALE: Color = Color::hsla(237.0, 0.45, 0.9, 0.1);
    pub const BLUE_TRANSPARENT: Color = Color::hsla(232.0, 0.62, 0.57, 0.5);
    pub const BLUE: Color = Color::hsl(232.0, 0.62, 0.57);
    pub const WHITE: Color = Color::hsl(0., 0., 1.);
    pub const WALKABLE_INT_GRID_VALUE: i32 = 2;
    pub const DARK_OVERLAY: Color = Color::hsla(0., 0., 0., 0.9);
}

use prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins
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
        .insert_resource(LevelSelection::index(0))
        .insert_resource(FrameTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert_resource(CycleTimer(Timer::from_seconds(8.0, TimerMode::Repeating)))
        .insert_resource(MovementTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .insert_resource(field_of_view::Visibility::new(false, MAX_VISIBLE_DISTANCE))
        .add_event::<ToggleWallBlockEvent>()
        .add_event::<PlayerMoveEvent>()
        .add_event::<CyclePOIEvent>()
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::ChestBundle>("Chest")
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                cycle_point_of_interest,
                mouse_click,
                visibility_calc,
                /* pathfinding, */
                arrow_keys,
                player_move,
                bevy::window::close_when_requested,
            ),
        )
        // .add_system(play_speed)
        // .add_system(mouse_click)
        // .add_system(toggle_wall)
        // .add_system(pathfinding)
        .add_systems(FixedUpdate, (path_traversal, animate_player));

    app.run();
}
