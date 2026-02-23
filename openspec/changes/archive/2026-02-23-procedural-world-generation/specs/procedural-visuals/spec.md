## MODIFIED Requirements

### Requirement: Star Background
The background SHALL be dark to represent space. The background SHALL contain scattered small dots or circles representing stars rendered as a camera-relative parallax system with two layers at different scroll speeds. Stars SHALL NOT be world-space entities that spawn and despawn; they SHALL be rendered relative to the camera and tile infinitely across the world. The stars MUST provide visual feedback of movement through the parallax effect, with dimmer stars scrolling slower and brighter stars scrolling faster.

#### Scenario: Background is dark with parallax stars
- **WHEN** the game is running and the camera is at any position
- **THEN** the background SHALL appear dark (near-black)
- **THEN** small dots or circles representing stars SHALL be visible in the background
- **THEN** the stars SHALL be rendered as camera-relative parallax layers, not as world-space entities

#### Scenario: Stars provide sense of movement through parallax
- **WHEN** the ship is moving through space
- **THEN** the two star layers SHALL scroll at different speeds relative to the camera
- **THEN** the parallax effect SHALL provide visual feedback of movement and a sense of depth
- **THEN** stars SHALL be present at every position in the infinite world

## REMOVED Requirements

### Requirement: Optional Parallax Star Background

**Reason:** The parallax star background is no longer optional. It is now the default and only star rendering approach, implemented as a two-layer camera-relative system. The full specification of the parallax behavior is covered by the new `parallax-stars` capability spec.

**Migration:** The parallax behavior described in this requirement is now mandatory and is specified in detail in the `parallax-stars` spec under the `procedural-world-generation` change. The `Star Background` requirement above has been updated to reference the parallax system as the required implementation.
