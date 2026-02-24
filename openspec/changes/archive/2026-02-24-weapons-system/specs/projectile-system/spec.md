## ADDED Requirements

### Requirement: Projectile Component
The system SHALL have a `Projectile` component with a `lifetime: f32` field representing remaining seconds before despawn. Projectile entities represent bullets fired by the ship.

#### Scenario: Projectile has lifetime at spawn
- **WHEN** a projectile is spawned
- **THEN** it SHALL have a `Projectile` component with a positive lifetime value

### Requirement: Projectile Spawning on Fire Input
When the ship fires, a new projectile entity SHALL be spawned at the ship's nose position (offset from ship center in the ship's facing direction). The projectile SHALL have a `Velocity` equal to the ship's current velocity plus a fixed projectile speed in the ship's facing direction.

#### Scenario: Projectile spawns at ship nose
- **WHEN** the player presses Space and the weapon cooldown has elapsed
- **THEN** a projectile entity SHALL be spawned
- **THEN** its position SHALL be offset from the ship center in the ship's facing direction
- **THEN** its velocity SHALL be the ship's velocity plus projectile speed in the facing direction

#### Scenario: Projectile inherits ship momentum
- **WHEN** the ship is moving forward at 100 units/sec and fires
- **THEN** the projectile's speed SHALL be greater than the base projectile speed (ship velocity + projectile speed)

### Requirement: Weapon Cooldown
The ship SHALL have a weapon cooldown that prevents firing faster than a defined rate. After firing, the weapon MUST NOT fire again until the cooldown period has elapsed.

#### Scenario: Cannot fire during cooldown
- **WHEN** the player fires a projectile
- **THEN** pressing Space again before the cooldown elapses SHALL NOT spawn another projectile

#### Scenario: Can fire after cooldown
- **WHEN** the cooldown period has fully elapsed since the last shot
- **THEN** pressing Space SHALL spawn a new projectile

### Requirement: Projectile Lifetime Despawn
Projectile entities SHALL despawn automatically when their lifetime reaches zero. The lifetime SHALL decrease each physics step by the elapsed delta time.

#### Scenario: Projectile despawns after timeout
- **WHEN** a projectile's lifetime reaches zero or below
- **THEN** the projectile entity SHALL be despawned

#### Scenario: Projectile survives while lifetime remains
- **WHEN** a projectile has remaining lifetime greater than zero
- **THEN** the projectile SHALL continue to exist and move

### Requirement: Projectile Visual Representation
Each projectile entity SHALL be rendered as a small shape (e.g., circle or small rectangle) in a bright, distinct color.

#### Scenario: Projectile has a mesh
- **WHEN** a projectile entity exists without a mesh
- **THEN** the visuals system SHALL attach a small mesh to it
- **THEN** the projectile's color SHALL be visually distinct from ship, wall, and asteroid colors
