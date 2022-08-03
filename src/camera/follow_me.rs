use bevy_dolly::prelude::*;
use bevy::prelude::*;
use bevy_ggrs::Rollback;
use bevy_mod_picking::*;

use crate::players::{info, movement};
use crate::ggrs_rollback::network;

// #[derive(Component)]
// pub struct Rig;

// #[derive(Component)]
// pub struct MainCamera;

// pub fn setup_follow_camera(
//     mut commands: Commands,
//     mut query: Query<(&mut Transform, &info::Player, &network::Me), With<Rollback>>, 
// ) {
//     println!("started setup camera");
//     if query.is_empty() { 
//         println!("emtpy"); 
//     } 
//     for i in query.iter_mut() {
//         println!("in query");
//     }
//     let (mut t, p, m) = query.single_mut();
//     let camera = CameraRig::builder()
//             .with(Position::new(t.translation))
//             .with(Rotation::new(t.rotation))
//             .with(Smooth::new_position(1.25).predictive(true))
//             .with(Arm::new(Vec3::new(0.0, 1.5, -3.5)))
//             .with(Smooth::new_position(2.5))
//             .with(
//                 LookAt::new(t.translation + Vec3::Y)
//                     .tracking_smoothness(1.25)
//                     .tracking_predictive(true),
//             )
//             .build();

//     commands.spawn().insert(camera).insert(Rig);

//     commands
//         .spawn_bundle(Camera3dBundle {
//             ..Default::default()
//         })
//         .insert(UiCameraConfig { //idk why not displaying
//             show_ui: true,
//             ..default()
//         })

//         .insert_bundle(PickingCameraBundle::default())
//         .insert(MainCamera);

//     // light
//     commands.spawn_bundle(PointLightBundle {
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..Default::default()
//     });
//     println!("finished setup camera");
// }

pub fn frame(
    mut q: Query<(&mut CameraRig, &network::Rig)>,
    //mut cam: Query<(&mut Transform), With<network::MainCamera>>,
    mut me: Query<(&mut Transform, &info::Player, &network::Me), With<Rollback>>,
) {
    let (mut t, p, m) = me.single_mut();
    println!("Me: {:?}", t);
    let (mut camera, rig) = q.single_mut();
    camera.driver_mut::<Position>().translation = t.translation;
    camera.driver_mut::<Rotation>().rotation = t.rotation;
    camera.driver_mut::<LookAt>().target = t.translation + Vec3::Y;

    // let mut cam_3d = cam.single_mut();
    // cam_3d = t;
}
