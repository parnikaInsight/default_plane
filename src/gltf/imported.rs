use bevy::prelude::*;

pub struct Animations(Vec<Handle<AnimationClip>>);

pub fn create_land(mut commands: Commands, asset_server: Res<AssetServer>) {
    //bright green grass
    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/grass2/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(25.0, 0.5, 60.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    commands.insert_resource(Animations(vec![
        asset_server.load("sketchfab/grass2/scene.gltf#Animation0")
    ]));

    //let player_handle = asset_server.load("kronos/glTF/Sponza.gltf#Scene0");
}

pub fn create_water(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/ocean/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(40.0, 0.0, -20.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });
    
    commands.insert_resource(Animations(vec![
        asset_server.load("sketchfab/ocean/scene.gltf#Animation0"),
    ]));
}


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