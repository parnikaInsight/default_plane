use crate::{camera::dolly_free::MainCamera, players::info};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_mod_picking::*;
use bevy_mod_picking::{HoverEvent, PickingEvent};
use bevy_render::camera::ActiveCamera;
use std::{
    marker::{PhantomData, PhantomPinned},
    thread, time,
};

pub fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Parnika").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

#[derive(Component)]
pub struct UICamera;

/// Spawn the UI camera
pub fn setup_ui_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // let sprite_handle = asset_server.load("branding/icon.png");

    // commands.spawn_bundle(SpriteBundle {
    //     texture: sprite_handle.clone(),
    //     ..default()
    // });

    println!("spawned");
}

pub fn click_for_display(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let sprite_handle: Handle<Image> = asset_server.load("branding/icon.png");

    for event in events.iter() {
        match event {
            PickingEvent::Hover(e) => {
                //spawn sprite bundle with transparent sprite background overlaid with text specific to player
                let player: Entity;
                if matches!(e, HoverEvent::JustEntered(player)) {
                    commands.spawn_bundle(SpriteBundle {
                        texture: sprite_handle.clone(),
                        ..default()
                    });

                    println!("here");
                    break;
                } else {
                    //despawn or make invisible
                }
            }
            _ => info!("nothing"),
        }
    }
}

pub fn print_events(mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Hover! {:?}", e),
            PickingEvent::Clicked(e) => info!("Click! {:?}", e),
        }
    }
}
