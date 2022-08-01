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

use crate::cubemap::cubemap_materials::*;

pub const CUBEMAPS: &[(&str, CompressedImageFormats)] = &[
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

struct Cubemap {
    is_loaded: bool,
    index: usize,
    image_handle: Handle<Image>,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // // a grid of spheres with different metallicity and roughness values
    // let mesh_handle = meshes.add(Mesh::from(shape::Icosphere {
    //     radius: 0.45,
    //     subdivisions: 32,
    // }));
    // for y in -2..=2 {
    //     for x in -5..=5 {
    //         let x01 = (x + 5) as f32 / 10.0;
    //         let y01 = (y + 2) as f32 / 4.0;
    //         commands.spawn_bundle(PbrBundle {
    //             mesh: mesh_handle.clone(),
    //             material: materials.add(StandardMaterial {
    //                 base_color: Color::hex("ffd891").unwrap(),
    //                 metallic: y01,
    //                 perceptual_roughness: x01,
    //                 ..default()
    //             }),
    //             transform: Transform::from_xyz(x as f32, y as f32 + 0.5, 0.0),
    //             ..default()
    //         });
    //     }
    // }
    // // unlit sphere
    // commands.spawn_bundle(PbrBundle {
    //     mesh: mesh_handle,
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::hex("ffd891").unwrap(),
    //         unlit: true,
    //         ..default()
    //     }),
    //     transform: Transform::from_xyz(-5.0, -2.5, 0.0),
    //     ..default()
    // });

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
    // // camera
    // commands
    //     .spawn_bundle(Camera3dBundle {
    //         transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::default(), Vec3::Y),
    //         ..default()
    //     })
    //     .insert_bundle((
    //         CameraController::default(),
    //         EnvironmentMap {
    //             handle: skybox_handle.clone(),
    //         },
    //     ));

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

pub fn cycle_cubemap_asset(
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

pub fn asset_loaded(
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

pub fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}