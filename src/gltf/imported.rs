use bevy::prelude::*;

pub fn create_sponza(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let player_handle = asset_server.load("kronos/glTF/Sponza.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            scene: player_handle.clone(),
            ..default()
        });
    println!("sponza");
}
