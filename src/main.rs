use bevy::prelude::*;
use dolly::prelude::*;

mod camera;
mod math;
mod pcg_city;

fn main() {
    App::new()
        //Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        //Resources
        .init_resource::<math::city_perlin::HeightNoiseFn>()
        .init_resource::<camera::dolly_free::MyCameraRig>()
        //Startup system
        //.add_startup_system(camera::pan_orbit::spawn_camera)
        .add_startup_system(setup)
        //Systems
        //.add_system(camera::camera_controller::camera_controller)
        //.add_system(camera::pan_orbit::pan_orbit_camera)
        .add_system(camera::dolly_free::camera_move_free)
        .add_system(pcg_city::buildings::spawn_buildings)
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
        mesh: meshes.add(Mesh::from(shape::Plane { size: 15.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
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

    // camera
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
        //.insert_bundle((camera::camera_controller::CameraController::default(),));

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
        transform: Transform::from_xyz(0 as f32, 0 as f32, 0 as f32),
        ..default()
    });
}
