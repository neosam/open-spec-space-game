## ADDED Requirements

### Requirement: Asteroid Component
The system SHALL have an `Asteroid` component with a `half_size: Vec2` field, distinct from the `Wall` component. Asteroid entities represent destructible, moving obstacles.

#### Scenario: Asteroid is not a Wall
- **WHEN** an asteroid entity is spawned
- **THEN** it SHALL have an `Asteroid` component
- **THEN** it SHALL NOT have a `Wall` component

### Requirement: Asteroids Spawn with Drift Velocity
Asteroid field chunks SHALL spawn `Asteroid` entities with a random drift `Velocity`. The drift speed SHALL be slow (approximately 5–30 units/sec) so asteroids feel environmental rather than aggressive. The drift direction SHALL be random, determined by the chunk's deterministic RNG.

#### Scenario: Asteroid has velocity at spawn
- **WHEN** an asteroid field chunk is generated
- **THEN** each asteroid entity SHALL have a `Velocity` component with a non-zero value
- **THEN** the velocity magnitude SHALL be between 5 and 30 units per second

#### Scenario: Asteroid drift is deterministic per chunk seed
- **WHEN** the same chunk coordinates are generated twice with the same world seed
- **THEN** the asteroid positions and velocities SHALL be identical

### Requirement: Asteroids Move Each Physics Step
Asteroid entities with `Velocity` and `Transform` SHALL have their position updated each physics step, using the same integration system as the ship.

#### Scenario: Asteroid drifts over time
- **WHEN** an asteroid has velocity (10, 0) and position (500, 500)
- **WHEN** one second elapses
- **THEN** the asteroid's position SHALL be approximately (510, 500)

### Requirement: Asteroid Visual Representation
Each asteroid entity SHALL be rendered as a rectangle using its `half_size`, visually distinct from walls.

#### Scenario: Asteroid has a mesh
- **WHEN** an asteroid entity exists without a mesh
- **THEN** the visuals system SHALL attach a rectangular mesh sized to `half_size * 2`
- **THEN** the asteroid's color SHALL be visually distinct from wall color
