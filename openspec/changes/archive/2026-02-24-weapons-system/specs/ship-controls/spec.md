## ADDED Requirements

### Requirement: Fire Input Binding
The ship SHALL fire a projectile when the player presses the Space bar. The fire input MUST be compatible with simultaneous thrust, brake, and rotation inputs.

#### Scenario: Space bar fires projectile
- **WHEN** the player presses the Space bar and the weapon cooldown has elapsed
- **THEN** the ship SHALL fire a projectile

#### Scenario: Fire while thrusting and rotating
- **WHEN** the player holds W, D, and presses Space simultaneously
- **THEN** the ship SHALL thrust, rotate, and fire without any input being suppressed
