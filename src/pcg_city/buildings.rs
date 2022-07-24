use crate::{
    camera::{pan_orbit::PanOrbitCamera, camera_controller::{CameraController, self}},
    math::{city_perlin, grid, random},
};
use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use std::{thread, time};

fn get_camera(world: &mut World) -> Vec3 {
    let camera_focus_vector: Vec3 = world.get_resource_mut::<PanOrbitCamera>().unwrap().focus;
    camera_focus_vector
}

pub fn spawn_buildings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
    mut query: Query<(&mut Transform, &mut Camera, &mut CameraController)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let delay = time::Duration::from_secs(3);

    //instantiate height noise function
    let height_noise_fn = city_perlin::HeightNoiseFn::default();

    if let Ok((mut transform, mut camera, mut options)) = query.get_single_mut() {
        // let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        // let coord = grid::Coordinate {
        //     x: yaw as f64,
        //     y: pitch as f64,
        //     z: roll as f64,
        // };
        let vector: Vec3 = transform.translation;

        //let vector: Vec3 = transform.forward();
        let coord = grid::Coordinate {
            x: vector.x as f64,
            y: vector.y as f64,
            z: vector.z as f64,
            //z: (vector.z + 15.0) as f64,
        };

        println!("{:?}", coord);

        let height = height_noise_fn.function.get([coord.x, coord.z]);
        let dimension = random::Random {
            number: (height + 1.0),
        };

        // cube
        let size: f32 = dimension.number as f32;
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: size })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            //transform: Transform::from_xyz(location.x * 14.0, 0.0, 0.0),
            transform: Transform::from_xyz((coord.x * 10.0) as f32, 0.0 as f32, coord.z as f32),
            ..default()
        });
        println!("created cube {}", size);
    }
    //let camera_focus_vector: Vec3 = query.get_mut(entity).focus;

    // let coord = grid::Coordinate{
    //     x: camera_focus_vector.x as f64,
    //     y: camera_focus_vector.y as f64,
    //     z: camera_focus_vector.z as f64,
    // };

    // let coord = grid::Coordinate{
    //     x: 90.5,
    //     y: 0.0,
    //     z: 432.8,
    // };

    //get block dimensions
    // let height = height_noise_fn.function.get([coord.x, coord.z]);
    // let dimension = random::Random{
    //     number: height * 10.0,
    // };
    // println!("Height: {}", height);
    // let width = height_noise_fn.function.get([90.5, 432.8]);
    // println!("Width: {}", width);
    // let hey = height_noise_fn.function.get([980.5, 42.8]);
    // println!("Hey: {}", hey);

    //get block location - Coordinate
    // let loc = random::get_block_location();
    // let location = random::Random{
    //     x: loc,
    // };

    // // cube
    // let size: f32 = dimension.number as f32;
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: size})),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     //transform: Transform::from_xyz(location.x * 14.0, 0.0, 0.0),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });
    //thread::sleep(delay);
}
