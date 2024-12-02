use crate::prelude::*;

// Initial setup, spawns the camera, and LDTK World
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2dBundle::default();

    commands.spawn(camera);

    let ldtk_handle = asset_server.load("basic_map.ldtk");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}
