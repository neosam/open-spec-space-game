## ADDED Requirements

### Requirement: Open 2D World Space
The game world SHALL be a 2D space that is significantly larger than the visible screen area. The world MUST allow the ship to fly in any direction without encountering invisible boundaries within the playable area.

#### Scenario: World extends beyond screen
- **WHEN** the game starts and the ship begins moving in any direction
- **THEN** the ship SHALL be able to travel a distance of many screen-widths in any direction
- **THEN** the world content SHALL extend beyond the initial visible area

### Requirement: Static Structures from Simple Geometry
Structures in the world SHALL be composed of simple geometric shapes including rectangles, lines, and polygons. Structures MUST be static and SHALL NOT move or change during gameplay. Each structure MUST be defined by its geometric components and their positions in world space.

#### Scenario: Structures are rendered as geometric shapes
- **WHEN** the player flies near a structure
- **THEN** the structure SHALL be visible as composed of rectangles, lines, or polygon shapes
- **THEN** the structure SHALL remain in its fixed position regardless of player actions

### Requirement: Wall Collision
Structure walls SHALL block the ship's movement. When the ship contacts a wall, the ship MUST be stopped and SHALL NOT pass through the wall geometry. The collision response MUST NOT feel jarring; the ship SHALL stop smoothly without abrupt bouncing or teleportation.

#### Scenario: Ship collides with a wall and stops
- **WHEN** the ship is moving toward a structure wall
- **WHEN** the ship's position reaches the wall boundary
- **THEN** the ship's velocity component perpendicular to the wall SHALL be set to zero or near-zero
- **THEN** the ship's position SHALL remain outside the wall geometry
- **THEN** the ship MUST NOT pass through the wall

#### Scenario: Collision does not feel jarring
- **WHEN** the ship collides with a wall at moderate speed
- **THEN** the ship SHALL decelerate smoothly upon contact
- **THEN** there SHALL be no visible teleportation or abrupt position snapping

### Requirement: Openings in Structures
Structures MUST have gaps or openings that the ship can fly through. Openings MUST be wide enough for the ship to pass through with reasonable piloting. The ship MUST be able to enter and exit structures through these openings without collision.

#### Scenario: Ship flies through an opening
- **WHEN** the ship approaches a structure opening while properly aligned
- **WHEN** the ship passes through the gap
- **THEN** no collision SHALL occur
- **THEN** the ship SHALL maintain its velocity through the opening

#### Scenario: Openings are navigable
- **WHEN** the ship approaches any structure opening
- **THEN** the opening width SHALL be at least 2 times the ship's collision width to allow reasonable passage

### Requirement: Small Structure - Simple Room
The world SHALL contain at least one small structure representing a simple room with two openings. This structure MUST have enclosed walls forming a room shape and exactly two or more openings that serve as entry and exit points.

#### Scenario: Small room structure exists
- **WHEN** the game world is loaded
- **THEN** a small room structure SHALL be present at a defined world position
- **THEN** the structure SHALL have walls forming an enclosed room shape
- **THEN** the structure SHALL have at least two openings

#### Scenario: Player can fly through the small room
- **WHEN** the ship enters through one opening of the small room
- **THEN** the ship SHALL be able to fly through the interior without collision (if not touching walls)
- **THEN** the ship SHALL be able to exit through another opening

### Requirement: Medium Structure - Station with Corridors
The world SHALL contain at least one medium-sized structure representing a space station with corridors. This structure MUST have multiple connected corridors with walls and openings that create navigable pathways.

#### Scenario: Station structure exists
- **WHEN** the game world is loaded
- **THEN** a medium station structure SHALL be present at a defined world position
- **THEN** the structure SHALL have multiple corridor segments connected together
- **THEN** the structure SHALL have openings allowing entry and exit

#### Scenario: Player can navigate through corridors
- **WHEN** the ship enters the station through an opening
- **THEN** the ship SHALL be able to navigate through the connected corridors
- **THEN** the corridors SHALL be wide enough for the ship to pass through with skillful piloting

### Requirement: Large Structure - Asteroid Field with Narrow Passages
The world SHALL contain at least one large structure representing an asteroid field with narrow passages. This structure MUST consist of multiple obstacles arranged to create narrow pathways that require careful navigation.

#### Scenario: Asteroid field structure exists
- **WHEN** the game world is loaded
- **THEN** a large asteroid field structure SHALL be present at a defined world position
- **THEN** the structure SHALL consist of multiple obstacle shapes arranged with gaps between them
- **THEN** the passages between obstacles SHALL be narrower than those in other structures

#### Scenario: Player can navigate narrow passages
- **WHEN** the ship enters the asteroid field
- **THEN** the ship SHALL be able to navigate through narrow passages between obstacles
- **THEN** collision with obstacle walls SHALL block the ship as with other structures
