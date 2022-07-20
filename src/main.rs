//! A simple 3D scene with light shining over a cube sitting on a plane.

//cubemap:
// - cubemap_setup
// - cubemap_materials
// - camera_controller

//blender:
// - camera.rs

//gltf
// - bevy_gltf_setup

//cubemap
use bevy::{
    asset::LoadState,
    prelude::*,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        renderer::RenderDevice,
        texture::CompressedImageFormats,
        view::EnvironmentMap,
    },
};

//cubemap
mod camera_controller;
mod cubemap_materials;
mod cubemap_setup;
mod camera; // Use pan orbit camera
mod bevy_gltf_setup;

use camera_controller::*;
use cubemap_materials::*;
use cubemap_setup::*;
use camera::*;
use bevy_gltf_setup::*;

//blender
use bevy_blender::*;

fn main() {
    App::new()
        //cubemap
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<CubemapMaterial>::default())
        .add_plugin(MaterialPlugin::<CubemapArrayMaterial>::default())
        
        //blender
        .add_plugin(BlenderPlugin)
        .add_startup_system(setup_blender_camera)
        //.add_system(camera::pan_orbit_camera)

        //cubemap
        .add_startup_system(setup_cubemap)
        .add_system(cycle_cubemap_asset)
        .add_system(asset_loaded.after(cycle_cubemap_asset))
        .add_system(camera_controller)
        .add_system(animate_cubebox_light_direction)

        //fox
        .add_startup_system(setup_fox)
        .add_system(setup_scene_once_loaded)
        .add_system(keyboard_animation_control)

        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        //helmet
        .add_startup_system(setup_helmet)
        .add_system(animate_helmet_light_direction)

        .run();
}

