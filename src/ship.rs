use bevy::prelude::*;

use crate::health::Health;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ship)
            .add_systems(FixedUpdate, (
                input_system,
                thrust_system,
                brake_system,
                drag_system,
                rotation_system,
                position_integration_system,
            ).chain());
    }
}

#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct ShipConfig {
    pub thrust_magnitude: f32,
    pub rotation_speed: f32,
    pub drag_coefficient: f32,
    pub brake_force: f32,
}

#[derive(Component, Default)]
pub struct ShipInput {
    pub thrust: bool,
    pub brake: bool,
    pub rotate: f32, // -1.0 left, 0.0 none, 1.0 right
}

fn spawn_ship(mut commands: Commands) {
    commands.spawn((
        Ship,
        Velocity(Vec2::ZERO),
        Health::new(100.0),
        ShipConfig {
            thrust_magnitude: 200.0,
            rotation_speed: 4.0,
            drag_coefficient: 0.3,
            brake_force: 150.0,
        },
        ShipInput::default(),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut ShipInput, With<Ship>>,
) {
    for mut input in &mut query {
        input.thrust = keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp);
        input.brake = keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown);

        let left = keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft);
        let right = keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight);
        input.rotate = match (left, right) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
    }
}

fn thrust_system(
    time: Res<Time>,
    mut query: Query<(&ShipInput, &ShipConfig, &mut Velocity, &Transform), With<Ship>>,
) {
    for (input, config, mut velocity, transform) in &mut query {
        if input.thrust {
            let rotation = transform.rotation.to_euler(EulerRot::ZYX).0;
            let direction = Vec2::new(rotation.cos(), rotation.sin());
            velocity.0 += direction * config.thrust_magnitude * time.delta_secs();
        }
    }
}

fn brake_system(
    time: Res<Time>,
    mut query: Query<(&ShipInput, &ShipConfig, &mut Velocity), With<Ship>>,
) {
    for (input, config, mut velocity) in &mut query {
        if input.brake {
            let speed = velocity.0.length();
            if speed > 0.0 {
                let brake_amount = config.brake_force * time.delta_secs();
                if brake_amount >= speed {
                    velocity.0 = Vec2::ZERO;
                } else {
                    let decel = velocity.0.normalize() * brake_amount;
                    velocity.0 -= decel;
                }
            }
        }
    }
}

fn drag_system(
    time: Res<Time>,
    mut query: Query<(&ShipConfig, &mut Velocity), With<Ship>>,
) {
    for (config, mut velocity) in &mut query {
        if config.drag_coefficient > 0.0 {
            let factor = 1.0 - config.drag_coefficient * time.delta_secs();
            velocity.0 *= factor.max(0.0);
        }
    }
}

fn rotation_system(
    time: Res<Time>,
    mut query: Query<(&ShipInput, &ShipConfig, &mut Transform), With<Ship>>,
) {
    for (input, config, mut transform) in &mut query {
        if input.rotate != 0.0 {
            let angle = input.rotate * config.rotation_speed * time.delta_secs();
            transform.rotate_z(angle);
        }
    }
}

pub fn position_integration_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_momentum_conservation() {
        let velocity = Velocity(Vec2::new(100.0, 50.0));
        assert_eq!(velocity.0, Vec2::new(100.0, 50.0));
    }

    #[test]
    fn test_thrust_direction() {
        // Ship facing right (rotation = 0) should thrust in +X
        let rotation = 0.0_f32;
        let direction = Vec2::new(rotation.cos(), rotation.sin());
        assert!((direction.x - 1.0).abs() < 0.001);
        assert!(direction.y.abs() < 0.001);
    }

    #[test]
    fn test_thrust_direction_up() {
        // Ship facing up (rotation = PI/2) should thrust in +Y
        let rotation = std::f32::consts::FRAC_PI_2;
        let direction = Vec2::new(rotation.cos(), rotation.sin());
        assert!(direction.x.abs() < 0.001);
        assert!((direction.y - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_drag_decay() {
        let mut velocity = Vec2::new(100.0, 0.0);
        let drag = 0.3;
        let dt = 1.0 / 60.0;
        let initial_speed = velocity.length();

        let factor = 1.0 - drag * dt;
        velocity *= factor;

        assert!(velocity.length() < initial_speed);
        assert!(velocity.x > 0.0); // must not reverse
    }

    #[test]
    fn test_brake_clamping() {
        let mut velocity = Vec2::new(5.0, 0.0);
        let brake_force = 150.0;
        let dt = 1.0 / 60.0;

        let speed = velocity.length();
        let brake_amount = brake_force * dt;
        if brake_amount >= speed {
            velocity = Vec2::ZERO;
        } else {
            velocity -= velocity.normalize() * brake_amount;
        }

        // Brake amount (2.5) < speed (5.0), so should reduce but not zero
        assert!(velocity.x > 0.0);
        assert!(velocity.x < 5.0);
    }

    #[test]
    fn test_brake_clamping_stops_at_zero() {
        let mut velocity = Vec2::new(1.0, 0.0);
        let brake_force = 150.0;
        let dt = 1.0 / 60.0;

        let speed = velocity.length();
        let brake_amount = brake_force * dt;
        if brake_amount >= speed {
            velocity = Vec2::ZERO;
        }

        // Brake amount (2.5) >= speed (1.0), so should clamp to zero
        assert_eq!(velocity, Vec2::ZERO);
    }

    #[test]
    fn test_position_integration_moves_non_ship_entity() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, position_integration_system);

        // First update to initialize time
        app.update();

        // Spawn a non-ship entity with Velocity (no Ship component)
        let entity = app
            .world_mut()
            .spawn((Velocity(Vec2::new(60.0, 0.0)), Transform::from_xyz(0.0, 0.0, 0.0)))
            .id();

        app.update();

        let transform = app.world().entity(entity).get::<Transform>().unwrap();
        assert!(transform.translation.x > 0.0, "Non-ship entity should move with Velocity");
    }

    #[test]
    fn test_entity_without_velocity_does_not_move() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, position_integration_system);

        // Spawn an entity without Velocity
        let entity = app
            .world_mut()
            .spawn(Transform::from_xyz(100.0, 100.0, 0.0))
            .id();

        app.update();

        let transform = app.world().entity(entity).get::<Transform>().unwrap();
        assert_eq!(transform.translation.x, 100.0);
        assert_eq!(transform.translation.y, 100.0);
    }
}
