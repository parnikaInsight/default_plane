use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_ggrs::{GGRSPlugin, SessionType};
use ggrs::{P2PSession, PlayerType, SessionBuilder, UdpNonBlockingSocket};
use structopt::StructOpt;

mod box_game;
use box_game::box_logic;

const FPS: usize = 60;
const ROLLBACK_DEFAULT: &str = "rollback_default";

// structopt will read command line parameters for u
#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    local_port: u16,
    #[structopt(short, long)]
    players: Vec<String>,
    #[structopt(short, long)]
    spectators: Vec<SocketAddr>,
}

struct NetworkStatsTimer(Timer);

fn print_events_system(mut session: ResMut<P2PSession<box_logic::GGRSConfig>>) {
    for event in session.events() {
        println!("GGRS Event: {:?}", event);
    }
}

fn print_network_stats_system(
    time: Res<Time>,
    mut timer: ResMut<NetworkStatsTimer>,
    p2p_session: Option<Res<P2PSession<box_logic::GGRSConfig>>>,
) {
    // print only when timer runs out
    if timer.0.tick(time.delta()).just_finished() {
        if let Some(sess) = p2p_session {
            let num_players = sess.num_players() as usize;
            for i in 0..num_players {
                if let Ok(stats) = sess.network_stats(i) {
                    println!("NetworkStats for player {}: {:?}", i, stats);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read cmd line arguments
    let opt = Opt::from_args();
    let num_players = opt.players.len(); //number of discovered peers
    assert!(num_players > 0);

    // create a GGRS session
    let mut sess_build = SessionBuilder::<box_logic::GGRSConfig>::new()
        .with_num_players(num_players)
        .with_max_prediction_window(12) // (optional) set max prediction window
        .with_input_delay(2); // (optional) set input delay for the local player

    // add players
    for (i, player_addr) in opt.players.iter().enumerate() {
        // local player
        if player_addr == "localhost" { //receive my listening on address
            sess_build = sess_build.add_player(PlayerType::Local, i)?;
        } else {
            // remote players
            let remote_addr: SocketAddr = player_addr.parse()?; //receive addr of discovered peers
            sess_build = sess_build.add_player(PlayerType::Remote(remote_addr), i)?;
        }
    }

    // optionally, add spectators
    for (i, spec_addr) in opt.spectators.iter().enumerate() {
        sess_build = sess_build.add_player(PlayerType::Spectator(*spec_addr), num_players + i)?;
    }

    // start the GGRS session
     
    //let socket = UdpNonBlockingSocket::bind_to_port("/ip4/0.0.0.0/udp/0/quic")?;
    let socket = UdpNonBlockingSocket::bind_to_port(opt.local_port)?;
    let sess = sess_build.start_p2p_session(socket)?;

    let mut app = App::new();
    GGRSPlugin::<box_logic::GGRSConfig>::new()
        // define frequency of rollback game logic update
        .with_update_frequency(FPS)
        // define system that returns inputs given a player handle, so GGRS can send the inputs around
        .with_input_system(box_logic::input)
        // register types of components AND resources you want to be rolled back
        .register_rollback_type::<Transform>()
        .register_rollback_type::<box_logic::Velocity>()
        .register_rollback_type::<box_logic::FrameCount>()
        // these systems will be executed as part of the advance frame update
        .with_rollback_schedule(
            Schedule::default().with_stage(
                ROLLBACK_DEFAULT,
                SystemStage::parallel()
                    .with_system(box_logic::move_cube_system)
                    .with_system(box_logic::increase_frame_system),
            ),
        )
        // make it happen in the bevy app
        .build(&mut app);

    // continue building/running the app like you normally would
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor { //must come before default plugin
            width: 720.,
            height: 720.,
            title: "GGRS Box Game".to_owned(),
            ..Default::default()
        })
        
        .insert_resource(opt)
        .add_plugins(DefaultPlugins)
        .add_startup_system(box_logic::setup_system)
        // add your GGRS session
        .insert_resource(sess)
        .insert_resource(SessionType::P2PSession)

        // register a resource that will be rolled back
        .insert_resource(box_logic::FrameCount { frame: 0 })
        //print some network stats - not part of the rollback schedule as it does not need to be rolled back
        .insert_resource(NetworkStatsTimer(Timer::from_seconds(2.0, true)))
        .add_system(print_network_stats_system)
        .add_system(print_events_system)
        .run();

    Ok(())
}


// cargo run -- --local-port 7000 --players localhost 127.0.0.1:7001
// cargo run -- --local-port 7001 --players 127.0.0.1:7000 localhost

//every terminal must run with same order of players (bc of handle controlling moves)


//----------------------------------------------------------------

// use bevy::input::mouse::MouseMotion;
// use bevy::prelude::*;
// use bevy_dolly::prelude::*;
// use bevy_rapier3d::prelude::*;
// use std::env;

// mod math;
// use math::grid::MyGrid;
// mod camera;
// mod pcg_city;


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
//         //.add_system(pcg_city::buildings::spawn_buildings)

//         .run();
//   }

