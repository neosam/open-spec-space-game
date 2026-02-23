## MODIFIED Requirements

### Requirement: Position Integration
The position integration system SHALL update the position of ALL entities with `Velocity` and `Transform` components each physics step, not only the ship. Position SHALL be updated by adding velocity multiplied by the time delta. This integration MUST occur after all forces (thrust, drag, braking) have been applied for the current step.

#### Scenario: Position updates based on velocity
- **WHEN** the ship has position (0, 0) and velocity (60, 0)
- **WHEN** one physics step of 1/60th of a second elapses
- **THEN** the ship's position SHALL be approximately (1, 0)

#### Scenario: Non-ship entity with velocity moves
- **WHEN** an asteroid entity has position (500, 500) and velocity (10, 0)
- **WHEN** one physics step of 1/60th of a second elapses
- **THEN** the asteroid's position SHALL be approximately (500.17, 500)

#### Scenario: Entity without velocity does not move
- **WHEN** a wall entity has position (100, 100) and no `Velocity` component
- **WHEN** one physics step elapses
- **THEN** the wall's position SHALL remain (100, 100)
