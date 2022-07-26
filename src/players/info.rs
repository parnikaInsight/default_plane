use backroll_transport_udp::*;
use bevy::tasks::IoTaskPool;
use bevy::{core::FixedTimestep, prelude::*};
use bevy_backroll::{backroll::*, *};
use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::ops::Deref;

#[derive(Clone, Component)]
pub struct Player {
    pub handle: PlayerHandle, // the network id
}
pub struct Materials {
    pub player_material: Handle<ColorMaterial>,
}