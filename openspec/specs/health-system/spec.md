## Health System

### Requirement: Health Component
The system SHALL have a `Health` component with `current: f32` and `max: f32` fields. Any entity with `Health` can receive damage. `current` SHALL be clamped to the range `[0.0, max]`.

#### Scenario: Health is initialized at max
- **WHEN** an entity is spawned with `Health { current: 100.0, max: 100.0 }`
- **THEN** its current health SHALL equal its max health

#### Scenario: Health does not go below zero
- **WHEN** an entity with 10 HP current receives 50 damage
- **THEN** its current health SHALL be 0.0, not -40.0

### Requirement: Ship Has Health
The ship entity SHALL be spawned with a `Health` component. The ship's max HP SHALL be a defined constant (e.g., 100.0).

#### Scenario: Ship spawns with full health
- **WHEN** the ship is spawned
- **THEN** it SHALL have a `Health` component with `current` equal to `max`

### Requirement: Asteroids Have Health
Each asteroid entity SHALL be spawned with a `Health` component. Asteroid HP SHALL be high enough that they are effectively indestructible in this iteration (e.g., 9999.0).

#### Scenario: Asteroid spawns with health
- **WHEN** an asteroid is spawned in an asteroid field chunk
- **THEN** it SHALL have a `Health` component

### Requirement: Ship-Asteroid Collision Deals Damage
When the ship collides with an asteroid, the ship SHALL take damage proportional to the relative impact speed along the collision normal. The damage formula SHALL be `damage = relative_speed * damage_factor` where `damage_factor` is a tunable constant.

#### Scenario: Slow collision deals little damage
- **WHEN** the ship collides with an asteroid at low relative speed (e.g., 10 units/sec)
- **THEN** the ship SHALL take a small amount of damage

#### Scenario: Fast collision deals more damage
- **WHEN** the ship collides with an asteroid at high relative speed (e.g., 200 units/sec)
- **THEN** the ship SHALL take significantly more damage than a slow collision

#### Scenario: Stationary contact deals no damage
- **WHEN** the ship and asteroid are in contact but have zero relative velocity
- **THEN** no damage SHALL be applied

### Requirement: Ship-Asteroid Collision Bounces Both
When the ship collides with an asteroid, both entities SHALL bounce apart. The velocity component along the collision axis SHALL be reversed for both entities, and the entities SHALL be separated so they no longer overlap.

#### Scenario: Head-on collision reverses velocities
- **WHEN** the ship moving right at (100, 0) hits an asteroid moving left at (-10, 0)
- **THEN** after collision, the ship's X velocity SHALL be negative (bounced left)
- **THEN** after collision, the asteroid's X velocity SHALL be positive (bounced right)

### Requirement: Asteroid-Wall Collision Bounces Asteroid
When an asteroid collides with a wall, the asteroid SHALL bounce. The asteroid's velocity component along the collision axis SHALL be reversed. The wall SHALL remain unaffected.

#### Scenario: Asteroid bounces off wall
- **WHEN** an asteroid moving right at (20, 0) collides with a wall
- **THEN** the asteroid's X velocity SHALL become approximately (-20, 0)
- **THEN** the wall SHALL not move or change

### Requirement: Asteroid-Asteroid Collision Bounces Both
When two asteroids collide, both SHALL bounce apart. The velocity component along the collision axis SHALL be reversed for both. No damage is dealt between asteroids.

#### Scenario: Two asteroids bounce off each other
- **WHEN** asteroid A moving right and asteroid B moving left collide
- **THEN** both asteroids SHALL have their velocity along the collision axis reversed
- **THEN** neither asteroid SHALL take damage

### Requirement: Ship-Wall Collision Unchanged
Ship-wall collision behavior SHALL remain unchanged: the ship's velocity is zeroed on the collision axis and the ship is pushed out of the wall. No damage is dealt.

#### Scenario: Ship hits wall and stops
- **WHEN** the ship collides with a wall
- **THEN** the ship's velocity on the collision axis SHALL be zeroed
- **THEN** no damage SHALL be applied to the ship
