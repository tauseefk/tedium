use crate::prelude::*;

// Initial setup, spawns the camera, and LDTK World
pub fn spawn_map_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ldtk_handle = asset_server.load("maps/basic_map.ldtk");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}
