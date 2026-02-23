use std::collections::HashMap;

use bevy::prelude::*;

use crate::ship::{Ship, Velocity};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadedChunks>()
            .add_systems(Startup, init_world_seed)
            .add_systems(Update, chunk_lifecycle_system)
            .add_systems(
                FixedUpdate,
                collision_system.after(crate::ship::position_integration_system),
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
struct ChunkCoord(i32, i32);

// --- World seed initialization ---

fn init_world_seed(mut commands: Commands) {
    let args: Vec<String> = std::env::args().collect();
    let seed = parse_seed_from_args(&args).unwrap_or_else(|| {
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        t
    });
    info!("World seed: {seed}");
    commands.insert_resource(WorldSeed(seed));
}

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
        entities.push(spawn_wall_in_chunk(
            commands,
            center + Vec2::new(ox, oy),
            Vec2::new(hx, hy),
            chunk_x,
            chunk_y,
        ));
    }

    entities
}

// --- Collision (unchanged) ---

const SHIP_HALF_SIZE: Vec2 = Vec2::new(8.0, 8.0);

fn collision_system(
    mut ship_query: Query<(&mut Transform, &mut Velocity), With<Ship>>,
    wall_query: Query<(&Transform, &Wall), Without<Ship>>,
) {
    for (mut ship_transform, mut velocity) in &mut ship_query {
        let ship_pos = ship_transform.translation.truncate();

        for (wall_transform, wall) in &wall_query {
            let wall_pos = wall_transform.translation.truncate();

            let overlap_x =
                (SHIP_HALF_SIZE.x + wall.half_size.x) - (ship_pos.x - wall_pos.x).abs();
            let overlap_y =
                (SHIP_HALF_SIZE.y + wall.half_size.y) - (ship_pos.y - wall_pos.y).abs();

            if overlap_x > 0.0 && overlap_y > 0.0 {
                if overlap_x < overlap_y {
                    let sign = (ship_pos.x - wall_pos.x).signum();
                    ship_transform.translation.x =
                        wall_pos.x + sign * (wall.half_size.x + SHIP_HALF_SIZE.x);
                    velocity.0.x = 0.0;
                } else {
                    let sign = (ship_pos.y - wall_pos.y).signum();
                    ship_transform.translation.y =
                        wall_pos.y + sign * (wall.half_size.y + SHIP_HALF_SIZE.y);
                    velocity.0.y = 0.0;
                }
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
}
