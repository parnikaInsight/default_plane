use backroll_transport_udp::*;
use bevy::tasks::IoTaskPool;
use bevy::{core::FixedTimestep, prelude::*};
use bevy_backroll::{backroll::*, *};
use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::ops::Deref;
use std::marker::PhantomData;

use crate::players::{info, create, movement};

// pub struct BevyBackrollConfig<Input>;

// impl Config for BevyBackrollConfig<movement::PlayerInputFrame> {
//     //type Input = movement::PlayerInputFrame;
//     type State = GameState;
// }

// pub struct BevyBackrollConfig<Input> {
//     _marker: PhantomData<Input>,
// }

// impl<Input: PartialEq + bytemuck::Pod + bytemuck::Zeroable + Send + Sync> Config
//     for BevyBackrollConfig<Input>
// {
//     type Input =  movement::PlayerInputFrame;
//     type State = GameState;
// }

#[derive(Clone, PartialEq, Hash)]
pub struct GameState {}

const MATCH_UPDATE_LABEL: &str = "MATCH_UPDATE";

const DELTA_TIME: f32 = 1.0 / 60.0; // in ms

pub struct OurBackrollPlugin;

impl Plugin for OurBackrollPlugin {
    fn build(&self, builder: &mut App) {
        println!("our backroll plugin");
        builder
            .add_plugin(BackrollPlugin)
            // .with_rollback_run_criteria::<BevyBackrollConfig<movement::PlayerInputFrame>, _>(
            //     FixedTimestep::step(DELTA_TIME.into()).with_label(MATCH_UPDATE_LABEL),
            // )
            .register_rollback_input::<movement::PlayerInputFrame, _>(
                movement::sample_input.system(), //need .system()
            );
            //.with_rollback_system::<BevyBackrollConfig, _>(movement::player_movement.system());
            //.with_world_save_system::<BevyBackrollConfig, _>(save_world.system());
            // .with_world_load_system::<BackrollConfig, _>(load_world.system());
    }
}

#[derive(Debug)]
pub struct StartupNetworkConfig {
    pub client: usize,
    pub bind: SocketAddr,
    pub remote: SocketAddr,
}

fn save_world() -> GameState {
    //println!("Save da world");
    GameState {}
}

fn load_world(state: In<GameState>) {
    //println!("Load da world");
}

pub fn setup_game(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    println!("in setup game");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(info::Materials {
        player_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
}