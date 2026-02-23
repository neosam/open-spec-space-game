use bevy::prelude::*;

use crate::ship::Ship;

pub struct CameraPlugin;

const CAMERA_SMOOTHING: f32 = 5.0;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow_system);
    }
}

#[derive(Component)]
struct GameCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, GameCamera));
}

fn camera_follow_system(
    time: Res<Time>,
    ship_query: Query<&Transform, (With<Ship>, Without<GameCamera>)>,
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Ship>)>,
) {
    let Ok(ship_transform) = ship_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    let target = ship_transform.translation.truncate();
    let current = camera_transform.translation.truncate();
    let t = (CAMERA_SMOOTHING * time.delta_secs()).min(1.0);
    let new_pos = current.lerp(target, t);
    camera_transform.translation.x = new_pos.x;
    camera_transform.translation.y = new_pos.y;
}
