## ADDED Requirements

### Requirement: Ship State Components
The ship entity SHALL have position, velocity, and rotation components that represent its state in 2D space. The position SHALL be a 2D vector representing world coordinates. The velocity SHALL be a 2D vector representing the current movement direction and speed. The rotation SHALL be a scalar representing the ship's facing angle in radians.

#### Scenario: Ship has initial state at spawn
- **WHEN** the ship entity is spawned into the world
- **THEN** the ship SHALL have a position component initialized to a defined spawn point
- **THEN** the ship SHALL have a velocity component initialized to zero
- **THEN** the ship SHALL have a rotation component initialized to a defined starting angle

### Requirement: Thrust Applies Acceleration in Facing Direction
When the player applies thrust, the ship SHALL accelerate in the direction it is currently facing. Thrust MUST NOT set velocity directly; it MUST add to the existing velocity vector based on the ship's current rotation and a defined thrust magnitude.

#### Scenario: Thrust increases velocity in facing direction
- **WHEN** the ship is stationary and facing to the right (rotation = 0)
- **WHEN** thrust input is applied for one physics step
- **THEN** the ship's velocity SHALL have a positive X component proportional to thrust magnitude
- **THEN** the ship's velocity Y component SHALL remain approximately zero

#### Scenario: Thrust adds to existing velocity
- **WHEN** the ship is already moving upward with velocity (0, 100)
- **WHEN** the ship is facing to the right (rotation = 0)
- **WHEN** thrust input is applied for one physics step
- **THEN** the ship's velocity SHALL have both a positive X component (from thrust) and a positive Y component (from prior momentum)

### Requirement: Momentum Conservation
The ship SHALL maintain its velocity when no thrust or braking input is applied. Releasing the thrust input MUST NOT cause the ship to stop; the ship MUST continue drifting at its current velocity, subject only to the optional drag force.

#### Scenario: Ship drifts after thrust is released
- **WHEN** the ship has velocity (100, 50) and no input is applied
- **WHEN** one physics step elapses (assuming zero drag)
- **THEN** the ship's velocity SHALL remain (100, 50)
- **THEN** the ship's position SHALL have changed by the velocity multiplied by the time step

### Requirement: Independent Rotation
Rotation MUST be independent of the movement direction. Changing the ship's rotation SHALL NOT alter the ship's velocity vector. The ship MUST be able to face one direction while drifting in a completely different direction.

#### Scenario: Rotating does not change velocity
- **WHEN** the ship has velocity (100, 0) and is facing right (rotation = 0)
- **WHEN** a rotation input is applied to turn the ship 90 degrees
- **THEN** the ship's velocity SHALL remain (100, 0)
- **THEN** the ship's rotation SHALL have changed by the rotation amount

### Requirement: Drag Force
A configurable drag coefficient SHALL be applied to the ship's velocity each physics step to keep the game playable. The drag MUST reduce the velocity magnitude gradually over time. The drag coefficient MUST be configurable and MAY be set to zero for pure Newtonian behavior.

#### Scenario: Drag gradually reduces velocity
- **WHEN** the ship has velocity (100, 0) and no input is applied
- **WHEN** drag coefficient is set to a positive value
- **WHEN** multiple physics steps elapse
- **THEN** the ship's velocity magnitude SHALL decrease each step
- **THEN** the velocity SHALL approach zero over time but MUST NOT reverse direction

#### Scenario: Zero drag preserves velocity indefinitely
- **WHEN** the ship has velocity (100, 0) and no input is applied
- **WHEN** drag coefficient is set to zero
- **WHEN** multiple physics steps elapse
- **THEN** the ship's velocity SHALL remain (100, 0)

### Requirement: Brake Input
The ship SHALL support a brake input that applies counter-thrust opposite to the ship's current velocity vector. The brake MUST decelerate the ship toward zero velocity. The brake MUST NOT cause the ship to reverse direction; it SHALL stop applying force once velocity reaches zero.

#### Scenario: Brake decelerates the ship
- **WHEN** the ship has velocity (100, 0)
- **WHEN** brake input is applied for one physics step
- **THEN** the ship's velocity X component SHALL be less than 100
- **THEN** the ship's velocity SHALL not have reversed to a negative X component within a single step

#### Scenario: Brake does not reverse direction
- **WHEN** the ship has a small velocity close to zero
- **WHEN** brake input is applied
- **THEN** the ship's velocity SHALL reach zero and MUST NOT overshoot into the opposite direction

### Requirement: Position Integration
The ship's position SHALL be updated each physics step by adding the velocity multiplied by the time delta. This integration MUST occur after all forces (thrust, drag, braking) have been applied to the velocity for the current step.

#### Scenario: Position updates based on velocity
- **WHEN** the ship has position (0, 0) and velocity (60, 0)
- **WHEN** one physics step of 1/60th of a second elapses
- **THEN** the ship's position SHALL be approximately (1, 0)
