use crate::prelude::*;

// Initial setup, spawns the camera, and LDTK World
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2dBundle {
        transform: Transform {
            translation: Vec3::new((WINDOW_WIDTH / 2) as f32, (WINDOW_HEIGHT / 2) as f32, 0.),
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn(camera);

    let ldtk_handle = asset_server.load("maps/basic_map.ldtk");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::linear_rgb(0., 0., 255.0),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
