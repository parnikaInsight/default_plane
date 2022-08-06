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
            // scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::ball(45.0)) //half the cube size
    .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 1.0)));

    commands.insert_resource(Animations(vec![
        asset_server.load("sketchfab/space/scene.gltf#Animation0")
    ]));
}

pub fn create_city(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut meshes: ResMut<Assets<Mesh>>,) {

    let player_handle2: Handle<Scene> = asset_server.load("sketchfab/free_spaceship_unitron/scene.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(0.1, 0.1, 0.1),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    commands.insert_resource(Animations(vec![
       // asset_server.load("sketchfab/free_spaceship/scene.gltf#Animation0"),
        asset_server.load("sketchfab/free_spaceship_unitron/scene.gltf#Animation0")
    ]));
}