//uses bevy 0.7

use crate::{
    math::{city_perlin, grid, random},
};
use crate::{
    camera::{dolly_free},
};
use bevy::{prelude::*};
use bevy_dolly::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use std::{thread, time};

fn convert(x: f32) -> i32 {
    x as i32
}

pub fn spawn_buildings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: ParamSet<(
        Query<(&mut Transform, With<dolly_free::MainCamera>)>,
        Query<&mut CameraRig>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    //instantiate height noise function
    let height_noise_fn = city_perlin::HeightNoiseFn::default();

   // if let Ok((mut transform, mut camera)) = query.p0().get_single_mut() {
    if let Ok((mut rig)) = query.p1().get_single_mut() {
        let transform = rig.final_transform;

        let mut vector: Vec3 = transform.translation;
        let x = vector.x;
        let y = vector.y;
        let z = vector.z;
        println!("location: x = {}, y = {}, z = {}", x, y, z);        

        // vector = transform.rotation.mul_vec3a(bevy::math::Vec3A::from((x, y, z))).into();
        // println!("rotated vector: x = {}, y = {}, z = {}", vector.x, vector.y, vector.z);   

        //vector += Vec3::new(0.0, 0.0, z + vector.z*5.0);
        // if vector.z > 0.0 {
        //     vector += Vec3::new(0.0, 0.0, z+5.0);
        // }
        // else{
        //     vector += Vec3::new(0.0, 0.0, z-5.0); 
        // }

        vector = transform.rotation.mul_vec3a(bevy::math::Vec3A::from((vector.x, vector.y, vector.z))).into();
        vector += Vec3::new(0.0, 0.0, z - 10.0);
        //vector *= Vec3::new(0.5, 0.5, 0.5);

        let x_right_bound: i32 = convert(vector.x + 2.0);
        let x_left_bound: i32 = convert(vector.x - 1.0);

        for i in x_left_bound..x_right_bound {
            let coord = grid::Coordinate {
                x: i as f64,
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
                transform: Transform::from_xyz(coord.x as f32, 0.0 as f32, coord.z as f32),
                ..default()
            });
        }
    }
}
