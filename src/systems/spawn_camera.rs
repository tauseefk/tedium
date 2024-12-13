use crate::prelude::*;

pub fn spawn_camera_system(mut commands: Commands) {
    let camera_transform = Transform {
        // this spawns the camera at the center of the LDTK map
        translation: bevy::prelude::Vec3::new(
            (WINDOW_WIDTH / 2) as f32,
            (WINDOW_HEIGHT / 2) as f32,
            1.,
        ),
        ..Default::default()
    };

    let mut camera = Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(CLEAR_COLOR),
            ..Default::default()
        },
        transform: camera_transform,
        ..Default::default()
    };

    // ensure only the character visible parts are visible to the user
    camera.projection.scale = 0.5;

    camera.transform.translation.x = camera_transform.translation.x;
    camera.transform.translation.y = camera_transform.translation.y;

    commands.spawn(camera).insert(IntermediateCamera {
        transform: Transform {
            translation: camera_transform.translation.clone(),
            ..Default::default()
        },
    });
}
