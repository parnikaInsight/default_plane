use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_dolly::prelude::*;

mod math;
mod pcg_city;
mod camera;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyCursorGrab)
        .add_startup_system(camera::dolly_free::setup)
        .add_system(camera::dolly_free::update_camera)

        .init_resource::<math::city_perlin::HeightNoiseFn>()
        .add_system(pcg_city::buildings::spawn_buildings)

        .run();
  }
