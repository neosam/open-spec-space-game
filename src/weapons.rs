use bevy::prelude::*;

use crate::health::Health;
use crate::ship::{Ship, ShipInput, Velocity};
use crate::world::{Asteroid, Wall};

pub struct WeaponsPlugin;

const PROJECTILE_SPEED: f32 = 500.0;
const PROJECTILE_LIFETIME: f32 = 2.0;
const FIRE_COOLDOWN: f32 = 0.2;
const PROJECTILE_DAMAGE: f32 = 25.0;
const PROJECTILE_HALF_SIZE: Vec2 = Vec2::new(3.0, 3.0);
const SHIP_NOSE_OFFSET: f32 = 18.0;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                weapon_cooldown_system,
                fire_projectile_system,
                projectile_lifetime_system,
                projectile_asteroid_collision_system,
                projectile_wall_collision_system,
                zero_health_despawn_system,
            )
                .chain()
                .after(crate::ship::position_integration_system),
        );
    }
}

#[derive(Component)]
pub struct Projectile {
    pub lifetime: f32,
}

#[derive(Component)]
pub struct WeaponCooldown {
    pub remaining: f32,
}

fn weapon_cooldown_system(time: Res<Time>, mut query: Query<&mut WeaponCooldown>) {
    for mut cooldown in &mut query {
        cooldown.remaining = (cooldown.remaining - time.delta_secs()).max(0.0);
    }
}

fn fire_projectile_system(
    mut commands: Commands,
    mut query: Query<
        (&ShipInput, &mut WeaponCooldown, &Velocity, &Transform),
        With<Ship>,
    >,
) {
    for (input, mut cooldown, velocity, transform) in &mut query {
        if input.fire && cooldown.remaining <= 0.0 {
            let rotation = transform.rotation.to_euler(EulerRot::ZYX).0;
            let direction = Vec2::new(rotation.cos(), rotation.sin());
            let spawn_pos = transform.translation.truncate() + direction * SHIP_NOSE_OFFSET;
            let projectile_vel = velocity.0 + direction * PROJECTILE_SPEED;

            commands.spawn((
                Projectile {
                    lifetime: PROJECTILE_LIFETIME,
                },
                Velocity(projectile_vel),
                Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.5),
            ));

            cooldown.remaining = FIRE_COOLDOWN;
        }
    }
}

fn projectile_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile)>,
) {
    for (entity, mut projectile) in &mut query {
        projectile.lifetime -= time.delta_secs();
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn projectile_asteroid_collision_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    mut asteroid_query: Query<(&Transform, &Asteroid, &mut Health)>,
) {
    for (proj_entity, proj_transform) in &projectile_query {
        let proj_pos = proj_transform.translation.truncate();

        for (ast_transform, asteroid, mut health) in &mut asteroid_query {
            let ast_pos = ast_transform.translation.truncate();
            let (overlap_x, overlap_y) =
                crate::world::aabb_overlap(proj_pos, PROJECTILE_HALF_SIZE, ast_pos, asteroid.half_size);

            if overlap_x > 0.0 && overlap_y > 0.0 {
                health.apply_damage(PROJECTILE_DAMAGE);
                commands.entity(proj_entity).despawn();
                break; // Projectile is gone, stop checking
            }
        }
    }
}

fn projectile_wall_collision_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    wall_query: Query<(&Transform, &Wall)>,
) {
    for (proj_entity, proj_transform) in &projectile_query {
        let proj_pos = proj_transform.translation.truncate();

        for (wall_transform, wall) in &wall_query {
            let wall_pos = wall_transform.translation.truncate();
            let (overlap_x, overlap_y) =
                crate::world::aabb_overlap(proj_pos, PROJECTILE_HALF_SIZE, wall_pos, wall.half_size);

            if overlap_x > 0.0 && overlap_y > 0.0 {
                commands.entity(proj_entity).despawn();
                break;
            }
        }
    }
}

pub fn zero_health_despawn_system(
    mut commands: Commands,
    query: Query<(Entity, &Health), Without<Ship>>,
) {
    for (entity, health) in &query {
        if health.current <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projectile_spawns_on_fire() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, fire_projectile_system);

        // First update to init time
        app.update();

        app.world_mut().spawn((
            Ship,
            ShipInput { fire: true, ..default() },
            WeaponCooldown { remaining: 0.0 },
            Velocity(Vec2::ZERO),
            Health::new(100.0),
            Transform::from_xyz(0.0, 0.0, 1.0),
        ));

        app.update();

        let projectile_count = app
            .world_mut()
            .query_filtered::<&Projectile, ()>()
            .iter(app.world())
            .count();
        assert_eq!(projectile_count, 1, "Should spawn exactly one projectile");
    }

    #[test]
    fn test_cooldown_prevents_rapid_fire() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, fire_projectile_system);

        // First update to init time
        app.update();

        app.world_mut().spawn((
            Ship,
            ShipInput { fire: true, ..default() },
            WeaponCooldown { remaining: 1.0 }, // Cooldown active
            Velocity(Vec2::ZERO),
            Health::new(100.0),
            Transform::from_xyz(0.0, 0.0, 1.0),
        ));

        app.update();

        let projectile_count = app
            .world_mut()
            .query_filtered::<&Projectile, ()>()
            .iter(app.world())
            .count();
        assert_eq!(projectile_count, 0, "Should not fire during cooldown");
    }

    #[test]
    fn test_projectile_despawns_after_lifetime() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, projectile_lifetime_system);

        // First update to init time
        app.update();

        let entity = app
            .world_mut()
            .spawn((
                Projectile { lifetime: 0.0 }, // Already expired
                Velocity(Vec2::new(100.0, 0.0)),
                Transform::from_xyz(0.0, 0.0, 0.5),
            ))
            .id();

        app.update();

        assert!(
            app.world().get_entity(entity).is_err(),
            "Projectile should be despawned after lifetime expires"
        );
    }

    #[test]
    fn test_projectile_damages_asteroid_on_collision() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, projectile_asteroid_collision_system);

        // Projectile overlapping with asteroid
        app.world_mut().spawn((
            Projectile { lifetime: 2.0 },
            Velocity(Vec2::new(500.0, 0.0)),
            Transform::from_xyz(0.0, 0.0, 0.5),
        ));
        let asteroid = app
            .world_mut()
            .spawn((
                Asteroid { half_size: Vec2::new(20.0, 20.0) },
                Velocity(Vec2::ZERO),
                Health::new(100.0),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();

        app.update();

        let health = app.world().entity(asteroid).get::<Health>().unwrap();
        assert!(
            health.current < 100.0,
            "Asteroid should take damage, health={}",
            health.current
        );
    }

    #[test]
    fn test_projectile_despawns_on_wall_hit() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, projectile_wall_collision_system);

        // Projectile overlapping with wall
        let proj = app
            .world_mut()
            .spawn((
                Projectile { lifetime: 2.0 },
                Velocity(Vec2::new(500.0, 0.0)),
                Transform::from_xyz(0.0, 0.0, 0.5),
            ))
            .id();
        app.world_mut().spawn((
            Wall { half_size: Vec2::new(20.0, 20.0) },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        app.update();

        assert!(
            app.world().get_entity(proj).is_err(),
            "Projectile should be despawned on wall hit"
        );
    }

    #[test]
    fn test_zero_health_despawn() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, zero_health_despawn_system);

        let entity = app
            .world_mut()
            .spawn((
                Asteroid { half_size: Vec2::new(10.0, 10.0) },
                Velocity(Vec2::ZERO),
                Health::new(0.0), // Already at zero
                Transform::default(),
            ))
            .id();

        app.update();

        assert!(
            app.world().get_entity(entity).is_err(),
            "Entity with zero health should be despawned"
        );
    }

    #[test]
    fn test_positive_health_survives() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, zero_health_despawn_system);

        let entity = app
            .world_mut()
            .spawn((
                Asteroid { half_size: Vec2::new(10.0, 10.0) },
                Velocity(Vec2::ZERO),
                Health::new(50.0),
                Transform::default(),
            ))
            .id();

        app.update();

        assert!(
            app.world().get_entity(entity).is_ok(),
            "Entity with positive health should survive"
        );
    }

    #[test]
    fn test_weapon_cooldown_ticks_down() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, weapon_cooldown_system);

        // First update to init time
        app.update();

        let entity = app
            .world_mut()
            .spawn(WeaponCooldown { remaining: 1.0 })
            .id();

        app.update();

        let cooldown = app.world().entity(entity).get::<WeaponCooldown>().unwrap();
        assert!(
            cooldown.remaining < 1.0,
            "Cooldown should tick down, remaining={}",
            cooldown.remaining
        );
    }
}
