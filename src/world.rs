use std::collections::HashMap;

use bevy::prelude::*;

use crate::health::Health;
use crate::ship::{Ship, Velocity};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadedChunks>()
            .add_systems(Startup, init_world_seed)
            .add_systems(Update, chunk_lifecycle_system)
            .add_systems(
                FixedUpdate,
                (
                    ship_wall_collision_system,
                    ship_asteroid_collision_system,
                    asteroid_wall_collision_system,
                    asteroid_asteroid_collision_system,
                )
                    .after(crate::ship::position_integration_system),
            );
    }
}

// --- Constants ---

const CHUNK_SIZE: f32 = 1024.0;
const LOAD_RADIUS: i32 = 2;
const UNLOAD_RADIUS: i32 = 3;
const CHUNK_INSET: f32 = 100.0;

// Primes for seed mixing
const PRIME_A: u64 = 6364136223846793005;
const PRIME_B: u64 = 1442695040888963407;

// --- Resources ---

#[derive(Resource)]
pub struct WorldSeed(pub u64);

#[derive(Resource, Default)]
struct LoadedChunks(HashMap<(i32, i32), Vec<Entity>>);

// --- Components ---

#[derive(Component)]
pub struct Wall {
    pub half_size: Vec2,
}

#[derive(Component)]
pub struct Asteroid {
    pub half_size: Vec2,
}

#[derive(Component)]
struct ChunkCoord(i32, i32);

// --- World seed initialization ---

fn init_world_seed(mut commands: Commands) {
    #[cfg(not(target_arch = "wasm32"))]
    let cli_seed = {
        let args: Vec<String> = std::env::args().collect();
        parse_seed_from_args(&args)
    };
    #[cfg(target_arch = "wasm32")]
    let cli_seed: Option<u64> = None;

    let seed = cli_seed.unwrap_or_else(|| {
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64
        }
        #[cfg(target_arch = "wasm32")]
        {
            js_sys::Date::now() as u64
        }
    });
    info!("World seed: {seed}");
    commands.insert_resource(WorldSeed(seed));
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_seed_from_args(args: &[String]) -> Option<u64> {
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--seed" {
            return iter.next().and_then(|v| v.parse().ok());
        }
    }
    None
}

// --- Chunk coordinate helpers ---

fn world_to_chunk(x: f32, y: f32) -> (i32, i32) {
    (
        (x / CHUNK_SIZE).floor() as i32,
        (y / CHUNK_SIZE).floor() as i32,
    )
}

fn chunk_seed(world_seed: u64, chunk_x: i32, chunk_y: i32) -> u64 {
    world_seed
        ^ (chunk_x as u64).wrapping_mul(PRIME_A)
        ^ (chunk_y as u64).wrapping_mul(PRIME_B)
}

// --- Simple LCG for deterministic generation ---

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    fn next_f32(&mut self) -> f32 {
        (self.next() >> 33) as f32 / (1u64 << 31) as f32
    }
}

// --- Chunk lifecycle ---

fn chunk_lifecycle_system(
    mut commands: Commands,
    ship_query: Query<&Transform, With<Ship>>,
    world_seed: Option<Res<WorldSeed>>,
    mut loaded: ResMut<LoadedChunks>,
) {
    let Some(world_seed) = world_seed else {
        return;
    };
    let Ok(ship_transform) = ship_query.single() else {
        return;
    };

    let ship_pos = ship_transform.translation.truncate();
    let (cx, cy) = world_to_chunk(ship_pos.x, ship_pos.y);

    // Load chunks within load radius
    for dx in -LOAD_RADIUS..=LOAD_RADIUS {
        for dy in -LOAD_RADIUS..=LOAD_RADIUS {
            let coord = (cx + dx, cy + dy);
            if !loaded.0.contains_key(&coord) {
                let entities = generate_chunk(&mut commands, world_seed.0, coord.0, coord.1);
                loaded.0.insert(coord, entities);
            }
        }
    }

    // Unload chunks beyond unload radius
    let to_unload: Vec<(i32, i32)> = loaded
        .0
        .keys()
        .filter(|(x, y)| (x - cx).abs() > UNLOAD_RADIUS || (y - cy).abs() > UNLOAD_RADIUS)
        .copied()
        .collect();

    for coord in to_unload {
        if let Some(entities) = loaded.0.remove(&coord) {
            for entity in entities {
                commands.entity(entity).despawn();
            }
        }
    }
}

// --- Chunk generation ---

fn generate_chunk(commands: &mut Commands, world_seed: u64, chunk_x: i32, chunk_y: i32) -> Vec<Entity> {
    let seed = chunk_seed(world_seed, chunk_x, chunk_y);
    let mut rng = Lcg::new(seed);
    let roll = rng.next_f32();

    let chunk_center = Vec2::new(
        chunk_x as f32 * CHUNK_SIZE + CHUNK_SIZE / 2.0,
        chunk_y as f32 * CHUNK_SIZE + CHUNK_SIZE / 2.0,
    );

    if roll < 0.70 {
        // Empty chunk
        Vec::new()
    } else if roll < 0.85 {
        spawn_asteroid_field_chunk(commands, chunk_center, chunk_x, chunk_y, &mut rng)
    } else if roll < 0.95 {
        spawn_room_chunk(commands, chunk_center, chunk_x, chunk_y)
    } else {
        spawn_station_chunk(commands, chunk_center, chunk_x, chunk_y)
    }
}

// --- Structure spawning (chunk-aware) ---

fn spawn_wall_in_chunk(
    commands: &mut Commands,
    position: Vec2,
    half_size: Vec2,
    chunk_x: i32,
    chunk_y: i32,
) -> Entity {
    commands
        .spawn((
            Wall { half_size },
            ChunkCoord(chunk_x, chunk_y),
            Transform::from_xyz(position.x, position.y, 0.0),
        ))
        .id()
}

fn spawn_room_chunk(commands: &mut Commands, center: Vec2, chunk_x: i32, chunk_y: i32) -> Vec<Entity> {
    let wall_thickness = 10.0;
    let room_half = 100.0;
    let opening_half = 30.0;
    let mut entities = Vec::new();

    // Top wall
    entities.push(spawn_wall_in_chunk(commands, center + Vec2::new(0.0, room_half),
        Vec2::new(room_half, wall_thickness / 2.0), chunk_x, chunk_y));
    // Bottom wall
    entities.push(spawn_wall_in_chunk(commands, center + Vec2::new(0.0, -room_half),
        Vec2::new(room_half, wall_thickness / 2.0), chunk_x, chunk_y));

    // Left wall segments (with opening)
    let seg_height = (room_half - opening_half) / 2.0;
    entities.push(spawn_wall_in_chunk(commands,
        center + Vec2::new(-room_half, room_half - seg_height / 2.0 - wall_thickness / 2.0),
        Vec2::new(wall_thickness / 2.0, seg_height), chunk_x, chunk_y));
    entities.push(spawn_wall_in_chunk(commands,
        center + Vec2::new(-room_half, -(room_half - seg_height / 2.0 - wall_thickness / 2.0)),
        Vec2::new(wall_thickness / 2.0, seg_height), chunk_x, chunk_y));

    // Right wall segments (with opening)
    entities.push(spawn_wall_in_chunk(commands,
        center + Vec2::new(room_half, room_half - seg_height / 2.0 - wall_thickness / 2.0),
        Vec2::new(wall_thickness / 2.0, seg_height), chunk_x, chunk_y));
    entities.push(spawn_wall_in_chunk(commands,
        center + Vec2::new(room_half, -(room_half - seg_height / 2.0 - wall_thickness / 2.0)),
        Vec2::new(wall_thickness / 2.0, seg_height), chunk_x, chunk_y));

    entities
}

fn spawn_station_chunk(commands: &mut Commands, center: Vec2, chunk_x: i32, chunk_y: i32) -> Vec<Entity> {
    let wall_thickness = 10.0;
    let corridor_width = 80.0;
    let ht = wall_thickness / 2.0;
    let hw = corridor_width / 2.0;
    let mut entities = Vec::new();

    // Horizontal corridor
    entities.push(spawn_wall_in_chunk(commands, center + Vec2::new(0.0, hw),
        Vec2::new(200.0, ht), chunk_x, chunk_y));
    entities.push(spawn_wall_in_chunk(commands, center + Vec2::new(-100.0, -hw),
        Vec2::new(100.0, ht), chunk_x, chunk_y));
    entities.push(spawn_wall_in_chunk(commands, center + Vec2::new(140.0, -hw),
        Vec2::new(60.0, ht), chunk_x, chunk_y));

    // Vertical corridor
    let vert_start = center + Vec2::new(20.0, -hw);
    entities.push(spawn_wall_in_chunk(commands, vert_start + Vec2::new(-hw, -100.0),
        Vec2::new(ht, 100.0), chunk_x, chunk_y));
    entities.push(spawn_wall_in_chunk(commands, vert_start + Vec2::new(hw, -100.0),
        Vec2::new(ht, 100.0), chunk_x, chunk_y));
    entities.push(spawn_wall_in_chunk(commands,
        vert_start + Vec2::new(-hw / 2.0 - 10.0, -200.0),
        Vec2::new(hw / 2.0 - 10.0, ht), chunk_x, chunk_y));
    entities.push(spawn_wall_in_chunk(commands,
        vert_start + Vec2::new(hw / 2.0 + 10.0, -200.0),
        Vec2::new(hw / 2.0 - 10.0, ht), chunk_x, chunk_y));

    entities
}

const ASTEROID_MIN_SPEED: f32 = 5.0;
const ASTEROID_MAX_SPEED: f32 = 30.0;
const ASTEROID_HP_FACTOR: f32 = 0.1;

fn spawn_asteroid_field_chunk(
    commands: &mut Commands,
    center: Vec2,
    chunk_x: i32,
    chunk_y: i32,
    rng: &mut Lcg,
) -> Vec<Entity> {
    let mut entities = Vec::new();
    let max_offset = CHUNK_SIZE / 2.0 - CHUNK_INSET;

    for _ in 0..12 {
        let ox = (rng.next_f32() * 2.0 - 1.0) * max_offset;
        let oy = (rng.next_f32() * 2.0 - 1.0) * max_offset;
        let hx = 15.0 + rng.next_f32() * 30.0;
        let hy = 15.0 + rng.next_f32() * 30.0;

        // Random drift velocity
        let angle = rng.next_f32() * std::f32::consts::TAU;
        let speed = ASTEROID_MIN_SPEED + rng.next_f32() * (ASTEROID_MAX_SPEED - ASTEROID_MIN_SPEED);
        let drift = Vec2::new(angle.cos(), angle.sin()) * speed;

        let position = center + Vec2::new(ox, oy);
        let half_size = Vec2::new(hx, hy);

        let asteroid_hp = half_size.x * half_size.y * ASTEROID_HP_FACTOR;
        entities.push(
            commands
                .spawn((
                    Asteroid { half_size },
                    Velocity(drift),
                    Health::new(asteroid_hp),
                    ChunkCoord(chunk_x, chunk_y),
                    Transform::from_xyz(position.x, position.y, 0.0),
                ))
                .id(),
        );
    }

    entities
}

// --- Collision ---

const SHIP_HALF_SIZE: Vec2 = Vec2::new(8.0, 8.0);
const DAMAGE_FACTOR: f32 = 0.1;

/// Returns (overlap_x, overlap_y) for AABB overlap between two boxes.
/// Positive values mean overlap on that axis.
pub fn aabb_overlap(pos_a: Vec2, half_a: Vec2, pos_b: Vec2, half_b: Vec2) -> (f32, f32) {
    let overlap_x = (half_a.x + half_b.x) - (pos_a.x - pos_b.x).abs();
    let overlap_y = (half_a.y + half_b.y) - (pos_a.y - pos_b.y).abs();
    (overlap_x, overlap_y)
}

/// Separates entity A from static entity B on the minimum overlap axis.
/// Returns the axis: true = X axis, false = Y axis.
fn separate_from_static(
    transform_a: &mut Transform,
    vel_a: &mut Velocity,
    pos_b: Vec2,
    half_a: Vec2,
    half_b: Vec2,
    overlap_x: f32,
    overlap_y: f32,
) -> bool {
    let pos_a = transform_a.translation.truncate();
    if overlap_x < overlap_y {
        let sign = (pos_a.x - pos_b.x).signum();
        transform_a.translation.x = pos_b.x + sign * (half_b.x + half_a.x);
        vel_a.0.x = 0.0;
        true
    } else {
        let sign = (pos_a.y - pos_b.y).signum();
        transform_a.translation.y = pos_b.y + sign * (half_b.y + half_a.y);
        vel_a.0.y = 0.0;
        false
    }
}

/// Bounces entity A off static entity B on the minimum overlap axis.
fn bounce_off_static(
    transform_a: &mut Transform,
    vel_a: &mut Velocity,
    pos_b: Vec2,
    half_a: Vec2,
    half_b: Vec2,
    overlap_x: f32,
    overlap_y: f32,
) {
    let pos_a = transform_a.translation.truncate();
    if overlap_x < overlap_y {
        let sign = (pos_a.x - pos_b.x).signum();
        transform_a.translation.x = pos_b.x + sign * (half_b.x + half_a.x);
        vel_a.0.x = -vel_a.0.x;
    } else {
        let sign = (pos_a.y - pos_b.y).signum();
        transform_a.translation.y = pos_b.y + sign * (half_b.y + half_a.y);
        vel_a.0.y = -vel_a.0.y;
    }
}

// Ship ↔ Wall: stop (unchanged behavior)
fn ship_wall_collision_system(
    mut ship_query: Query<(&mut Transform, &mut Velocity), With<Ship>>,
    wall_query: Query<(&Transform, &Wall), (Without<Ship>, Without<Asteroid>)>,
) {
    for (mut ship_transform, mut velocity) in &mut ship_query {
        let ship_pos = ship_transform.translation.truncate();

        for (wall_transform, wall) in &wall_query {
            let wall_pos = wall_transform.translation.truncate();
            let (overlap_x, overlap_y) = aabb_overlap(ship_pos, SHIP_HALF_SIZE, wall_pos, wall.half_size);

            if overlap_x > 0.0 && overlap_y > 0.0 {
                separate_from_static(
                    &mut ship_transform, &mut velocity,
                    wall_pos, SHIP_HALF_SIZE, wall.half_size,
                    overlap_x, overlap_y,
                );
            }
        }
    }
}

// Ship ↔ Asteroid: bounce both, damage ship
fn ship_asteroid_collision_system(
    mut ship_query: Query<(&mut Transform, &mut Velocity, &mut Health), With<Ship>>,
    mut asteroid_query: Query<(&mut Transform, &mut Velocity, &Asteroid), Without<Ship>>,
) {
    for (mut ship_transform, mut ship_vel, mut ship_health) in &mut ship_query {
        let ship_pos = ship_transform.translation.truncate();

        for (mut ast_transform, mut ast_vel, asteroid) in &mut asteroid_query {
            let ast_pos = ast_transform.translation.truncate();
            let (overlap_x, overlap_y) = aabb_overlap(ship_pos, SHIP_HALF_SIZE, ast_pos, asteroid.half_size);

            if overlap_x > 0.0 && overlap_y > 0.0 {
                // Calculate relative speed for damage before bouncing
                let relative_vel = ship_vel.0 - ast_vel.0;
                let relative_speed = if overlap_x < overlap_y {
                    relative_vel.x.abs()
                } else {
                    relative_vel.y.abs()
                };

                let damage = relative_speed * DAMAGE_FACTOR;
                if damage > 0.0 {
                    ship_health.apply_damage(damage);
                }

                // Bounce both: swap velocity components on collision axis and separate
                let ship_pos_current = ship_transform.translation.truncate();
                let ast_pos_current = ast_transform.translation.truncate();
                if overlap_x < overlap_y {
                    let sign = (ship_pos_current.x - ast_pos_current.x).signum();
                    let total_half = SHIP_HALF_SIZE.x + asteroid.half_size.x;
                    let midpoint = (ship_pos_current.x + ast_pos_current.x) / 2.0;
                    ship_transform.translation.x = midpoint + sign * total_half / 2.0;
                    ast_transform.translation.x = midpoint - sign * total_half / 2.0;

                    std::mem::swap(&mut ship_vel.0.x, &mut ast_vel.0.x);
                } else {
                    let sign = (ship_pos_current.y - ast_pos_current.y).signum();
                    let total_half = SHIP_HALF_SIZE.y + asteroid.half_size.y;
                    let midpoint = (ship_pos_current.y + ast_pos_current.y) / 2.0;
                    ship_transform.translation.y = midpoint + sign * total_half / 2.0;
                    ast_transform.translation.y = midpoint - sign * total_half / 2.0;

                    std::mem::swap(&mut ship_vel.0.y, &mut ast_vel.0.y);
                }
            }
        }
    }
}

// Asteroid ↔ Wall: bounce asteroid
fn asteroid_wall_collision_system(
    mut asteroid_query: Query<(&mut Transform, &mut Velocity, &Asteroid), Without<Wall>>,
    wall_query: Query<(&Transform, &Wall), Without<Asteroid>>,
) {
    for (mut ast_transform, mut ast_vel, asteroid) in &mut asteroid_query {
        let ast_pos = ast_transform.translation.truncate();

        for (wall_transform, wall) in &wall_query {
            let wall_pos = wall_transform.translation.truncate();
            let (overlap_x, overlap_y) = aabb_overlap(ast_pos, asteroid.half_size, wall_pos, wall.half_size);

            if overlap_x > 0.0 && overlap_y > 0.0 {
                bounce_off_static(
                    &mut ast_transform, &mut ast_vel,
                    wall_pos, asteroid.half_size, wall.half_size,
                    overlap_x, overlap_y,
                );
            }
        }
    }
}

// Asteroid ↔ Asteroid: bounce both, no damage
fn asteroid_asteroid_collision_system(
    mut asteroid_query: Query<(Entity, &mut Transform, &mut Velocity, &Asteroid)>,
) {
    let mut pairs: Vec<(Entity, Entity)> = Vec::new();

    // Collect colliding pairs
    let combinations = asteroid_query.iter().collect::<Vec<_>>();
    for i in 0..combinations.len() {
        for j in (i + 1)..combinations.len() {
            let (_, t_a, _, a_a) = &combinations[i];
            let (_, t_b, _, a_b) = &combinations[j];
            let pos_a = t_a.translation.truncate();
            let pos_b = t_b.translation.truncate();
            let (overlap_x, overlap_y) = aabb_overlap(pos_a, a_a.half_size, pos_b, a_b.half_size);
            if overlap_x > 0.0 && overlap_y > 0.0 {
                pairs.push((combinations[i].0, combinations[j].0));
            }
        }
    }
    drop(combinations);

    // Resolve collisions
    for (entity_a, entity_b) in pairs {
        let Ok([mut a, mut b]) = asteroid_query.get_many_mut([entity_a, entity_b]) else {
            continue;
        };
        let pos_a = a.1.translation.truncate();
        let pos_b = b.1.translation.truncate();
        let (overlap_x, overlap_y) = aabb_overlap(pos_a, a.3.half_size, pos_b, b.3.half_size);

        if overlap_x > 0.0 && overlap_y > 0.0 {
            if overlap_x < overlap_y {
                let sign = (pos_a.x - pos_b.x).signum();
                let total_half = a.3.half_size.x + b.3.half_size.x;
                let midpoint = (pos_a.x + pos_b.x) / 2.0;
                a.1.translation.x = midpoint + sign * total_half / 2.0;
                b.1.translation.x = midpoint - sign * total_half / 2.0;
                std::mem::swap(&mut a.2.0.x, &mut b.2.0.x);
            } else {
                let sign = (pos_a.y - pos_b.y).signum();
                let total_half = a.3.half_size.y + b.3.half_size.y;
                let midpoint = (pos_a.y + pos_b.y) / 2.0;
                a.1.translation.y = midpoint + sign * total_half / 2.0;
                b.1.translation.y = midpoint - sign * total_half / 2.0;
                std::mem::swap(&mut a.2.0.y, &mut b.2.0.y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_to_chunk() {
        assert_eq!(world_to_chunk(0.0, 0.0), (0, 0));
        assert_eq!(world_to_chunk(1023.0, 0.0), (0, 0));
        assert_eq!(world_to_chunk(1024.0, 0.0), (1, 0));
        assert_eq!(world_to_chunk(-1.0, -1.0), (-1, -1));
        assert_eq!(world_to_chunk(2048.0, -1024.0), (2, -1));
    }

    #[test]
    fn test_chunk_seed_deterministic() {
        let seed_a = chunk_seed(42, 3, -7);
        let seed_b = chunk_seed(42, 3, -7);
        assert_eq!(seed_a, seed_b);
    }

    #[test]
    fn test_chunk_seed_different_coords() {
        let seed_a = chunk_seed(42, 0, 0);
        let seed_b = chunk_seed(42, 1, 0);
        let seed_c = chunk_seed(42, 0, 1);
        assert_ne!(seed_a, seed_b);
        assert_ne!(seed_a, seed_c);
        assert_ne!(seed_b, seed_c);
    }

    #[test]
    fn test_chunk_seed_different_world_seeds() {
        let seed_a = chunk_seed(1, 5, 5);
        let seed_b = chunk_seed(2, 5, 5);
        assert_ne!(seed_a, seed_b);
    }

    #[test]
    fn test_load_radius_covers_viewport() {
        // Viewport is ~1280x720. Chunk is 1024. Load radius of 2 means
        // 5 chunks loaded (±2), covering 5*1024 = 5120 units.
        // Player can see at most ~1280 units. Distance from player chunk edge
        // to load boundary is at least 1*1024 = 1024 units beyond viewport edge.
        let loaded_span = (LOAD_RADIUS * 2 + 1) as f32 * CHUNK_SIZE;
        let viewport_width = 1280.0;
        assert!(loaded_span > viewport_width * 2.0);
    }

    #[test]
    fn test_unload_radius_gap_prevents_thrashing() {
        assert!(UNLOAD_RADIUS > LOAD_RADIUS);
    }

    #[test]
    fn test_probability_thresholds() {
        let mut rng = Lcg::new(0);
        let mut empty: i32 = 0;
        let mut asteroid: i32 = 0;
        let mut room: i32 = 0;
        let mut station: i32 = 0;
        let n: i32 = 10000;
        for _ in 0..n {
            let roll = rng.next_f32();
            if roll < 0.70 {
                empty += 1;
            } else if roll < 0.85 {
                asteroid += 1;
            } else if roll < 0.95 {
                room += 1;
            } else {
                station += 1;
            }
        }
        // Allow 5% tolerance
        let tol = (n as f32 * 0.05) as i32;
        assert!((empty - 7000).abs() < tol, "empty: {empty}");
        assert!((asteroid - 1500).abs() < tol, "asteroid: {asteroid}");
        assert!((room - 1000).abs() < tol, "room: {room}");
        assert!((station - 500).abs() < tol, "station: {station}");
    }

    #[test]
    fn test_structures_fit_within_chunk_inset() {
        // Room is ±100 from center, chunk is 1024, inset is 100
        let max_structure_half = 200.0; // station is the largest (~200 units horizontal)
        let available_half = CHUNK_SIZE / 2.0 - CHUNK_INSET;
        assert!(max_structure_half < available_half);
    }

    #[test]
    fn test_parse_seed_from_args() {
        let args = vec![
            "spacegame".to_string(),
            "--seed".to_string(),
            "42".to_string(),
        ];
        assert_eq!(parse_seed_from_args(&args), Some(42));
    }

    #[test]
    fn test_parse_seed_missing() {
        let args = vec!["spacegame".to_string()];
        assert_eq!(parse_seed_from_args(&args), None);
    }

    #[test]
    fn test_parse_seed_invalid() {
        let args = vec![
            "spacegame".to_string(),
            "--seed".to_string(),
            "notanumber".to_string(),
        ];
        assert_eq!(parse_seed_from_args(&args), None);
    }

    #[test]
    fn test_seed_fallback_produces_nonzero() {
        // Simulates the WASM path: no CLI args → fall through to time-based seed
        let cli_seed: Option<u64> = None;
        let seed = cli_seed.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64
        });
        assert!(seed > 0, "Fallback seed should be nonzero");
    }

    #[test]
    fn test_aabb_overlap_detected() {
        let ship_pos = Vec2::new(50.0, 0.0);
        let wall_pos = Vec2::new(55.0, 0.0);
        let wall_half = Vec2::new(10.0, 10.0);

        let overlap_x = (SHIP_HALF_SIZE.x + wall_half.x) - (ship_pos.x - wall_pos.x).abs();
        let overlap_y = (SHIP_HALF_SIZE.y + wall_half.y) - (ship_pos.y - wall_pos.y).abs();

        assert!(overlap_x > 0.0);
        assert!(overlap_y > 0.0);
    }

    #[test]
    fn test_aabb_no_overlap() {
        let ship_pos = Vec2::new(0.0, 0.0);
        let wall_pos = Vec2::new(100.0, 0.0);
        let wall_half = Vec2::new(10.0, 10.0);

        let overlap_x = (SHIP_HALF_SIZE.x + wall_half.x) - (ship_pos.x - wall_pos.x).abs();
        assert!(overlap_x <= 0.0);
    }

    #[test]
    fn test_opening_pass_through() {
        let ship_pos = Vec2::new(0.0, 0.0);
        let wall1_pos = Vec2::new(0.0, 60.0);
        let wall1_half = Vec2::new(5.0, 20.0);
        let wall2_pos = Vec2::new(0.0, -60.0);
        let wall2_half = Vec2::new(5.0, 20.0);

        let overlap1_y =
            (SHIP_HALF_SIZE.y + wall1_half.y) - (ship_pos.y - wall1_pos.y).abs();
        let overlap2_y =
            (SHIP_HALF_SIZE.y + wall2_half.y) - (ship_pos.y - wall2_pos.y).abs();

        assert!(overlap1_y <= 0.0);
        assert!(overlap2_y <= 0.0);
    }

    #[test]
    fn test_asteroid_spawn_has_correct_components() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let center = Vec2::new(512.0, 512.0);
        let mut rng = Lcg::new(42);
        let entities = spawn_asteroid_field_chunk(&mut app.world_mut().commands(), center, 0, 0, &mut rng);
        app.world_mut().flush();

        assert_eq!(entities.len(), 12);
        for &entity in &entities {
            let world = app.world();
            let e = world.entity(entity);
            assert!(e.get::<Asteroid>().is_some(), "Asteroid component missing");
            assert!(e.get::<Velocity>().is_some(), "Velocity component missing");
            assert!(e.get::<Health>().is_some(), "Health component missing");
            assert!(e.get::<Wall>().is_none(), "Asteroid should not have Wall component");
        }
    }

    #[test]
    fn test_asteroid_drift_speed_in_range() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let center = Vec2::new(512.0, 512.0);
        let mut rng = Lcg::new(42);
        let entities = spawn_asteroid_field_chunk(&mut app.world_mut().commands(), center, 0, 0, &mut rng);
        app.world_mut().flush();

        for &entity in &entities {
            let velocity = app.world().entity(entity).get::<Velocity>().unwrap();
            let speed = velocity.0.length();
            assert!(
                speed >= ASTEROID_MIN_SPEED && speed <= ASTEROID_MAX_SPEED,
                "Asteroid speed {speed} out of range [{ASTEROID_MIN_SPEED}, {ASTEROID_MAX_SPEED}]"
            );
        }
    }

    #[test]
    fn test_asteroid_spawn_deterministic() {
        let mut app1 = App::new();
        app1.add_plugins(MinimalPlugins);
        let mut rng1 = Lcg::new(42);
        let entities1 = spawn_asteroid_field_chunk(&mut app1.world_mut().commands(), Vec2::ZERO, 0, 0, &mut rng1);
        app1.world_mut().flush();

        let mut app2 = App::new();
        app2.add_plugins(MinimalPlugins);
        let mut rng2 = Lcg::new(42);
        let entities2 = spawn_asteroid_field_chunk(&mut app2.world_mut().commands(), Vec2::ZERO, 0, 0, &mut rng2);
        app2.world_mut().flush();

        for (e1, e2) in entities1.iter().zip(entities2.iter()) {
            let t1 = app1.world().entity(*e1).get::<Transform>().unwrap().translation;
            let t2 = app2.world().entity(*e2).get::<Transform>().unwrap().translation;
            assert_eq!(t1, t2, "Positions should be deterministic");

            let v1 = app1.world().entity(*e1).get::<Velocity>().unwrap().0;
            let v2 = app2.world().entity(*e2).get::<Velocity>().unwrap().0;
            assert_eq!(v1, v2, "Velocities should be deterministic");
        }
    }

    #[test]
    fn test_aabb_overlap_helper() {
        let (ox, oy) = aabb_overlap(
            Vec2::new(0.0, 0.0), Vec2::new(8.0, 8.0),
            Vec2::new(10.0, 0.0), Vec2::new(8.0, 8.0),
        );
        assert!(ox > 0.0, "Should overlap on X");
        assert!(oy > 0.0, "Should overlap on Y");
    }

    #[test]
    fn test_aabb_no_overlap_helper() {
        let (ox, _oy) = aabb_overlap(
            Vec2::new(0.0, 0.0), Vec2::new(8.0, 8.0),
            Vec2::new(100.0, 0.0), Vec2::new(8.0, 8.0),
        );
        assert!(ox <= 0.0, "Should not overlap on X");
    }

    #[test]
    fn test_ship_wall_collision_stops_ship() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, ship_wall_collision_system);

        // Ship moving right
        app.world_mut().spawn((
            Ship,
            Velocity(Vec2::new(100.0, 0.0)),
            Health::new(100.0),
            Transform::from_xyz(5.0, 0.0, 0.0),
        ));
        // Wall to the right, overlapping
        app.world_mut().spawn((
            Wall { half_size: Vec2::new(10.0, 10.0) },
            Transform::from_xyz(15.0, 0.0, 0.0),
        ));

        app.update();

        let (vel, health) = app.world_mut()
            .query_filtered::<(&Velocity, &Health), With<Ship>>()
            .single(app.world())
            .unwrap();
        assert_eq!(vel.0.x, 0.0, "Ship X velocity should be zeroed");
        assert_eq!(health.current, 100.0, "Ship should not take damage from wall");
    }

    #[test]
    fn test_ship_asteroid_collision_bounces_and_damages() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, ship_asteroid_collision_system);

        // Ship moving right at speed 100
        app.world_mut().spawn((
            Ship,
            Velocity(Vec2::new(100.0, 0.0)),
            Health::new(100.0),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
        // Asteroid at x=10, moving left at speed 10
        app.world_mut().spawn((
            Asteroid { half_size: Vec2::new(15.0, 15.0) },
            Velocity(Vec2::new(-10.0, 0.0)),
            Health::new(9999.0),
            Transform::from_xyz(10.0, 0.0, 0.0),
        ));

        app.update();

        let (vel, health) = app.world_mut()
            .query_filtered::<(&Velocity, &Health), With<Ship>>()
            .single(app.world())
            .unwrap();
        // Ship should have bounced (velocity swapped with asteroid on X)
        assert!(vel.0.x < 0.0, "Ship should bounce left, got {}", vel.0.x);
        // Ship should have taken damage
        assert!(health.current < 100.0, "Ship should take damage, health={}", health.current);
    }

    #[test]
    fn test_ship_asteroid_no_damage_at_zero_relative_speed() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, ship_asteroid_collision_system);

        // Ship and asteroid moving at same velocity (zero relative speed)
        app.world_mut().spawn((
            Ship,
            Velocity(Vec2::new(50.0, 0.0)),
            Health::new(100.0),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
        app.world_mut().spawn((
            Asteroid { half_size: Vec2::new(15.0, 15.0) },
            Velocity(Vec2::new(50.0, 0.0)),
            Health::new(9999.0),
            Transform::from_xyz(10.0, 0.0, 0.0),
        ));

        app.update();

        let health = app.world_mut()
            .query_filtered::<&Health, With<Ship>>()
            .single(app.world())
            .unwrap();
        assert_eq!(health.current, 100.0, "No damage at zero relative speed");
    }

    #[test]
    fn test_asteroid_wall_collision_bounces() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, asteroid_wall_collision_system);

        // Asteroid moving right
        let ast_entity = app.world_mut().spawn((
            Asteroid { half_size: Vec2::new(15.0, 15.0) },
            Velocity(Vec2::new(20.0, 0.0)),
            Health::new(9999.0),
            Transform::from_xyz(0.0, 0.0, 0.0),
        )).id();
        // Wall to the right, overlapping
        app.world_mut().spawn((
            Wall { half_size: Vec2::new(10.0, 50.0) },
            Transform::from_xyz(20.0, 0.0, 0.0),
        ));

        app.update();

        let vel = app.world().entity(ast_entity).get::<Velocity>().unwrap();
        assert!(vel.0.x < 0.0, "Asteroid should bounce left, got {}", vel.0.x);
    }

    #[test]
    fn test_asteroid_asteroid_collision_bounces_no_damage() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, asteroid_asteroid_collision_system);

        // Two asteroids moving toward each other
        let a1 = app.world_mut().spawn((
            Asteroid { half_size: Vec2::new(15.0, 15.0) },
            Velocity(Vec2::new(20.0, 0.0)),
            Health::new(9999.0),
            Transform::from_xyz(0.0, 0.0, 0.0),
        )).id();
        let a2 = app.world_mut().spawn((
            Asteroid { half_size: Vec2::new(15.0, 15.0) },
            Velocity(Vec2::new(-20.0, 0.0)),
            Health::new(9999.0),
            Transform::from_xyz(20.0, 0.0, 0.0),
        )).id();

        app.update();

        let v1 = app.world().entity(a1).get::<Velocity>().unwrap();
        let v2 = app.world().entity(a2).get::<Velocity>().unwrap();
        // Velocities should have swapped (bounced)
        assert!(v1.0.x < 0.0, "Asteroid 1 should bounce left, got {}", v1.0.x);
        assert!(v2.0.x > 0.0, "Asteroid 2 should bounce right, got {}", v2.0.x);

        // No damage to either
        let h1 = app.world().entity(a1).get::<Health>().unwrap();
        let h2 = app.world().entity(a2).get::<Health>().unwrap();
        assert_eq!(h1.current, 9999.0);
        assert_eq!(h2.current, 9999.0);
    }
}
