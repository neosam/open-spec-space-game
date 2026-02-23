use bevy::{color::palettes::css, prelude::*};

use crate::ship::{Ship, ShipInput};
use crate::world::Wall;

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_parallax_stars)
            .add_systems(PostStartup, attach_ship_mesh_startup)
            .add_systems(Update, (
                attach_wall_meshes,
                parallax_update_system,
                thrust_particle_system,
            ));
    }
}

// --- Ship visuals ---

fn attach_ship_mesh_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, (With<Ship>, Without<Mesh2d>)>,
) {
    for entity in &query {
        let triangle = Triangle2d::new(
            Vec2::new(16.0, 0.0),
            Vec2::new(-10.0, 10.0),
            Vec2::new(-10.0, -10.0),
        );
        commands.entity(entity).insert((
            Mesh2d(meshes.add(triangle)),
            MeshMaterial2d(materials.add(Color::from(css::LIMEGREEN))),
        ));

        let accent = Triangle2d::new(
            Vec2::new(16.0, 0.0),
            Vec2::new(8.0, 4.0),
            Vec2::new(8.0, -4.0),
        );
        commands.spawn((
            Mesh2d(meshes.add(accent)),
            MeshMaterial2d(materials.add(Color::from(css::WHITE))),
            Transform::from_xyz(0.0, 0.0, 1.1),
            ShipAccent,
        ));
    }
}

#[derive(Component)]
struct ShipAccent;

// --- Wall visuals (runs every frame for dynamically spawned walls) ---

fn attach_wall_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &Wall), Without<Mesh2d>>,
) {
    if query.is_empty() {
        return;
    }
    let wall_material = materials.add(Color::from(css::STEEL_BLUE));
    for (entity, wall) in &query {
        let rect = Rectangle::new(wall.half_size.x * 2.0, wall.half_size.y * 2.0);
        commands.entity(entity).insert((
            Mesh2d(meshes.add(rect)),
            MeshMaterial2d(wall_material.clone()),
        ));
    }
}

// --- Parallax star background ---

#[derive(Component)]
struct ParallaxStar {
    layer: u8,
    grid_offset: Vec2,
}

const STAR_GRID_SIZE: f32 = 2000.0;
const LAYER_1_PARALLAX: f32 = 0.1;
const LAYER_2_PARALLAX: f32 = 0.3;

fn spawn_parallax_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layer1_material = materials.add(Color::srgba(0.6, 0.6, 0.7, 0.3));
    let layer2_material = materials.add(Color::srgba(0.9, 0.9, 1.0, 0.6));
    let star_mesh = meshes.add(Circle::new(1.0));

    let mut seed: u64 = 98765;
    let next = |s: &mut u64| -> u64 {
        *s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
        *s
    };

    // Layer 1: dim, small, slow
    for _ in 0..100 {
        let x = ((next(&mut seed) >> 16) as i32 % (STAR_GRID_SIZE as i32)) as f32
            - STAR_GRID_SIZE / 2.0;
        let y = ((next(&mut seed) >> 16) as i32 % (STAR_GRID_SIZE as i32)) as f32
            - STAR_GRID_SIZE / 2.0;
        let size = 0.5 + (next(&mut seed) % 80) as f32 / 100.0;

        commands.spawn((
            ParallaxStar {
                layer: 1,
                grid_offset: Vec2::new(x, y),
            },
            Mesh2d(star_mesh.clone()),
            MeshMaterial2d(layer1_material.clone()),
            Transform::from_xyz(x, y, -2.0).with_scale(Vec3::splat(size)),
        ));
    }

    // Layer 2: brighter, larger, faster
    for _ in 0..100 {
        let x = ((next(&mut seed) >> 16) as i32 % (STAR_GRID_SIZE as i32)) as f32
            - STAR_GRID_SIZE / 2.0;
        let y = ((next(&mut seed) >> 16) as i32 % (STAR_GRID_SIZE as i32)) as f32
            - STAR_GRID_SIZE / 2.0;
        let size = 0.8 + (next(&mut seed) % 120) as f32 / 100.0;

        commands.spawn((
            ParallaxStar {
                layer: 2,
                grid_offset: Vec2::new(x, y),
            },
            Mesh2d(star_mesh.clone()),
            MeshMaterial2d(layer2_material.clone()),
            Transform::from_xyz(x, y, -1.5).with_scale(Vec3::splat(size)),
        ));
    }
}

fn parallax_update_system(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut star_query: Query<(&ParallaxStar, &mut Transform), Without<Camera2d>>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };
    let cam_pos = camera_transform.translation.truncate();

    for (star, mut transform) in &mut star_query {
        let parallax = match star.layer {
            1 => LAYER_1_PARALLAX,
            _ => LAYER_2_PARALLAX,
        };

        // Position = grid_offset + camera_pos * parallax, wrapped to tile infinitely
        let base = star.grid_offset - cam_pos * parallax;
        let wrapped_x = ((base.x % STAR_GRID_SIZE) + STAR_GRID_SIZE * 1.5) % STAR_GRID_SIZE
            - STAR_GRID_SIZE / 2.0;
        let wrapped_y = ((base.y % STAR_GRID_SIZE) + STAR_GRID_SIZE * 1.5) % STAR_GRID_SIZE
            - STAR_GRID_SIZE / 2.0;

        transform.translation.x = cam_pos.x + wrapped_x;
        transform.translation.y = cam_pos.y + wrapped_y;
    }
}

// --- Thrust particles ---

#[derive(Component)]
struct ThrustParticle {
    lifetime: f32,
}

fn thrust_particle_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    ship_query: Query<(&Transform, &ShipInput), With<Ship>>,
    mut particles: Query<(Entity, &mut ThrustParticle, &mut Transform), Without<Ship>>,
) {
    for (entity, mut particle, mut transform) in &mut particles {
        particle.lifetime -= time.delta_secs();
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            let alpha = particle.lifetime / 0.3;
            transform.scale = Vec3::splat(alpha * 3.0);
        }
    }

    for (ship_transform, input) in &ship_query {
        if input.thrust {
            let rotation = ship_transform.rotation.to_euler(EulerRot::ZYX).0;
            let behind = Vec2::new(-rotation.cos(), -rotation.sin()) * 14.0;
            let pos = ship_transform.translation.truncate() + behind;

            commands.spawn((
                ThrustParticle { lifetime: 0.3 },
                Mesh2d(meshes.add(Circle::new(2.0))),
                MeshMaterial2d(materials.add(Color::from(css::ORANGE_RED))),
                Transform::from_xyz(pos.x, pos.y, 0.5),
            ));
        }
    }
}
