# Spacegame Roadmap

## Completed

### Space Flight Prototype
Core flight model with Newtonian physics, static structures with collision,
smooth camera follow, and procedural visuals using Bevy shapes.

## Next Up

### Procedural World Generation
Replace hardcoded structures with chunk-based lazy generation.

- Chunk grid (e.g., 1024x1024 units per chunk)
- Deterministic seeding: `hash(world_seed, chunk_x, chunk_y)`
- Flat probability distribution: ~70% empty, ~15% asteroid field, ~10% room, ~5% station
- Structures fully contained within their chunk (no cross-boundary)
- Load chunks within radius of player, unload distant chunks
- Parallax star background (2 layers at different speeds, camera-relative, replaces per-entity stars)

### Density Gradient
Make the world get denser the further from the origin the player explores.

- Adjust empty-chunk probability as a function of distance from origin
- One-line change on top of the chunk generation system
- Rewards exploration without changing architecture

## Future Ideas (Unexplored)

- Biomes / themed regions (asteroid belt, station cluster, open void)
- Unique landmark structures
- Moving obstacles (rotating asteroids, sliding doors)
- Collectibles / goals to give the player a reason to fly
- Sound (engine hum, thrust, collision)
- Combat / weapons
- Enemies / AI
- UI / HUD (minimap, speed indicator)
