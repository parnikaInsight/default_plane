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

pub fn create_default_plane(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
){
    let player_handle2: Handle<Scene> = asset_server.load("default/sci_fi_level_design/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            //scale: Vec3::new(5.0, 5.0, 5.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    // commands.insert_resource(Animations(vec![
    //     asset_server.load("default/level_02_-_vr_platform_game_on_sketchfab/scene.gltf#Animation0")
    // ]));
}


pub fn create_world(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/uxrzone_circle_floor_nav/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(5.0, 5.0, 5.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/uxrzone_circle_floor_nav/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            scale: Vec3::new(5.0, 5.0, 5.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/uxrzone_circle_floor_nav/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            scale: Vec3::new(5.0, 5.0, 5.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });





    //center pyramid
    // let player_handle2: Handle<Scene> = asset_server.load("sketchfab/fwd_-_forward_-_pyramid_block/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 0.0, 0.0),
    //         ..default()
    //     },
    //     scene: player_handle2.clone(),
    //     ..default()
    // });

     //circle around pyramid
    //  let player_handle2: Handle<Scene> = asset_server.load("sketchfab/uxrzone_circle_floor_nav/scene.gltf#Scene0");
    //  commands.spawn_bundle(SceneBundle {
    //      transform: Transform {
    //          translation: Vec3::new(0.0, -3.0, 0.0),
    //          scale: Vec3::new(50.0, 50.0, 50.0),
    //          ..default()
    //      },
    //      scene: player_handle2.clone(),
    //      ..default()
    //  });

    //stairs to pyramid
    // let player_handle2: Handle<Scene> = asset_server.load("sketchfab/uxr-2/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(2.0, 0.0, -1.0),
    //         //scale: Vec3::new(0.01, 0.01, 0.01),
    //         ..default()
    //     },
    //     scene: player_handle2.clone(),
    //     ..default()
    // });

    //arora
    // let player_handle2: Handle<Scene> = asset_server.load("sketchfab/aurora/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 150.0, 0.0),
    //         scale: Vec3::new(10.0, 10.0, 10.0),
    //         ..default()
    //     },
    //     scene: player_handle2.clone(),
    //     ..default()
    // });

    //surrounding wall
    // let player_handle2: Handle<Scene> = asset_server.load("sketchfab/thedome_se02_circular_columns/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 0.0, 0.0),
    //         ..default()
    //     },
    //     scene: player_handle2.clone(),
    //     ..default()
    // });

    // //magical ball
    // let player_handle2: Handle<Scene> = asset_server.load("sketchfab/enter_the_uxrzone/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 100.0, 0.0),
    //         ..default()
    //     },
    //     scene: player_handle2.clone(),
    //     ..default()
    // });
    
    //center place
    // let player_handle2: Handle<Scene> = asset_server.load("sketchfab/circular_hub_structure/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 20.0, 0.0),
    //         ..default()
    //     },
    //     scene: player_handle2.clone(),
    //     ..default()
    // });

    //fractal tower
    // let player_handle2: Handle<Scene> = asset_server.load("sketchfab/uxr_city_fractal_tower/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 0.0, 0.0),
    //         scale: Vec3::new(10.0, 10.0, 10.0),
    //         ..default()
    //     },
    //     scene: player_handle2.clone(),
    //     ..default()
    // });


    //circle base at end of stairs
    // let player_handle2: Handle<Scene> = asset_server.load("sketchfab/uxrzone_the_circle_base/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(190.0, 4.0, 0.0),
    //         scale: Vec3::new(0.5, 0.5, 0.5),
    //         ..default()
    //     },
    //     scene: player_handle2.clone(),
    //     ..default()
    // });

    //mole in circle base at end of stairs
    // let player_handle2: Handle<Scene> = asset_server.load("sketchfab/mole_antonelliana_-_uxzone/scene.gltf#Scene0");
    // commands.spawn_bundle(SceneBundle {
    //     transform: Transform {
    //         translation: Vec3::new(190.0, 4.0, 0.0),
    //         scale: Vec3::new(0.05, 0.05, 0.05),
    //         ..default()
    //     },
    //     scene: player_handle2.clone(),
    //     ..default()
    // });
}