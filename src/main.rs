use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use bevy_rapier3d::prelude::*;

#[macro_use]
extern crate bitflags;

mod math;
mod players;
mod backroll;
use math::grid::MyGrid;
mod pcg_city;
mod camera;

use players::{info, create, movement};
use backroll::network_config;

// fn main() {
//     App::new()
//         .init_resource::<MyGrid>()
//         .insert_resource(Msaa { samples: 4 })
//         .add_plugins(DefaultPlugins)
//         .add_plugin(DollyCursorGrab)
//         .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
//         .add_plugin(RapierDebugRenderPlugin::default())
//         .add_startup_system(camera::dolly_free::setup)
//         .add_system(camera::dolly_free::update_camera)

//         .init_resource::<math::city_perlin::HeightNoiseFn>()
//         .add_system(pcg_city::buildings::spawn_buildings)

//         .run();
//   }

fn start_app(player_num: usize) {
    let bind_addr = if player_num == 0 {
        "127.0.0.1:4001".parse().unwrap()
    } else {
        "127.0.0.1:4002".parse().unwrap()
    };

    let remote_addr = if player_num == 0 {
        "127.0.0.1:4002".parse().unwrap()
    } else {
        "127.0.0.1:4001".parse().unwrap()
    };

    App::new()
        .add_startup_system(network_config::setup_game)
        //.add_startup_system(create::spawn_players)
        .add_startup_stage("game_setup", SystemStage::single(create::spawn_players))

        .add_plugins(DefaultPlugins)
        .add_plugin(network_config::OurBackrollPlugin)
        .insert_resource(network_config::StartupNetworkConfig {
            client: player_num,
            bind: bind_addr,
            remote: remote_addr,
        })

        .add_system(movement::player_movement)

        .run();
}
fn main() {
    let mut args = std::env::args();
    let base = args.next().unwrap();
    if let Some(player_num) = args.next() {
        start_app(player_num.parse().unwrap());
    } else {
        let mut child_1 = std::process::Command::new(base.clone())
            .args(&["0"])
            .spawn()
            .unwrap();
        let mut child_2 = std::process::Command::new(base)
            .args(&["1"])
            .spawn()
            .unwrap();
        child_1.wait().unwrap();
        child_2.wait().unwrap();
    }
}