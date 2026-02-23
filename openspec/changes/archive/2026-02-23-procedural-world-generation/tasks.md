## 1. Chunk Infrastructure

- [x] 1.1 Add `ChunkCoord(i32, i32)` component to tag wall entities with their chunk
- [x] 1.2 Add `LoadedChunks` resource: `HashMap<(i32, i32), Vec<Entity>>` tracking loaded chunk entities
- [x] 1.3 Parse optional `--seed <u64>` CLI argument (e.g., via `std::env::args`); if not provided, generate random seed from system time
- [x] 1.4 Log the world seed at startup (`info!("World seed: {seed}")`)
- [x] 1.5 Insert `WorldSeed(u64)` resource into the app with the resolved seed value
- [x] 1.6 Define chunk constants: `CHUNK_SIZE` (1024.0), `LOAD_RADIUS` (2), `UNLOAD_RADIUS` (3)

## 2. Chunk Lifecycle System

- [x] 2.1 Implement `chunk_lifecycle_system`: compute player's current chunk from ship position
- [x] 2.2 Load logic: iterate chunks within load radius, call `generate_chunk` for any not in `LoadedChunks`
- [x] 2.3 Unload logic: iterate `LoadedChunks`, despawn all entities for chunks beyond unload radius, remove from map
- [x] 2.4 Add tests: chunk coordinate calculation from world position, load radius coverage, unload radius gap prevents thrashing

## 3. Chunk Generation

- [x] 3.1 Implement deterministic seed function: `chunk_seed(world_seed, chunk_x, chunk_y) -> u64`
- [x] 3.2 Implement `generate_chunk`: seed RNG, roll probability, select structure type (70% empty, 15% asteroid, 10% room, 5% station)
- [x] 3.3 Place structures centered in chunk with 100-unit inset from edges
- [x] 3.4 Tag all spawned wall entities with `ChunkCoord` and collect into entity list for `LoadedChunks`
- [x] 3.5 Add tests: same coordinates always produce same seed, probability thresholds select correct structure type, structures fit within chunk inset

## 4. Remove Hardcoded Structures

- [x] 4.1 Remove `spawn_structures` startup system and its call in `WorldPlugin::build`
- [x] 4.2 Register `chunk_lifecycle_system` in `WorldPlugin::build` (run in `Update` or `FixedUpdate`)
- [x] 4.3 Keep `spawn_room`, `spawn_station`, `spawn_asteroid_field` functions (now called by `generate_chunk`)
- [x] 4.4 Verify collision system still works unchanged with dynamically spawned walls

## 5. Parallax Star Background

- [x] 5.1 Add `ParallaxStar` component with layer index and grid offset
- [x] 5.2 Implement `spawn_parallax_stars` startup system: spawn ~100 star entities per layer (2 layers), with layer 1 dim/small and layer 2 brighter/larger
- [x] 5.3 Implement `parallax_update_system`: reposition star entities each frame based on camera position × parallax factor (layer 1: 0.1x, layer 2: 0.3x), wrap positions using modulo for infinite tiling
- [x] 5.4 Remove old `spawn_stars` function from `visuals.rs`

## 6. Visual Integration

- [x] 6.1 Change `attach_wall_meshes` from `PostStartup` to `Update` schedule (walls now spawn at runtime, need continuous mesh attachment)
- [x] 6.2 Verify wall meshes attach correctly to dynamically spawned walls (query for `Wall` without `Mesh2d`)

## 7. Testing and Verification

- [x] 7.1 Add test: chunks within load radius are generated before they could be visible on screen
- [x] 7.2 Add test: deterministic generation produces identical content for same chunk coordinates
- [x] 7.3 Verify no pop-in: structures appear smoothly as player flies in any direction
- [x] 7.4 Verify parallax stars tile seamlessly with no visible seams at any position
- [x] 7.5 Verify memory is stable: flying in one direction for extended time does not grow entity count
