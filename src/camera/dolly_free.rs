use dolly::{driver::RigDriver, prelude::*};
use bevy::prelude::*;

#[derive(Debug)]
pub struct Follow(dolly::rig::CameraRig);

impl RigDriver for Follow {
    fn update(&mut self, params: dolly::rig::RigUpdateParams) -> dolly::transform::Transform {
        self.0.update(params.delta_time_seconds)
    }
}

impl Follow {
    pub fn from_transform(transform: dolly::transform::Transform) -> Self {
        Self(
            dolly::rig::CameraRig::builder()
                .with(Position::new(transform.position))
                .with(Rotation::new(transform.rotation))
                .with(Smooth::new_position(1.25).predictive(true))
                .with(Arm::new(dolly::glam::Vec3::new(4.0, 3.5, 2.)))
                .with(Smooth::new_position(2.5))
                .with(
                    LookAt::new(transform.position + dolly::glam::Vec3::Y)
                        .tracking_smoothness(1.25)
                        .tracking_predictive(true),
                )
                .build(),
        )
    }

    pub fn follow(
        &mut self,
        position: dolly::glam::Vec3,
        rotation: dolly::glam::Quat,
        target: dolly::glam::Vec3,
    ) {
        self.0.driver_mut::<Position>().position = position;
        self.0.driver_mut::<Rotation>().rotation = rotation;
        self.0.driver_mut::<LookAt>().target = target;
    }
}

pub fn camera_move_free(
    //mut rig: Query<&mut MyCameraRig>,
    mut query_rig: ResMut<MyCameraRig>,
    mut query_cam: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    //mut query_cam: Query<(&mut PerspectiveCameraBundle)>,
) {
    // let mut camera_xform: dolly::transform::Transform = dolly::transform::Transform {
    //     position: Vec3::new(0.0, 0.0, 0.0),
    //     rotation: Quat::IDENTITY,
    // };

    if key_input.pressed(KeyCode::C) {
        query_rig.is_player = !query_rig.is_player;
        println!(
            "{}",
            if query_rig.is_player {
                "Player"
            } else {
                "Camera"
            }
        );
    }

    let mut delta_pos = Vec3::new(0., 0., 0.);

    if key_input.pressed(KeyCode::D) {
        delta_pos.x += query_rig.speed;
    }
    if key_input.pressed(KeyCode::A) {
        delta_pos.x -= query_rig.speed;
    }
    if key_input.pressed(KeyCode::S) {
        delta_pos.z += query_rig.speed;
    }
    if key_input.pressed(KeyCode::W) {
        delta_pos.z -= query_rig.speed;
    }
    if key_input.pressed(KeyCode::LShift) {
        delta_pos.y -= query_rig.speed;
    }
    if key_input.pressed(KeyCode::Space) {
        delta_pos.y += query_rig.speed;
    }

    if query_rig.is_player {
        query_rig.yellow_pos += delta_pos;
    } else {
        query_rig.camera_transform.position.x += delta_pos.x;
        query_rig.camera_transform.position.y += delta_pos.y;
        query_rig.camera_transform.position.z += delta_pos.z;
    }

    //Update the camera follow driver
    let pos = query_rig.camera_transform.position;
    let rot = query_rig.camera_transform.rotation;
    let yx = query_rig.yellow_pos.x;
    let yy = query_rig.yellow_pos.y;
    let yz = query_rig.yellow_pos.z;

    query_rig.camera.driver_mut::<Follow>().follow(
        pos,
        rot,
        dolly::glam::Vec3::new(
            yx,
            yy,
            yz,
        ),
    );

    // Update the camera rig, and get the interpolated transform
    let camera_xform = query_rig.camera.update(time.delta_seconds());

    if let Ok(mut cam) = query_cam.get_single_mut() {
        println!("in query cam");
        cam.translation = camera_xform.position;
        cam.rotation = camera_xform.rotation;
    }
}

//----------------------------------------------------------------
#[derive(Component)]
pub struct MyCameraRig {
    pub camera_transform: dolly::transform::Transform,
    pub camera: CameraRig,
    pub yellow_pos: Vec3,
    pub speed: f32,
    pub is_player: bool,
}

impl Default for MyCameraRig {
    fn default() -> Self {
        // The transform (the position and rotation) of the camera
        let camera_transform: dolly::transform::Transform = dolly::transform::Transform::from_position_rotation(
            dolly::glam::Vec3::new(0.0, 0., 4.),
            Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),//IDENTITY,
        );

        // Create a smoothed orbit camera
        let camera = CameraRig::builder()
            .with(Follow::from_transform(camera_transform))
            .build();

        println!("camera rig created");

        MyCameraRig {
            camera_transform,
            camera,
            yellow_pos: Vec3::new(2., 2., 2.),
            speed: 0.05,
            is_player: true,
        }

        // let mut camera = CameraRig::builder()
        //     .with(Position::new(Vec3::Y))
        //     .with(YawPitdrch::new())
        //     .with(Smooth::new_position_rotation(1.0, 1.0))
        //     .build();
        // let base = MyCameraRig { camera: camera };
        // base
    }
}

// pub fn camera_move_free() {
//     let move_vec = camera.transform.rotation
//         * Vec3::new(input["move_right"], input["move_up"], -input["move_fwd"])
//             .clamp_length_max(1.0)
//         * 10.0f32.powf(input["boost"]);

//     camera
//         .driver_mut::<YawPitch>()
//         .rotate_yaw_pitch(-0.3 * mouse.delta.x, -0.3 * mouse.delta.y);
//     camera
//         .driver_mut::<Position>()
//         .translate(move_vec * time_delta_seconds * 10.0);
//     camera.update(time_delta_seconds);
// }
