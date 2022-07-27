use std::net::SocketAddr;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_backroll::{backroll::*, *};
use bevy_dolly::prelude::*;
use bevy_rapier3d::prelude::*;
use std::env;

#[macro_use]
extern crate bitflags;

mod backroll;
mod math;
mod players;
use math::grid::MyGrid;
mod camera;
mod pcg_city;

use backroll::network_config;
use players::{create, info, movement};

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

fn start_app(player_num: usize) -> (usize, SocketAddr, SocketAddr) {
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

    (player_num, bind_addr, remote_addr)

    // commands.insert_resource(network_config::StartupNetworkConfig {
    //     client: player_num,
    //     bind: bind_addr,
    //     remote: remote_addr,
    // });
}

// fn main() {
//     App::new()
//         .add_startup_system(network_config::setup_game)
//         .add_startup_system(start_app)
//         //.add_startup_system(create::spawn_players)
//         .add_startup_stage("game_setup", SystemStage::single(create::spawn_players))

//         .add_plugins(DefaultPlugins)
//         .add_plugin(network_config::OurBackrollPlugin)
//         // .insert_resource(network_config::StartupNetworkConfig {
//         //     client: player_num,
//         //     bind: bind_addr,
//         //     remote: remote_addr,
//         // })

//         .add_system(movement::player_movement)

//         .run();
// }

fn main() {
    let mut args = std::env::args();
    let base = args.next().unwrap();
    if let Some(player_num) = args.next() {
        println!("many worlds pring created?");
        let (player_num, bind_addr, remote_addr) = start_app(player_num.parse().unwrap());
        App::new()
            .add_startup_system(network_config::setup_game)
            //.add_startup_system(start_app)
            //.add_startup_system(create::spawn_players)
            .add_startup_stage("game_setup", SystemStage::single(create::spawn_players))
            .add_plugins(DefaultPlugins)
            .add_plugin(network_config::OurBackrollPlugin)
            .insert_resource(network_config::StartupNetworkConfig {
                client: player_num,
                bind: bind_addr,
                remote: remote_addr,
            })
            .add_rollback_system(movement::player_movement)
            .run();
    } else {
        println!("in else");
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

// fn main() {
//    // env::set_var("RUST_BACKTRACE", "1");

//     let (player_num, bind_addr, remote_addr) = start_app(2); //works w/ more than 2
//     let mut app = App::new();
//     app
//         .add_plugins(DefaultPlugins)
//         .add_plugin(network_config::OurBackrollPlugin)

//         .add_startup_system(network_config::setup_game);
//         //.add_startup_system(start_app)

//     app.add_startup_system(create::spawn_players);
//         //.add_startup_stage("game_setup", SystemStage::single(create::spawn_players))

//     app.add_rollback_system(movement::player_movement);

//     //println!("after spawning");
//     app
//         .insert_resource(network_config::StartupNetworkConfig {
//             client: player_num,
//             bind: bind_addr,
//             remote: remote_addr,
//         });

//     //println!("after inserting resource");
//     //app.with_rollback_system::<BevyBackrollConfig, _>(movement::player_movement.system());
//     //app.add_system(movement::player_movement);

//     //println!("after movement");
//     app
//         .run();
// }
