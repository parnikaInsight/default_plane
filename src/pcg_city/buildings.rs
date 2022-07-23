use crate::math::{random, city_perlin, grid};
use bevy::prelude::*;
use std::{thread, time};

pub fn spawn_buildings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let delay = time::Duration::from_secs(3);

    //get block dimensions
    let x = random::get_block_dimensions();
    let dimension = random::Random{
        x,    
    };
    
    //get block location
    let loc = random::get_block_location();
    let location = random::Random{
        x: loc,    
    };
    println!("{:?}", dimension);
    
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: dimension.x })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        //transform: Transform::from_xyz(location.x * 14.0, 0.0, 0.0),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    //thread::sleep(delay);
}
