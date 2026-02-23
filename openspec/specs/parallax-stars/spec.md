## ADDED Requirements

### Requirement: Camera-Relative Star Rendering
Stars SHALL be rendered relative to the camera position rather than as world-space entities that spawn and despawn with chunks. The star rendering system SHALL NOT create individual entities per star. Stars MUST remain visible at all times regardless of which chunks are loaded or unloaded.

#### Scenario: Stars are not world entities
- **WHEN** the game is running
- **THEN** stars SHALL be rendered relative to the camera's current position
- **THEN** stars SHALL NOT be spawned as individual world-space entities
- **THEN** stars SHALL NOT be affected by chunk loading or unloading

#### Scenario: Stars are always visible
- **WHEN** the camera is at any world position
- **THEN** background stars SHALL be visible on screen
- **THEN** there SHALL be no positions in the world where stars are absent

### Requirement: Two-Layer Parallax Effect
The star background SHALL consist of exactly two layers that scroll at different speeds relative to the camera movement to create a parallax depth effect. The first layer SHALL contain dimmer, smaller stars that scroll at a slower speed to represent distant stars. The second layer SHALL contain brighter, larger stars that scroll at a faster speed to represent closer stars.

#### Scenario: Two distinct star layers are visible
- **WHEN** the game is running and the background is visible
- **THEN** there SHALL be two visually distinct layers of stars
- **THEN** one layer SHALL have dimmer, smaller stars
- **THEN** the other layer SHALL have brighter, larger stars

#### Scenario: Layers scroll at different speeds
- **WHEN** the player moves the ship and the camera follows
- **THEN** the dim star layer SHALL scroll at a slower fraction of the camera movement
- **THEN** the bright star layer SHALL scroll at a faster fraction of the camera movement (but still slower than world-space objects)
- **THEN** the speed difference SHALL create a visual impression of depth

### Requirement: Infinite Star Tiling
The star pattern for each layer SHALL tile seamlessly so that stars appear at any camera position without visible seams, gaps, or repeating pattern edges. The tiling MUST work in all directions for an infinite world. The player SHALL NOT be able to perceive the tiling boundary during normal gameplay.

#### Scenario: No visible seams at tile boundaries
- **WHEN** the camera moves across a tile boundary in any direction
- **THEN** the star pattern SHALL continue seamlessly
- **THEN** no visible gap, line, or discontinuity SHALL appear at tile edges

#### Scenario: Stars are present at extreme world positions
- **WHEN** the player has traveled a very large distance from the world origin
- **THEN** stars SHALL still be visible and correctly rendered
- **THEN** the tiling SHALL continue to function without visual artifacts
