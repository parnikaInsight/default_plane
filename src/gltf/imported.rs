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
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(40.0, 0.5, 60.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(20.0, 0.5, 60.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    // Light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        )),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });


    //bird
    commands.insert_resource(Animations(vec![
        asset_server.load("sketchfab/phoenix_bird/scene.gltf#Animation0"),
        asset_server.load("sketchfab/grass2/scene.gltf#Animation0")
    ]));

    //let player_handle = asset_server.load("kronos/glTF/Sponza.gltf#Scene0");
    //let player_handle = asset_server.load("sketchfab/phoenix_bird/scene.gltf#Scene0");
    //let player_handle = asset_server.load("sketchfab/wanderers/scene.gltf#Scene0");
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
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(40.0, 0.0, -20.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(40.0, 0.0, 5.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(40.0, 0.0, 5.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(40.0, 0.0, 30.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(40.0, 0.0, 30.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(20.0, -5.0, 40.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(20.0, -5.0, 40.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });
    commands.insert_resource(Animations(vec![
        asset_server.load("sketchfab/ocean/scene.gltf#Animation0"),
    ]));
}

pub fn create_rocks(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/rock_layering_1/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -30.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    commands.insert_resource(Animations(vec![
        asset_server.load("sketchfab/rock_layering_1/scene.gltf#Animation0"),
    ]));
}

pub fn create_architecture(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_handle: Handle<Scene> = asset_server.load("sketchfab/landscape/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        scene: player_handle.clone(),
        ..default()
    });

    // let player_handle2: Handle<Scene> = asset_server.load("sketchfab/northumberlandia_land_sculpture/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 0.0, 0.0),
    //         ..default()
    //     },
    //     scene: player_handle2.clone(),
    //     ..default()
    // });

    commands.insert_resource(Animations(vec![
        asset_server.load("sketchfab/landscape/scene.gltf#Animation0"),
        asset_server.load("sketchfab/northumberlandia_land_sculpture/scene.gltf#Animation0"),
    ]));
}

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
