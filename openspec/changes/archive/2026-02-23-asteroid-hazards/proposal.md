## Why

The world is static and unthreatening. Asteroid fields exist but are just immovable rectangles indistinguishable from walls. Adding moving asteroids with a health system introduces the first real hazard, making flight feel dangerous and giving the player something to avoid.

## What Changes

- Generalize `Velocity` component to move any entity, not just the ship
- Introduce `Asteroid` component separate from `Wall`
- Asteroids spawn with slow random drift velocities in asteroid field chunks
- Add `Health` component for ship and asteroids
- New collision matrix:
  - Ship ↔ Wall → stop (unchanged)
  - Ship ↔ Asteroid → speed-proportional damage to ship, bounce both
  - Asteroid ↔ Wall → bounce
  - Asteroid ↔ Asteroid → bounce
- Ship visual feedback: tint green → yellow → red as HP drops
- Health bar HUD in screen corner
- Ship at 0 HP: no game-over logic yet, HP just stays at zero

### Not in scope
- Weapons / destroying asteroids
- Game over / respawn
- Asteroid destruction visuals

## Capabilities

### New Capabilities
- `asteroid-movement`: Asteroid entities with drift velocity, separate from static walls
- `health-system`: Generic health and damage components, speed-proportional collision damage
- `health-visuals`: Ship color tint based on HP and health bar HUD

### Modified Capabilities
- `ship-physics`: Velocity and position integration generalized to all entities, not just ship

## Impact

- `src/ship.rs`: `position_integration_system` loses `With<Ship>` filter; `Velocity` becomes a general-purpose component
- `src/world.rs`: `spawn_asteroid_field_chunk` spawns `Asteroid` + `Velocity` instead of `Wall`; collision system rewritten to handle the full collision matrix
- `src/visuals.rs`: New systems for ship tint and health bar HUD; asteroid mesh attachment
- May introduce a new module (e.g., `src/health.rs`) for health/damage logic
