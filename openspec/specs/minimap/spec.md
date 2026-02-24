## Minimap

### Requirement: Minimap Panel
The system SHALL display a fixed-size opaque minimap panel in the bottom-right corner of the screen. The panel SHALL have a dark background with a visible border.

#### Scenario: Minimap is visible on screen
- **WHEN** the game is running
- **THEN** a minimap panel SHALL be visible in the bottom-right corner
- **THEN** the panel SHALL have a dark opaque background and a visible border

### Requirement: Ship Indicator
The ship SHALL always appear at the center of the minimap as a distinct colored indicator.

#### Scenario: Ship is centered on minimap
- **WHEN** the minimap is displayed
- **THEN** a ship indicator SHALL appear at the center of the minimap panel
- **THEN** the ship indicator color SHALL be distinct from asteroid and wall indicators

### Requirement: Asteroid Indicators
Asteroids within the minimap's world radius SHALL appear as colored dots on the minimap at positions corresponding to their world position relative to the ship.

#### Scenario: Nearby asteroid shown on minimap
- **WHEN** an asteroid is within the minimap's world radius of the ship
- **THEN** a colored indicator SHALL appear on the minimap at the asteroid's relative position

#### Scenario: Distant asteroid not shown
- **WHEN** an asteroid is beyond the minimap's world radius
- **THEN** no indicator SHALL appear for that asteroid on the minimap

### Requirement: Wall Indicators
Walls within the minimap's world radius SHALL appear as colored dots on the minimap at positions corresponding to their world position relative to the ship. Wall indicators SHALL be visually distinct from asteroid indicators.

#### Scenario: Nearby wall shown on minimap
- **WHEN** a wall is within the minimap's world radius of the ship
- **THEN** a colored indicator SHALL appear on the minimap at the wall's relative position
- **THEN** the wall indicator color SHALL be distinct from asteroid indicators

### Requirement: Minimap Updates Each Frame
The minimap indicator positions SHALL update every frame to reflect current world positions of entities relative to the ship.

#### Scenario: Indicators move as ship moves
- **WHEN** the ship moves through the world
- **THEN** entity indicators on the minimap SHALL shift to reflect the new relative positions

### Requirement: Minimap Scale
The minimap SHALL display a configurable world radius mapped to the panel size. Entities beyond this radius SHALL not have indicators on the minimap.

#### Scenario: Scale maps world to panel
- **WHEN** the minimap world radius is 2000 units and the panel is 160 pixels wide
- **THEN** an entity 1000 units to the right of the ship SHALL appear at 3/4 of the way from center to the right edge of the panel
