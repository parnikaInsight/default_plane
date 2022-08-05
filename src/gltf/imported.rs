use bevy::prelude::*;

pub struct Animations(Vec<Handle<AnimationClip>>);

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

pub fn create_space(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_handle1: Handle<Scene> = asset_server.load("sketchfab/space/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 5.0, 12.0),
            scale: Vec3::new(20.0, 20.0, 20.0),
            ..default()
        },
        scene: player_handle1.clone(),
        ..default()
    });

    let player_handle4: Handle<Scene> = asset_server.load("sketchfab/fake_space/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(-100.0, 100.0, 25.0),
            scale: Vec3::new(0.01, 0.01, 0.01),
            ..default()
        },
        scene: player_handle4.clone(),
        ..default()
    });

    let player_handle5: Handle<Scene> = asset_server.load("sketchfab/inside_the_space_jellyfish/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(100.0, 80.0, 45.0),
           // scale: Vec3::new(0.1, 0.1, 0.1),
            ..default()
        },
        scene: player_handle5.clone(),
        ..default()
    });

    // let player_handle6: Handle<Scene> = asset_server.load("sketchfab/timetunnel/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(100.0, -20.0, 35.0),
    //        // scale: Vec3::new(0.1, 0.1, 0.1),
    //         ..default()
    //     },
    //     scene: player_handle6.clone(),
    //     ..default()
    // });

    // let player_handle7: Handle<Scene> = asset_server.load("sketchfab/fibonacci_sphere/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(-100.0, -100.0, 35.0),
    //        // scale: Vec3::new(2.0, 2.0, 2.0),
    //         ..default()
    //     },
    //     scene: player_handle7.clone(),
    //     ..default()
    // });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/heaven/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 5.0, 12.0),
           // scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    let player_handle8: Handle<Scene> = asset_server.load("sketchfab/temple2/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(-30.0, -5.0, 20.0),
            scale: Vec3::new(0.1, 0.1, 0.1),
            ..default()
        },
        scene: player_handle8.clone(),
        ..default()
    });

    let player_handle8: Handle<Scene> = asset_server.load("sketchfab/temple3/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(30.0, 15.0, 12.0),
            scale: Vec3::new(0.1, 0.1, 0.1),
            ..default()
        },
        scene: player_handle8.clone(),
        ..default()
    });

    let player_handle3: Handle<Scene> = asset_server.load("sketchfab/temple/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 5.0, 5.0),
            scale: Vec3::new(0.1, 0.1, 0.1),
            ..default()
        },
        scene: player_handle3.clone(),
        ..default()
    });


    commands.insert_resource(Animations(vec![
        asset_server.load("sketchfab/heaven/scene.gltf#Animation0"),
        asset_server.load("sketchfab/space/scene.gltf#Animation0"),
        asset_server.load("sketchfab/temple/scene.gltf#Animation0"),
    ]));

}