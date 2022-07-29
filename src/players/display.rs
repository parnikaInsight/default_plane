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

#[derive(Component)]
pub struct UICamera;

/// Spawn the UI camera
pub fn setup_ui_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component)]
pub struct InfoDisplay;

pub fn click_for_display(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    asset_server: Res<AssetServer>,
    players: Query<(Entity, &Transform, &mut info::Player)>,
    mut query: Query<(Entity, With<InfoDisplay>)>, //there should only be one info display at a time
) {
    let sprite_handle: Handle<Image> = asset_server.load("branding/icon.png");

    for event in events.iter() {
        match event {
            PickingEvent::Hover(e) => {
                //spawn sprite bundle with transparent sprite background overlaid with text specific to player
                if matches!(e, HoverEvent::JustEntered(_)) {
                    // if let HoverEvent::JustEntered(player) = e {
                    //     for i in players.iter() {
                    //         if i.0.id() == player.id() {
                    //             println!("value: {:?}", i.0);
                    //         }
                    //     }
                    // }

                    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                    let text_style = TextStyle {
                        font,
                        font_size: 50.0,
                        color: Color::WHITE,
                    };
                    let text_alignment = TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    };

                    if let HoverEvent::JustEntered(player) = e {
                        for i in players.iter() {
                            if i.0.id() == player.id() {
                                let id: String = i.2.handle.to_string();
                                let money = i.2.money.to_string();
                                let bounties = i.2.bounties.to_string();
                                commands
                                    .spawn_bundle(Text2dBundle {
                                        text: Text::with_section(
                                            String::from("Id: ") + &*id + 
                                                &*String::from("\nMoney: ") + &*money +
                                                &*String::from("\nBounties: ") + &*bounties ,
                                            text_style.clone(),
                                            text_alignment,
                                        ),
                                        ..default()
                                    })
                                    .insert(InfoDisplay);
                            }
                        }
                    }

                    // commands
                    //     .spawn_bundle(Text2dBundle {
                    //         text: Text::with_section(
                    //             "translation",
                    //             text_style.clone(),
                    //             text_alignment,
                    //         ),
                    //         ..default()
                    //     })
                    //     .insert(InfoDisplay);

                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgba(0.0, 0.0, 1.0, 0.7),
                                ..default()
                            },
                            texture: sprite_handle.clone(),
                            ..default()
                        })
                        .insert(InfoDisplay);
                } else {
                    //despawn or make invisible
                    for q in query.iter() {
                        commands.entity(q.0).despawn();
                        //commands.entity(query.single_mut().0).despawn();
                    }
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
