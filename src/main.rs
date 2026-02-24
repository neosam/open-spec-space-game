use bevy::prelude::*;

mod camera;
mod health;
mod minimap;
mod ship;
mod visuals;
mod weapons;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(health::HealthPlugin)
        .add_plugins(ship::ShipPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(weapons::WeaponsPlugin)
        .add_plugins(minimap::MinimapPlugin)
        .add_plugins(visuals::VisualsPlugin)
        .run();
}
