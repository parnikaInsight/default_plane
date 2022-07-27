use backroll_transport_udp::*;
use bevy::tasks::IoTaskPool;
use bevy::{core::FixedTimestep, prelude::*};
use bevy_backroll::{backroll::*, *};
use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::ops::Deref;

use crate::players::{info, create, movement};

bitflags! {
    #[derive(Default, Pod, Zeroable)]
    #[repr(C)]
    pub struct PlayerInputFrame: u32 {
        // bit shift the stuff in the input struct
        const UP = 1<<0;
        const DOWN = 1<<1;
        const LEFT = 1<<2;
        const RIGHT = 1<<3;
    }
}

pub fn sample_input(handle: In<PlayerHandle>, keyboard_input: Res<Input<KeyCode>>) -> PlayerInputFrame {
    let mut local_input = PlayerInputFrame::empty();

    // local input handling
    {
        if keyboard_input.pressed(KeyCode::Left) {
            local_input.insert(PlayerInputFrame::LEFT);
            println!("Left");
        } else if keyboard_input.pressed(KeyCode::Right) {
            local_input.insert(PlayerInputFrame::RIGHT);
            println!("Right");
        }

        if keyboard_input.pressed(KeyCode::Up) {
            local_input.insert(PlayerInputFrame::UP);
            println!("Up");
        } else if keyboard_input.pressed(KeyCode::Down) {
            local_input.insert(PlayerInputFrame::DOWN);
            println!("Down");
        }
    }

    local_input
}

pub fn player_movement(
    keyboard_input: Res<GameInput<PlayerInputFrame>>,
    mut player_positions: Query<(&mut Transform, &info::Player)>,
) {
    println!("in player_movment");
    for (mut transform, player) in player_positions.iter_mut() {
        let input = keyboard_input.get(player.handle).unwrap();
        if input.contains(PlayerInputFrame::LEFT) {
            transform.translation.x -= 2.;
        }
        if input.contains(PlayerInputFrame::RIGHT) {
            transform.translation.x += 2.;
        }
        if input.contains(PlayerInputFrame::DOWN) {
            transform.translation.y -= 2.;
        }
        if input.contains(PlayerInputFrame::UP) {
            transform.translation.y += 2.;
        }
    }
    println!("after player_movment in method");
}