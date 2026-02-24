## MODIFIED Requirements

### Requirement: Asteroids Have Health
Each asteroid entity SHALL be spawned with a `Health` component. Asteroid HP SHALL be proportional to the asteroid's size so that larger asteroids require more hits to destroy.

#### Scenario: Asteroid spawns with size-based health
- **WHEN** an asteroid is spawned in an asteroid field chunk
- **THEN** it SHALL have a `Health` component
- **THEN** its max HP SHALL be proportional to its size (larger asteroids have more HP)

## ADDED Requirements

### Requirement: Zero Health Despawn
Any entity whose `Health.current` reaches zero SHALL be despawned. This applies to asteroids destroyed by projectiles and any other damageable entity.

#### Scenario: Asteroid despawns at zero HP
- **WHEN** an asteroid's health reaches zero from projectile damage
- **THEN** the asteroid entity SHALL be despawned

#### Scenario: Entity with remaining health survives
- **WHEN** an entity has health greater than zero after taking damage
- **THEN** the entity SHALL NOT be despawned
