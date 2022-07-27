use backroll_transport_udp::*;
use bevy::tasks::IoTaskPool;
use bevy::{core::FixedTimestep, prelude::*};
use bevy_backroll::{backroll::*, *};
use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::ops::Deref;
use backroll::*;

use crate::players::{info, movement};
use crate::backroll::network_config;

pub fn spawn_players(
    mut commands: Commands,
    config: Res<network_config::StartupNetworkConfig>,
    pool: Res<IoTaskPool>,
    //materials: Res<info::Materials>,
) {
    //println!("world id: {:?}", world.id());

    println!("spawning players");
    //peerid needs to go here
    let socket = UdpManager::bind(pool.deref().deref().clone(), config.bind).unwrap();
    let peer = socket.connect(UdpConnectionConfig::unbounded(config.remote));

    commands.insert_resource(socket);

    //println!("check 1"); 

    let mut builder = backroll::P2PSession::<BevyBackrollConfig<movement::PlayerInputFrame>>::build();

    //println!("check 2"); 

    commands
        .spawn_bundle(SpriteBundle {
            //material: materials.player_material.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        // make sure to clone the player handles for reference stuff
        .insert(if config.client == 0 {
            // set up local player
            info::Player {
                handle: builder.add_player(backroll::Player::Local),
            }
        } else {
            // set up remote player
            info::Player {
                // make sure to clone the remote peer for reference stuff
                handle: builder.add_player(backroll::Player::Remote(peer.clone())),
            }
        });

    //println!("check 3"); 

    commands
        .spawn_bundle(SpriteBundle {
            //material: materials.player_material.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(if config.client == 1 {
            // set up local player
            info::Player {
                handle: builder.add_player(backroll::Player::Local),
            }
        } else {
            // set up remote player
            info::Player {
                handle: builder.add_player(backroll::Player::Remote(peer)),
            }
        });
    //println!("check 4"); 

   //let MyP2PSession = bevy_backroll::backroll::P2PSession::clone();
    //let MyP2PSession = builder.start(pool.deref().deref().clone()).unwrap();
    //let MyP2PSession = HereP2PSession();
    //commands.start_backroll_session(MyP2PSession);
    //commands.insert_resource(MyP2PSession);
   // commands.start_backroll_session(new_internal(builder, pool));
    //commands.start_backroll_session(backroll::P2PSession::<BevyBackrollConfig<movement::PlayerInputFrame>>::new_internal(builder, pool));
    
    commands.start_backroll_session(builder.start(pool.deref().deref().clone()).unwrap());
        //problem is here

    //println!("check 5"); 
}

