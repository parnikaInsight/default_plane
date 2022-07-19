//! A simple 3D scene with light shining over a cube sitting on a plane.

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

use camera_controller::*;
use cubemap_materials::*;

//blender
use bevy_blender::*;
// Use pan orbit camera
mod camera;

//cubemap
const CUBEMAPS: &[(&str, CompressedImageFormats)] = &[
    (
        "textures/Ryfjallet_cubemap.png",
        CompressedImageFormats::NONE,
    ),
    (
        "textures/Ryfjallet_cubemap_astc4x4.ktx2",
        CompressedImageFormats::ASTC_LDR,
    ),
    (
        "textures/Ryfjallet_cubemap_bc7.ktx2",
        CompressedImageFormats::BC,
    ),
    (
        "textures/Ryfjallet_cubemap_etc2.ktx2",
        CompressedImageFormats::ETC2,
    ),
];

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

//cubemap-----------------------------------------------
struct Cubemap {
    is_loaded: bool,
    index: usize,
    image_handle: Handle<Image>,
}

fn setup_cubemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // a grid of spheres with different metallicity and roughness values
    let mesh_handle = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.45,
        subdivisions: 32,
    }));
    for y in -2..=2 {
        for x in -5..=5 {
            let x01 = (x + 5) as f32 / 10.0;
            let y01 = (y + 2) as f32 / 4.0;
            commands.spawn_bundle(PbrBundle {
                mesh: mesh_handle.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::hex("ffd891").unwrap(),
                    metallic: y01,
                    perceptual_roughness: x01,
                    ..default()
                }),
                transform: Transform::from_xyz(x as f32, y as f32 + 0.5, 0.0),
                ..default()
            });
        }
    }
    // unlit sphere
    commands.spawn_bundle(PbrBundle {
        mesh: mesh_handle,
        material: materials.add(StandardMaterial {
            base_color: Color::hex("ffd891").unwrap(),
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(-5.0, -2.5, 0.0),
        ..default()
    });

    // directional 'sun' light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    let skybox_handle = asset_server.load(CUBEMAPS[0].0);
    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert_bundle((
            CameraController::default(),
            EnvironmentMap {
                handle: skybox_handle.clone(),
            },
        ));

    // ambient light
    // NOTE: The ambient light is used to scale how bright the environment map is so with a bright
    // environment map, use an appropriate colour and brightness to match
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 1.0,
    });

    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
}

const CUBEMAP_SWAP_DELAY: f64 = 3.0;

fn cycle_cubemap_asset(
    time: Res<Time>,
    mut next_swap: Local<f64>,
    mut cubemap: ResMut<Cubemap>,
    asset_server: Res<AssetServer>,
    render_device: Res<RenderDevice>,
) {
    let now = time.seconds_since_startup();
    if *next_swap == 0.0 {
        *next_swap = now + CUBEMAP_SWAP_DELAY;
        return;
    } else if now < *next_swap {
        return;
    }
    *next_swap += CUBEMAP_SWAP_DELAY;

    let supported_compressed_formats =
        CompressedImageFormats::from_features(render_device.features());

    let mut new_index = cubemap.index;
    for _ in 0..CUBEMAPS.len() {
        new_index = (new_index + 1) % CUBEMAPS.len();
        if supported_compressed_formats.contains(CUBEMAPS[new_index].1) {
            break;
        }
        info!("Skipping unsupported format: {:?}", CUBEMAPS[new_index]);
    }

    // Skip swapping to the same texture. Useful for when ktx2, zstd, or compressed texture support
    // is missing
    if new_index == cubemap.index {
        return;
    }

    cubemap.index = new_index;
    cubemap.image_handle = asset_server.load(CUBEMAPS[cubemap.index].0);
    cubemap.is_loaded = false;
}

fn asset_loaded(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cubemap_materials: ResMut<Assets<CubemapMaterial>>,
    mut cubemap_array_materials: ResMut<Assets<CubemapArrayMaterial>>,
    mut cubemap: ResMut<Cubemap>,
    cubes: Query<&Handle<CubemapMaterial>>,
    array_cubes: Query<&Handle<CubemapArrayMaterial>>,
    mut env_maps: Query<&mut EnvironmentMap>,
) {
    if !cubemap.is_loaded
        && asset_server.get_load_state(cubemap.image_handle.clone_weak()) == LoadState::Loaded
    {
        info!("Swapping to {}...", CUBEMAPS[cubemap.index].0);
        let mut image = images.get_mut(&cubemap.image_handle).unwrap();
        if image.texture_descriptor.array_layer_count() == 1 {
            image.texture_descriptor.size.depth_or_array_layers =
                image.texture_descriptor.size.height / image.texture_descriptor.size.width;
            image.texture_descriptor.size.height = image.texture_descriptor.size.width;
        }

        let is_array = image.texture_descriptor.array_layer_count() > 6;

        // spawn cube
        if is_array {
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::CubeArray),
                ..default()
            });
            let mut updated = false;
            for handle in array_cubes.iter() {
                if let Some(material) = cubemap_array_materials.get_mut(handle) {
                    updated = true;
                    material.base_color_texture = Some(cubemap.image_handle.clone_weak());
                }
            }
            if !updated {
                commands.spawn_bundle(MaterialMeshBundle::<CubemapArrayMaterial> {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 10000.0 })),
                    material: cubemap_array_materials.add(CubemapArrayMaterial {
                        base_color_texture: Some(cubemap.image_handle.clone_weak()),
                    }),
                    ..default()
                });
            }
        } else {
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
            let mut updated = false;
            for handle in cubes.iter() {
                if let Some(material) = cubemap_materials.get_mut(handle) {
                    updated = true;
                    material.base_color_texture = Some(cubemap.image_handle.clone_weak());
                }
            }
            if !updated {
                commands.spawn_bundle(MaterialMeshBundle::<CubemapMaterial> {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 10000.0 })),
                    material: cubemap_materials.add(CubemapMaterial {
                        base_color_texture: Some(cubemap.image_handle.clone_weak()),
                    }),
                    ..default()
                });
            }
        }

        for mut env_map in env_maps.iter_mut() {
            env_map.handle = cubemap.image_handle.clone_weak();
        }

        cubemap.is_loaded = true;
    }
}

fn animate_cubebox_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}

//blender-----------------------------------------------
fn setup_blender_camera(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    // Create and spawn a Blender object using a BlenderObjectBundle
    // This will only spawn the one object
    // This example is included for completeness, but it is recommended to use spawn_blender_object instead
    let mut suzanne = BlenderObjectBundle::new(&asset_server, "demo.blend", "Suzanne").unwrap();
    suzanne.transform = Transform::from_translation(Vec3::new(-4.0, 0.0, 0.0));
    commands.spawn_bundle(suzanne);

    // Spawn Blender object with children
    // The parent object's transform is taken from Blender
    spawn_blender_object(
        &mut commands,
        &asset_server,
        "demo.blend",
        "TransformCube",
        true,
        None,
    );

    // Spawn Blender object with children
    // The parent object's transform is provided
    spawn_blender_object(
        &mut commands,
        &asset_server,
        "demo.blend",
        "Suzanne",
        true,
        Some(Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::IDENTITY,
                Vec3::new(4.0, 0.0, 0.0),
            ),
        )),
    );

    // Light and camera
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    let translation = Vec3::new(5.0, 5.0, 5.0);
    let radius = translation.length();

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(camera::PanOrbitCamera {
            radius,
            ..Default::default()
        });
}

//gltf------------------------------------------------

//fox example
struct Animations(Vec<Handle<AnimationClip>>);

fn setup_fox(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load("models/animated/Fox.glb#Animation2"),
        asset_server.load("models/animated/Fox.glb#Animation1"),
        asset_server.load("models/animated/Fox.glb#Animation0"),
    ]));

    // Camera
    // commands.spawn_bundle(Camera3dBundle {
    //     transform: Transform::from_xyz(100.0, 100.0, 150.0)
    //         .looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
    //     ..default()
    // });

    // Plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 500000.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // Light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        )),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // Fox
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("models/animated/Fox.glb#Scene0"),
        ..default()
    });

    println!("Animation controls:");
    println!("  - spacebar: play / pause");
    println!("  - arrow up / down: speed up / slow down animation playback");
    println!("  - arrow left / right: seek backward / forward");
    println!("  - return: change animation");
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_player: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
) {
    if let Ok(mut player) = animation_player.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::Up) {
            let speed = player.speed();
            player.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Right) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed + 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Return) {
            *current_animation = (*current_animation + 1) % animations.0.len();
            player
                .play(animations.0[*current_animation].clone_weak())
                .repeat();
        }
    }
}

// ----------------------------------------------------------------
// // //helmet example
// fn main() {
//     App::new()
//         .insert_resource(AmbientLight {
//             color: Color::WHITE,
//             brightness: 1.0 / 5.0f32,
//         })
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(setup)
//         .add_system(animate_light_direction)
//         .run();
// }

fn setup_helmet(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..default()
    });
    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0"),
        ..default()
    });
}

fn animate_helmet_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}

