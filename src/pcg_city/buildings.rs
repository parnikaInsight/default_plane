//uses bevy 0.7

use crate::{
    math::{city_perlin, grid, random},
};
use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use std::{thread, time};

pub fn spawn_buildings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Transform, &mut Camera)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let delay = time::Duration::from_secs(3);

    //instantiate height noise function
    let height_noise_fn = city_perlin::HeightNoiseFn::default();

    if let Ok((mut transform, mut camera)) = query.get_single_mut() {
        //let vector: Vec3 = transform.forward();
        let vector: Vec3 = transform.translation;
        let coord = grid::Coordinate {
            x: vector.x as f64,
            y: vector.y as f64,
            z: vector.z as f64,
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
            //transform: Transform::from_xyz((coord.x * 10.0) as f32, 0.0 as f32, (coord.z * 10.0) as f32),
            transform: Transform::from_xyz((coord.x * 10.0) as f32, 0.0 as f32, (coord.z - 10.0) as f32),
            ..default()
        });
        println!("created cube {}", size);
    }
}
