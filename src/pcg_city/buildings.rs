//uses bevy 0.7

use crate::camera::dolly_free;
use crate::math::grid::MyGrid;
use crate::math::random::{get_block_dimensions, get_block_location};
use crate::math::{city_perlin, grid, random};
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use std::{thread, time};
use bevy_rapier3d::prelude::*;

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

        let mut x_right_bound: i32 = convert(vector.x + 2.0);
        if x_right_bound > 7 {
            x_right_bound = 8;
        }
        let mut x_left_bound: i32 = convert(vector.x - 2.0);
        if x_left_bound < -7 {
            x_left_bound = -8;
        }

        let mut z_front_bound: i32 = convert(vector.z + 2.0);
        if z_front_bound > 7 {
            z_front_bound = 8;
        }

        let mut z_back_bound: i32 = convert(vector.z - 2.0);
        if z_back_bound < -7 {
            z_back_bound = -8;
        }

        for x_iter in x_left_bound..x_right_bound {
            for z_iter in z_back_bound..z_front_bound {
                let x: f32 = x_iter as f32;
                let z = z_iter as f32;
                let cam_vec = Vec2::new(vector.x, vector.z);
                let pot_cube_vec = Vec2::new(x, z);
                if cam_vec.distance(pot_cube_vec) <= 2.0 {
                    if let Some(b) = grid.coordinate_system.get_mut(&vec![x_iter, z_iter]) {
                        if *b == false {
                            *b = true;
                            let coord = grid::Coordinate {
                                x: x as f64,
                                y: 0.0 as f64,
                                z: z as f64,
                            };

                            println!("{:?}", coord);

                            let height =
                                height_noise_fn.function.get([coord.x / 7.0, coord.z / 7.0]);
                            let dimension = random::Random {
                                number: height + 1.0,
                            };

                            // cube
                            let size: f32 = dimension.number as f32;
                            println!("Size: {:?}", size);

                            // commands
                            //     .spawn_bundle(PbrBundle {
                            //         mesh: meshes.add(Mesh::from(shape::Cube { size: 0.80 })),
                            //         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                            //         transform: Transform::from_xyz(
                            //             (coord.x) as f32,
                            //             -0.3 as f32,
                            //             (coord.z) as f32,
                            //         ),
                            //         ..default()
                            //     })
                            //     .insert(RigidBody::Dynamic)
                            //     .insert(Collider::cuboid(0.40, 0.40, 0.40)) //half the cube size
                            //     .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)));

                            let mut height = 0.1;

                            //should shift parts within 1x1 square (not necessarily one building)

                            let mut part_height = (size / 2.0) as f32;
                            while height < size {
                                commands
                                .spawn_bundle(PbrBundle {
                                    mesh: meshes.add(Mesh::from(shape::Cube { size: part_height })),
                                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                                    transform: Transform::from_xyz(
                                        coord.x as f32,
                                        height + part_height / 2.0 as f32,
                                        coord.z as f32,
                                    ),
                                    ..default()
                                })
                                .insert(RigidBody::Dynamic)
                                .insert(Collider::cuboid(part_height / 2.0, part_height / 2.0, part_height / 2.0)) //half the cube size
                                .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)));

                                height += part_height;

                                part_height = part_height / 2.0;
                                println!("part: {}", part_height);
                            }
                        }
                    }
                }
            }
        }
    }
}
