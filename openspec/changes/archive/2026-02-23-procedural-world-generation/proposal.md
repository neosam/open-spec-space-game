## Why

The prototype world is hardcoded — three structures at fixed positions. The player explores
them in a minute and there's nothing left to discover. A procedurally generated infinite
world gives the flight model a reason to exist: there's always something new to find.

## What Changes

- Replace hardcoded structure spawning with a chunk-based lazy generation system
- Accept a global world seed as an optional command-line argument; if not provided, generate a random seed at startup and log it so the world can be reproduced
- Divide the world into a grid of chunks, each deterministically seeded from the global world seed and its coordinates
- Generate structures inside chunks on demand as the player approaches, with the load radius large enough that generation always happens outside the visible screen (no pop-in)
- Despawn chunk contents when the player flies far enough away (well beyond the visible screen)
- Most chunks are empty; structures appear with flat probability (~70% empty, ~15% asteroid field, ~10% room, ~5% station)
- Structures are fully contained within their chunk (no cross-boundary placement)
- Replace the current per-entity star background with a two-layer parallax star system that is camera-relative and tiles infinitely
- Remove the three hardcoded structures from `world.rs`

## Capabilities

### New Capabilities
- `chunk-lifecycle`: Chunk grid system — loading, generating, and unloading chunks based on player proximity
- `chunk-generation`: Deterministic procedural generation of structure content within a chunk (seeded RNG, probability-based structure type selection)
- `world-seed`: Global world seed — accepted as optional CLI argument or randomly generated at startup, logged for reproducibility
- `parallax-stars`: Two-layer parallax star background rendered relative to the camera, replacing per-entity star spawning

### Modified Capabilities
- `world-structures`: Structures are no longer hardcoded; they are generated per-chunk. The structure types (room, station, asteroid field) remain the same but are now placed by the generation system. Wall/collision behavior is unchanged.
- `procedural-visuals`: Star background changes from spawned entities to a camera-relative parallax system with two layers at different scroll speeds.

## Impact

- **`src/world.rs`**: Major rewrite — remove hardcoded spawning, add chunk grid, chunk lifecycle system, and generation logic
- **`src/visuals.rs`**: Replace `spawn_stars` with parallax star rendering
- **`src/camera.rs`**: May need to expose camera position for parallax calculation
- **No new dependencies** — deterministic hashing and seeded RNG can use standard library or Bevy's built-in RNG
- **No changes to ship physics, controls, or collision logic**
