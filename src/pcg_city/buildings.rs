//uses bevy 0.7

use crate::camera::dolly_free;
use crate::math::grid::MyGrid;
use crate::math::random::{get_block_location, get_block_dimensions};
use crate::math::{city_perlin, grid, random};
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use std::{thread, time};

fn convert(x: f32) -> i32 {
    x as i32
}

pub fn spawn_buildings(
    mut commands: Commands,
    mut grid: ResMut<MyGrid>,
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
    if let Ok(mut rig) = query.p1().get_single_mut() {
        let transform = rig.final_transform;

        let vector: Vec3 = transform.translation;

        let x_right_bound: i32 = convert(vector.x + 2.0);
        let x_left_bound: i32 = convert(vector.x - 2.0);

        let z_front_bound: i32 = convert(vector.z + 2.0);
        let z_back_bound: i32 = convert(vector.z - 2.0);
        let mut count = 0;
        for x_iter in x_left_bound..x_right_bound {
            for z_iter in z_back_bound..z_front_bound {
                count += 1;
                let x: f32 = x_iter as f32;
                let z = z_iter as f32;
                let cam_vec = Vec2::new(vector.x, vector.z);
                let pot_cube_vec = Vec2::new(x, z);
                if cam_vec.distance(pot_cube_vec) <= 2.0 {
                    if let Some(b) = grid.coordinate_system.get_mut(&vec![x_iter, z_iter]){
                        if *b == false {
                            *b = true;
                            let coord = grid::Coordinate {
                                x: x as f64,
                                y: 0.0 as f64,
                                z: z as f64,
                            };
        
                            println!("{:?}", coord);
        
                            let height = height_noise_fn.function.get([coord.x / 7.0, coord.z / 7.0]);
                            let dimension = random::Random {
                                number: height + 1.0,
                            };
        
                            // cube
                            let size: f32 = dimension.number as f32;
                            println!("Size: {:?}", size);
        
                            commands.spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.80 })),
                                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                                transform: Transform::from_xyz((coord.x) as f32, 
                                                                -0.3 as f32, 
                                                                (coord.z) as f32),
                                ..default()
                            });
        
                            let mut height = 0.1;
        
                            //should shift parts within 1x1 square (not necessarily one building)

                            let mut part_height = (size / 2.0) as f32;
                            while height < size{
                                commands.spawn_bundle(PbrBundle {
                                    mesh: meshes.add(Mesh::from(shape::Cube { size: part_height})),
                                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                                    transform: Transform::from_xyz(coord.x as f32, 
                                                                height + part_height / 2.0 as f32, 
                                                                coord.z as f32),
                                    ..default()
                                });
        
                                height += part_height;

                                part_height = part_height / 2.0;
                                println!("part: {}", part_height);
                            }
                        }
                    }
                }
            }
        }
        println!("count {}", count);
    }
}
