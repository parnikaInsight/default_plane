use bevy::prelude::*;

pub fn create_sponza(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //let player_handle = asset_server.load("kronos/glTF/Sponza.gltf#Scene0");
    //let player_handle = asset_server.load("sketchfab/phoenix_bird/scene.gltf#Scene0");
    let player_handle = asset_server.load("sketchfab/grass/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
            transform: Transform {
                translation: Vec3::new(-10.0, 0.0, 0.0),
                ..default()
            },
            scene: player_handle.clone(),
            ..default()
        });
    println!("bird");
}
