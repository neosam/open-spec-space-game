## ADDED Requirements

### Requirement: Ship Color Tint Based on Health
The ship's mesh color SHALL interpolate based on its current health ratio (`current / max`). At full health the color SHALL be green. At approximately half health the color SHALL transition toward yellow. At low health the color SHALL be red.

#### Scenario: Full health shows green
- **WHEN** the ship's health is at 100%
- **THEN** the ship mesh color SHALL be green (matching the current ship color)

#### Scenario: Half health shows yellow
- **WHEN** the ship's health is at approximately 50%
- **THEN** the ship mesh color SHALL be approximately yellow

#### Scenario: Low health shows red
- **WHEN** the ship's health is at approximately 10%
- **THEN** the ship mesh color SHALL be approximately red

#### Scenario: Tint updates in real time
- **WHEN** the ship takes damage during gameplay
- **THEN** the ship color SHALL update on the next frame to reflect the new health ratio

### Requirement: Health Bar HUD
The system SHALL display a health bar in the top-left corner of the screen using Bevy UI. The bar SHALL visually represent the ship's current health as a proportion of max health.

#### Scenario: Health bar at full health
- **WHEN** the ship has full health
- **THEN** the health bar SHALL be displayed at full width
- **THEN** the health bar color SHALL be green

#### Scenario: Health bar shrinks with damage
- **WHEN** the ship takes damage reducing health to 50%
- **THEN** the health bar width SHALL be approximately 50% of its full width

#### Scenario: Health bar color matches health ratio
- **WHEN** the ship's health changes
- **THEN** the health bar color SHALL follow the same green → yellow → red gradient as the ship tint

#### Scenario: Health bar visible at zero health
- **WHEN** the ship's health reaches zero
- **THEN** the health bar SHALL be displayed at zero or near-zero width
- **THEN** the health bar SHALL remain visible (not despawned)
