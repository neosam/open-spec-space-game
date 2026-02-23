## Context

The spacegame currently has a flight prototype with Newtonian physics, chunk-based procedural world generation, and static structures (walls, rooms, stations, asteroid fields). Asteroid fields spawn 12 static rectangular `Wall` entities per chunk. The collision system handles ship-vs-wall only, zeroing the ship's velocity on contact. There is no health, damage, or entity movement beyond the ship.

The `Velocity` component and `position_integration_system` are currently scoped exclusively to `Ship` via a `With<Ship>` query filter.

## Goals / Non-Goals

**Goals:**
- Make asteroid fields feel dynamic by giving asteroids drift velocity
- Introduce a reusable health/damage system as foundation for future combat
- Provide visual feedback (ship tint + HUD bar) so the player can see their state
- Keep the collision system simple — AABB-based, no physics engine

**Non-Goals:**
- Weapons or ability to destroy asteroids
- Game over, respawn, or death screen
- Realistic rigid body physics (angular momentum, mass, restitution coefficients)
- Asteroid-asteroid damage

## Decisions

### 1. Generalize Velocity instead of creating AsteroidVelocity

**Decision**: Remove the `With<Ship>` filter from `position_integration_system` so any entity with `Velocity` + `Transform` moves.

**Alternatives considered**:
- Separate `AsteroidVelocity` component: duplicates logic, splits movement into two systems
- Bevy's built-in physics: overkill for AABB collision, adds heavy dependency

**Rationale**: `Velocity` is a generic concept. One system, one component, works for ship, asteroids, and any future moving entity.

### 2. Asteroid as a separate component from Wall

**Decision**: New `Asteroid { half_size: Vec2 }` component. Asteroid field chunks spawn `Asteroid` + `Velocity` entities instead of `Wall` entities.

**Rationale**: Asteroids have fundamentally different collision behavior (bounce + damage vs. stop). Separating them lets the collision system dispatch on component type cleanly. Walls remain static and indestructible.

### 3. Collision system rewrite with type-based dispatch

**Decision**: Replace the single ship-vs-wall collision system with a broader system that handles:
- Ship ↔ Wall → stop, zero velocity on collision axis (unchanged)
- Ship ↔ Asteroid → bounce both, apply speed-proportional damage to ship
- Asteroid ↔ Wall → bounce asteroid
- Asteroid ↔ Asteroid → bounce both

**Bounce model**: Simple velocity reflection on the minimum-overlap axis. No restitution coefficient — just reverse the velocity component on the collision axis and separate.

**Damage model**: `damage = impact_speed * damage_factor` where `impact_speed` is the relative speed between ship and asteroid along the collision normal. A `damage_factor` constant scales this to sensible HP values.

### 4. Health as a generic component

**Decision**: `Health { current: f32, max: f32 }` component. Attached to Ship (e.g., 100 HP) and Asteroids (for future use, high HP to make them effectively indestructible for now). Damage is applied by the collision system.

**Rationale**: Generic component is reusable. Putting HP on asteroids now means we don't need to restructure when weapons are added later.

### 5. Visual feedback: ship tint + HUD bar

**Decision**: Two visual systems:
- **Ship tint**: Interpolate ship mesh material color from green (full HP) → yellow (half) → red (low). Updated each frame based on `Health.current / Health.max`.
- **Health bar**: Bevy UI node anchored to top-left corner. Simple colored rectangle that shrinks as HP drops. Uses the same green → yellow → red gradient.

**Alternatives considered**:
- Screen shake: nice-to-have, adds complexity, can be added separately
- Numeric HP text: less readable at a glance than a color bar

### 6. Module organization

**Decision**: Add `src/health.rs` for `Health` component and damage-related systems. Collision logic stays in `world.rs` since it's tightly coupled to chunk/entity awareness. Health bar UI goes in `visuals.rs` alongside other visual systems.

## Risks / Trade-offs

- **Asteroids drifting out of chunks**: Asteroids spawned in a chunk may drift into neighboring chunks or beyond the unload radius. They'll be despawned when their parent chunk unloads, which is acceptable — the player won't notice distant asteroids vanishing. → No mitigation needed.

- **O(n²) collision checks**: With many asteroids, checking all pairs gets expensive. For now the asteroid count is low (12 per chunk, ~few hundred loaded). → Acceptable for current scale. Spatial partitioning can be added later if needed.

- **Velocity generalization side effects**: Removing `With<Ship>` means any entity accidentally given a `Velocity` will start moving. → Low risk since we control all entity spawning. Add `Velocity` only to entities that should move.

- **No invincibility frames**: Rapid repeated collisions could drain HP instantly. → Accept for now. Can add a brief damage cooldown later if it feels unfair.
