## 1. Generalize Velocity

- [x] 1.1 Remove `With<Ship>` filter from `position_integration_system` so any entity with `Velocity` + `Transform` moves
- [x] 1.2 Add tests verifying non-ship entities with `Velocity` are moved by the integration system

## 2. Health Component

- [x] 2.1 Create `src/health.rs` with `Health { current: f32, max: f32 }` component and `HealthPlugin`
- [x] 2.2 Add `Health` component to ship spawn (e.g., 100.0 max)
- [x] 2.3 Add damage helper that applies damage and clamps `current` to `[0.0, max]`
- [x] 2.4 Add tests for health clamping (no negative HP, no exceeding max)

## 3. Asteroid Component and Spawning

- [x] 3.1 Add `Asteroid { half_size: Vec2 }` component in `world.rs`
- [x] 3.2 Refactor `spawn_asteroid_field_chunk` to spawn `Asteroid` + `Velocity` + `Health` instead of `Wall`
- [x] 3.3 Generate random drift velocity (5–30 units/sec, random direction) from chunk RNG
- [x] 3.4 Add tests for asteroid spawn: has `Asteroid`, `Velocity`, `Health` components; no `Wall` component; deterministic per seed

## 4. Collision System Rewrite

- [x] 4.1 Refactor collision system to handle ship-vs-wall (stop, no damage — existing behavior)
- [x] 4.2 Add ship-vs-asteroid collision: bounce both, apply speed-proportional damage to ship
- [x] 4.3 Add asteroid-vs-wall collision: bounce asteroid
- [x] 4.4 Add asteroid-vs-asteroid collision: bounce both, no damage
- [x] 4.5 Add tests for each collision type: velocity reversal, separation, damage applied/not applied

## 5. Asteroid Visuals

- [x] 5.1 Add `attach_asteroid_meshes` system in `visuals.rs` (rectangle mesh, distinct color from walls)

## 6. Health Visuals

- [x] 6.1 Add ship tint system: interpolate ship mesh color green → yellow → red based on `Health.current / Health.max`
- [x] 6.2 Add health bar HUD: Bevy UI node in top-left corner, width proportional to HP, same color gradient
- [x] 6.3 Add tests for color interpolation logic (full HP → green, half → yellow, low → red)
