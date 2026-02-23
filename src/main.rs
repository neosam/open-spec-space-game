use bevy::prelude::*;

mod camera;
mod ship;
mod visuals;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ship::ShipPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(visuals::VisualsPlugin)
        .run();
}
