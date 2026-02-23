## ADDED Requirements

### Requirement: Procedurally Generated Graphics
All graphics in the prototype SHALL be generated procedurally using Bevy's built-in shape primitives. No external image assets or sprite sheets SHALL be required. All visual elements MUST be constructed from geometric shapes provided by the engine.

#### Scenario: No external image assets are used
- **WHEN** the game is built and run
- **THEN** all visual elements SHALL be rendered using engine-provided shape primitives
- **THEN** the game SHALL not load any external image or sprite files for gameplay graphics

### Requirement: Ship Visual Representation
The ship SHALL be rendered as a triangle or small arrow shape. The front of the ship MUST be visually distinguishable through a color accent or distinct shape so the player can determine the ship's facing direction at a glance.

#### Scenario: Ship is rendered as a triangle
- **WHEN** the ship is visible on screen
- **THEN** the ship SHALL appear as a triangle or arrow-like shape
- **THEN** the front of the ship SHALL have a distinct color or visual accent

#### Scenario: Ship facing direction is visually clear
- **WHEN** the ship rotates to any angle
- **THEN** the player SHALL be able to determine which direction the ship is facing based on the visual accent
- **THEN** the front of the ship SHALL be clearly distinguishable from the rear

### Requirement: Structure Visual Representation
Structures SHALL be rendered using rectangles and lines in a color that contrasts with the background. Structure visuals MUST clearly communicate which areas are walls (solid, impassable) and which areas are openings (gaps the ship can fly through).

#### Scenario: Structures are visually distinct from background
- **WHEN** structures are visible on screen
- **THEN** the structure geometry SHALL be rendered in a color that contrasts with the dark background
- **THEN** walls and openings SHALL be visually distinguishable

#### Scenario: Openings are visually identifiable
- **WHEN** a structure with openings is visible on screen
- **THEN** the gaps in the structure geometry SHALL be clearly visible as passable space
- **THEN** the player SHALL be able to identify openings by visual inspection

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

### Requirement: Optional Thrust Particle Effect
The ship MAY display a faint particle or visual effect behind it when thrust is being applied. This effect MUST only be visible while thrust input is active and SHALL disappear when thrust is released.

#### Scenario: Thrust effect appears during acceleration
- **WHEN** the player applies thrust input
- **THEN** a visual effect MAY appear at the rear of the ship
- **THEN** the effect SHALL be oriented opposite to the ship's facing direction

#### Scenario: Thrust effect disappears when thrust is released
- **WHEN** the player releases the thrust input
- **THEN** any thrust visual effect SHALL stop being generated
