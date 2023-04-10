mod components;
mod events;
mod field_of_view;
mod player_animation;
mod systems;
mod utils;

mod prelude {
    pub use std::ops::Not;

    pub use animation_transition::*;
    pub use bevy::utils::HashSet;
    pub use bevy::{prelude::*, time::FixedTimestep};
    pub use bevy_ecs_ldtk::prelude::*;
    pub use bevy_ecs_ldtk::utils::sprite_sheet_bundle_from_entity_info;
    pub use pathfinding::prelude::*;

    pub use crate::components::*;
    pub use crate::events::*;
    pub use crate::field_of_view::*;
    pub use crate::player_animation::*;
    pub use crate::systems::*;
    pub use crate::utils::*;

    // TODO: use for animation frames
    pub struct FrameTimer(pub Timer);
    pub struct CycleTimer(pub Timer);
    pub struct MovementTimer(pub Timer);

    pub const TIME_STEP: f32 = 1.0 / 60.0;

    pub const GRID_SIZE: i32 = 8;
    pub const GRID_BLOCK_SIZE: i32 = 32;
    pub const WINDOW_HEIGHT: i32 = 256;
    pub const WINDOW_WIDTH: i32 = 256;

    pub const YELLOW: Color = Color::hsl(53.0, 0.99, 0.50);
    pub const PALE: Color = Color::hsl(237.0, 0.45, 0.9);
    pub const BLUE_TRANSPARENT: Color = Color::hsla(232.0, 0.62, 0.57, 0.5);
    pub const BLUE: Color = Color::hsl(232.0, 0.62, 0.57);
    pub const WHITE: Color = Color::hsl(0., 0., 1.);
    pub const BLACK: Color = Color::hsl(0., 0., 0.);
}

use prelude::*;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::GRAY))
        .insert_resource(WindowDescriptor {
            title: "Pathfinder".to_string(),
            width: (WINDOW_WIDTH) as f32,
            height: (WINDOW_HEIGHT) as f32,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(FrameTimer(Timer::from_seconds(0.1, true)))
        .insert_resource(CycleTimer(Timer::from_seconds(8.0, true)))
        .insert_resource(MovementTimer(Timer::from_seconds(0.4, true)))
        .insert_resource(field_of_view::Visibility::new(false, 5))
        .add_event::<ToggleWallBlockEvent>()
        .add_event::<CyclePOIEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::ChestBundle>("Chest")
        // .add_system(play_speed)
        .add_system(mouse_click)
        .add_system(cycle_point_of_interest)
        // .add_system(toggle_wall)
        .add_system(pathfinding)
        .add_system(visibility_calc)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(path_traversal)
                .with_system(animate_player),
        )
        .add_system(bevy::window::close_on_esc);

    app.run();
}
