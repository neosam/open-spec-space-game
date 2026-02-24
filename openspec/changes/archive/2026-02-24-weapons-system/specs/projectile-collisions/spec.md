## ADDED Requirements

### Requirement: Projectile-Asteroid Collision Deals Damage
When a projectile overlaps an asteroid (AABB collision), the asteroid SHALL take a fixed amount of damage and the projectile SHALL be despawned.

#### Scenario: Projectile hits asteroid
- **WHEN** a projectile's bounding box overlaps an asteroid's bounding box
- **THEN** the asteroid SHALL take a defined amount of damage
- **THEN** the projectile SHALL be despawned

#### Scenario: Projectile does not survive asteroid hit
- **WHEN** a projectile collides with an asteroid
- **THEN** the projectile SHALL be despawned immediately
- **THEN** only one asteroid SHALL take damage per projectile

### Requirement: Projectile-Wall Collision Despawns Projectile
When a projectile overlaps a wall (AABB collision), the projectile SHALL be despawned. The wall SHALL not be affected.

#### Scenario: Projectile hits wall
- **WHEN** a projectile's bounding box overlaps a wall's bounding box
- **THEN** the projectile SHALL be despawned
- **THEN** the wall SHALL not take damage or move

### Requirement: No Projectile-Ship Collision
Projectiles SHALL NOT collide with or damage the ship that fired them.

#### Scenario: Projectile passes through ship
- **WHEN** a projectile's bounding box overlaps the ship's bounding box
- **THEN** no collision response SHALL occur
- **THEN** no damage SHALL be applied to the ship
