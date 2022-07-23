use bevy::prelude::*;

mod camera;
use camera::pan_orbit;

mod pcg_city;
use pcg_city::buildings;

mod math;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_startup_system(pan_orbit::spawn_camera)
        .add_startup_system(setup)
        .add_system(pan_orbit::pan_orbit_camera)
        .add_system(buildings::spawn_buildings)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // // cube
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}