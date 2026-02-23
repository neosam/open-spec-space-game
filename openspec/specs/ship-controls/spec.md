## ADDED Requirements

### Requirement: Rotate Left Input Binding
The ship SHALL rotate counter-clockwise when the player presses the A key or the Left Arrow key. Both key bindings MUST be active simultaneously and either key MUST produce the same rotation behavior.

#### Scenario: A key rotates ship left
- **WHEN** the player presses and holds the A key
- **THEN** the ship's rotation SHALL increase in the counter-clockwise direction each frame
- **THEN** the rotation rate SHALL be consistent and proportional to the time held

#### Scenario: Left Arrow key rotates ship left
- **WHEN** the player presses and holds the Left Arrow key
- **THEN** the ship's rotation SHALL increase in the counter-clockwise direction each frame
- **THEN** the behavior SHALL be identical to pressing the A key

### Requirement: Rotate Right Input Binding
The ship SHALL rotate clockwise when the player presses the D key or the Right Arrow key. Both key bindings MUST be active simultaneously and either key MUST produce the same rotation behavior.

#### Scenario: D key rotates ship right
- **WHEN** the player presses and holds the D key
- **THEN** the ship's rotation SHALL increase in the clockwise direction each frame
- **THEN** the rotation rate SHALL be consistent and proportional to the time held

#### Scenario: Right Arrow key rotates ship right
- **WHEN** the player presses and holds the Right Arrow key
- **THEN** the ship's rotation SHALL increase in the clockwise direction each frame
- **THEN** the behavior SHALL be identical to pressing the D key

### Requirement: Thrust Forward Input Binding
The ship SHALL apply forward thrust when the player presses the W key or the Up Arrow key. Thrust MUST be applied in the direction the ship is currently facing. Both key bindings MUST produce identical thrust behavior.

#### Scenario: W key applies forward thrust
- **WHEN** the player presses and holds the W key
- **THEN** the ship SHALL accelerate in its current facing direction
- **THEN** the acceleration SHALL continue for as long as the key is held

#### Scenario: Up Arrow key applies forward thrust
- **WHEN** the player presses and holds the Up Arrow key
- **THEN** the ship SHALL accelerate in its current facing direction
- **THEN** the behavior SHALL be identical to pressing the W key

#### Scenario: Releasing thrust key stops acceleration
- **WHEN** the player releases the W key or Up Arrow key
- **THEN** no further thrust SHALL be applied
- **THEN** the ship SHALL continue drifting at its current velocity

### Requirement: Brake Input Binding
The ship SHALL apply braking force when the player presses the S key or the Down Arrow key. The brake MUST apply counter-thrust opposing the ship's current velocity. Both key bindings MUST produce identical braking behavior.

#### Scenario: S key applies brake
- **WHEN** the ship is moving and the player presses and holds the S key
- **THEN** the ship SHALL decelerate toward zero velocity

#### Scenario: Down Arrow key applies brake
- **WHEN** the ship is moving and the player presses and holds the Down Arrow key
- **THEN** the ship SHALL decelerate toward zero velocity
- **THEN** the behavior SHALL be identical to pressing the S key

### Requirement: Responsive but Physical Controls
The ship controls SHALL feel responsive but MUST NOT feel instantaneous. Rotation and thrust SHALL have a defined rate that gives a sense of physicality. The rotation speed MUST be fast enough to feel responsive but slow enough that the player cannot instantly snap to any direction.

#### Scenario: Rotation has defined angular speed
- **WHEN** the player holds a rotation key for exactly one second
- **THEN** the ship SHALL have rotated by the defined rotation speed amount (in radians per second)
- **THEN** the rotation amount SHALL be consistent and predictable

#### Scenario: Thrust has defined acceleration rate
- **WHEN** the player holds the thrust key for exactly one second starting from rest
- **THEN** the ship's velocity magnitude SHALL equal the defined thrust acceleration rate
- **THEN** the acceleration SHALL feel responsive but not instantaneous

### Requirement: Simultaneous Input Handling
The input system MUST support multiple simultaneous key presses. The player MUST be able to thrust and rotate at the same time. The player MUST be able to brake and rotate at the same time.

#### Scenario: Thrust and rotate simultaneously
- **WHEN** the player holds W and D keys at the same time
- **THEN** the ship SHALL both accelerate forward and rotate clockwise simultaneously
- **THEN** neither input SHALL be ignored or suppressed

#### Scenario: Brake and rotate simultaneously
- **WHEN** the player holds S and A keys at the same time
- **THEN** the ship SHALL both decelerate and rotate counter-clockwise simultaneously
