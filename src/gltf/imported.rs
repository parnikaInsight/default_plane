use bevy::prelude::*;

pub struct Animations(Vec<Handle<AnimationClip>>);

// pub fn create_architecture(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let player_handle: Handle<Scene> = asset_server.load("sketchfab/landscape/scene.gltf#Scene0");
//     commands.spawn_bundle(SceneBundle {
//         transform: Transform {
//             translation: Vec3::new(0.0, 0.0, 0.0),
//             ..default()
//         },
//         scene: player_handle.clone(),
//         ..default()
//     });

//     commands.insert_resource(Animations(vec![
//         asset_server.load("sketchfab/landscape/scene.gltf#Animation0"),
//     ]));
// }

pub fn play_scene(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            let time_elapsed = player.play(animations.0[0].clone_weak()).repeat().elapsed();
            println!("time: {}", time_elapsed);
            *done = true;
            println!("Animation");
        }
    }
}
//----------------------------------------------------------------

pub fn create_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_handle3: Handle<Scene> = asset_server.load("terrain/arisaig_ns_part_ii/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 5.0, 12.0),
            ..default()
        },
        scene: player_handle3.clone(),
        ..default()
    });

    commands.insert_resource(Animations(vec![
        asset_server.load("terrain/arisaig_ns_part_ii/scene.gltf#Animation0"),
    ]));
}