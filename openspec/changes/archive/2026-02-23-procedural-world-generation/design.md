## Context

The prototype has three hardcoded structures in `world.rs` spawned at startup. The flight
model works well but the world is exhausted in under a minute. We need an infinite,
lazily-generated world to give exploration meaning.

The current `visuals.rs` spawns 300 star entities at startup scattered over a fixed area.
These will be replaced with a camera-relative parallax system that works at any position.

The collision system (`collision_system`) queries all `Wall` entities. This continues to
work unchanged — it doesn't care where walls came from.

## Goals / Non-Goals

**Goals:**
- Infinite explorable world generated lazily as the player flies
- Deterministic generation: same seed + coordinates always produce the same content
- Reproducible worlds: seed accepted via CLI argument or randomly generated and logged
- No pop-in: chunks load before they become visible
- Clean memory: chunks unload when far from the player
- Two-layer parallax star background that works at any world position

**Non-Goals:**
- Density gradient based on distance (deferred — trivial to add later)
- Biomes or themed regions
- Persistent state across sessions (no saving chunk modifications)
- Cross-chunk structures
- Dynamic or moving structures

## Decisions

### 1. Chunk size: 1024x1024 world units

**Choice**: Each chunk covers a 1024x1024 area in world space.

**Rationale**: Large enough that structures (max ~400 units across) fit comfortably with
inset margins. Small enough that loading a 5x5 grid around the player (25 chunks) is cheap.
The current window shows roughly 1280x720 units, so one chunk is slightly smaller than the
viewport — meaning a 3x3 grid already covers the screen with margin.

**Alternatives considered**:
- *512x512*: More chunks visible at once, more frequent load/unload. Structures would feel
  cramped.
- *2048x2048*: Fewer chunks but each contains more entities. Unloading is coarser.

### 2. Load radius: 2 chunks beyond player chunk (5x5 grid)

**Choice**: Load all chunks within a Manhattan distance of 2 from the player's current chunk.
This creates a 5x5 diamond/grid of loaded chunks.

**Rationale**: The viewport is ~1280x720. With 1024-unit chunks, the player can see at most
parts of 4 chunks. Loading 2 beyond the player chunk means structures are generated at least
one full chunk width (~1024 units) before they could possibly be visible. This prevents
pop-in even at high velocities.

Unload radius: 3 chunks beyond player chunk. The gap between load (2) and unload (3)
prevents thrashing when the player sits near a chunk boundary.

### 3. World seed: CLI argument or random

**Choice**: The game accepts an optional `--seed <u64>` command-line argument. If not
provided, generate a random seed using `std::time` (e.g., low bits of system time in nanos).
Log the seed at startup: `"World seed: {seed}"` so the player can note it and reuse it.

Store the seed in a `WorldSeed(u64)` resource inserted during app startup.

**Rationale**: Players who find an interesting world can share or revisit it by passing the
same seed. Random default means every new run is different without requiring input.

### 4. Deterministic seeding: hash-based per-chunk RNG

**Choice**: Each chunk's RNG seed is computed as:
`seed = world_seed ^ (chunk_x as u64).wrapping_mul(PRIME_A) ^ (chunk_y as u64).wrapping_mul(PRIME_B)`

Use this seed to initialize a simple LCG (linear congruential generator) — the same
approach already used in `spawn_stars`. No external RNG crate needed.

**Rationale**: Deterministic, stateless, zero dependencies. The player can revisit any
coordinates and see the same structures. The world seed comes from the `WorldSeed` resource.

**Alternatives considered**:
- *`rand` crate with `StdRng::seed_from_u64`*: Cleaner API but adds a dependency for
  something trivial.
- *Hashing with `std::hash`*: Hasher trait is not guaranteed deterministic across Rust
  versions. Manual bit mixing is safer.

### 4. Chunk entity tracking: `ChunkCoord` component + `HashMap` resource

**Choice**: Store a `HashMap<(i32, i32), Vec<Entity>>` resource that maps chunk coordinates
to the entities spawned for that chunk. Tag each wall entity with a `ChunkCoord(i32, i32)`
component for queries.

**Rationale**: The HashMap enables O(1) lookup for "is this chunk loaded?" and provides
the entity list for bulk despawning. The component enables per-chunk queries if needed later.

### 5. Structure placement: centered in chunk with inset

**Choice**: Each structure is placed at the chunk center. Structures must fit within an
inset of 100 units from the chunk edges (effective placement area: 824x824).

**Rationale**: Centering is simple and guarantees structures never touch chunk boundaries.
The 100-unit inset provides visual breathing room — you see empty space before entering
a structure.

### 6. Structure generation probabilities

**Choice**: Flat distribution per chunk:
- 70% empty (no structures)
- 15% asteroid field
- 10% room
- 5% station

**Rationale**: Mostly empty makes discoveries feel meaningful. Asteroid fields are the
most common structure because they're visually sparse and feel like natural space debris.
Stations are rare — finding one should feel like an event.

### 7. Parallax stars: two entity layers with offset calculation

**Choice**: Spawn two grids of star entities that reposition themselves relative to the
camera each frame. Layer 1 (dim, slow) moves at 0.1x camera speed. Layer 2 (brighter,
faster) moves at 0.3x camera speed.

Each layer has a fixed set of star entities (~100 per layer) arranged in a grid pattern
larger than the viewport. As the camera moves, star positions wrap around using modulo
arithmetic so they tile infinitely.

**Rationale**: Using entities (not custom rendering) keeps us in Bevy's standard pipeline.
The wrapping trick means we only need ~200 star entities total regardless of world position.
Two layers at different speeds create a convincing depth illusion.

**Alternatives considered**:
- *Custom shader / material*: Most convincing parallax but requires writing WGSL shaders.
  Overkill for a prototype.
- *Spawning stars per chunk*: What we have now. Doesn't scale to infinite world — either
  too many entities or stars disappear with chunk unloading.

### 8. Project structure changes

```
src/
  world.rs    — Remove hardcoded spawning. Add:
                - ChunkCoord component
                - LoadedChunks resource (HashMap)
                - chunk_lifecycle_system (load/unload)
                - generate_chunk() (seeded RNG → structure selection → wall spawning)
                - spawn_room, spawn_station, spawn_asteroid_field (unchanged, now called by generate_chunk)
                - collision_system (unchanged)
  visuals.rs  — Remove spawn_stars. Add:
                - ParallaxStar component (with layer info)
                - spawn_parallax_stars (Startup — spawns fixed star entities for both layers)
                - parallax_update_system (Update — repositions stars relative to camera)
                - attach_wall_meshes changes from PostStartup to Update (walls now spawn at runtime)
  camera.rs   — No changes (camera position already accessible via query)
  ship.rs     — No changes
  main.rs     — No changes
```

## Risks / Trade-offs

- **[Collision scales with total loaded walls]** → With 25 chunks loaded and ~30% having
  structures, worst case is ~8 structures × ~12 walls = ~96 wall entities. The collision
  system iterates all walls per frame, which is fine at this scale. Would need spatial
  partitioning at 1000+ walls.
- **[LCG randomness quality]** → LCG has known patterns (low bits correlate). For structure
  placement this is fine — we're not doing cryptography. If visual patterns emerge, switch
  to a better hash.
- **[Chunk boundary jitter]** → Player hovering on a chunk boundary could cause rapid
  load/unload. The gap between load radius (2) and unload radius (3) prevents this.
- **[Star wrapping seams]** → If the star grid spacing doesn't divide evenly into the wrap
  distance, there could be visible seams. Tuning needed at implementation time.
