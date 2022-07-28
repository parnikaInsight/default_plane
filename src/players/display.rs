use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use std::{thread, time};

pub fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Parnika").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

pub fn ui_display(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

pub fn click_for_display(
    mut egui_context: ResMut<EguiContext>,
    mut events: EventReader<PickingEvent>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Hover(e) => {
                ui_display(egui_context);
                println!("done");
                break;
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
