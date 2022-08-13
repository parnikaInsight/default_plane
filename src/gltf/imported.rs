use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
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

pub fn create_earth(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut meshes: ResMut<Assets<Mesh>>,) {
    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/space/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
             scale: Vec3::new(5.0, 5.0, 5.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });
    // .insert(RigidBody::Fixed)
    // .insert(Collider::ball(45.0)) //half the cube size
    // .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 1.0)));

    commands.insert_resource(Animations(vec![
        asset_server.load("sketchfab/space/scene.gltf#Animation0")
    ]));
}

pub fn create_city(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut meshes: ResMut<Assets<Mesh>>,) {

    // let player_handle2: Handle<Scene> = asset_server.load("sketchfab/blackhole/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 0.0, 0.0),
    //      //   scale: Vec3::new(0.1, 0.1, 0.1),
    //         ..default()
    //     },
    //     scene: player_handle2.clone(),
    //     ..default()
    // });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/free_spaceship/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(-20.0, -5.0, 0.0),
           // scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/free_spaceship_unitron/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(10.0, 0.0, 0.0),
           // scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/ag_127-720-00/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(30.0, 15.0, -10.0),
           // scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/ajf-12_dvergr/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(25.0, -10.0, 10.0),
            scale: Vec3::new(8.0, 8.0, 8.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/akira/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(-20.0, -15.0, -10.0),
           // scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    // let player_handle2: Handle<Scene> = asset_server.load("sketchfab/andromeda/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(-10.0, 10.0, 0.0),
    //        // scale: Vec3::new(0.5, 0.5, 0.5),
    //         ..default()
    //     },
    //     scene: player_handle2.clone(),
    //     ..default()
    // });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/bg_127-700-00/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(-30.0, 14.0, 10.0),
           // scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/death_row_spaceship/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(-10.0, 10.0, -10.0),
           // scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/defiant/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(-15.0, -1.0, -10.0),
           // scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/free_cyberpunk_hovercar/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(-15.0, -20.0, 10.0),
           // scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/futuristic_freighter_animated/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(10.0, -15.0, 10.0),
           // scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/free_sci_fi_fighter/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(10.0, -10.0, -10.0),
           // scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });



    commands.insert_resource(Animations(vec![
       // asset_server.load("sketchfab/free_spaceship/scene.gltf#Animation0"),
        asset_server.load("sketchfab/blackhole/scene.gltf#Animation0")
    ]));
}