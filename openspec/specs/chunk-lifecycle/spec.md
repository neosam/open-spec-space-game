## ADDED Requirements

### Requirement: Chunk Grid World Division
The game world SHALL be divided into a grid of fixed-size rectangular chunks. Each chunk SHALL be identified by its integer grid coordinates (chunk_x, chunk_y). The chunk size MUST be consistent across the entire world and MUST be large enough to contain at least one full structure with inset margins from chunk edges.

#### Scenario: World is divided into a uniform grid
- **WHEN** the game world is initialized
- **THEN** the world space SHALL be logically divided into a grid of equal-size chunks
- **THEN** each chunk SHALL be uniquely identified by its integer (chunk_x, chunk_y) coordinates
- **THEN** any world-space position SHALL map to exactly one chunk

### Requirement: Chunk Loading on Player Proximity
Chunks SHALL be loaded and their contents generated when the player is within a defined load radius. The load radius MUST be large enough that chunk generation always occurs outside the visible screen area so that the player never sees structures appear (no pop-in). The system SHALL check player proximity each frame and load any unloaded chunks that fall within the load radius.

#### Scenario: Chunks load before they become visible
- **WHEN** the player moves toward an unloaded chunk
- **WHEN** the player's distance to the chunk center is less than or equal to the load radius
- **THEN** the chunk SHALL be loaded and its contents generated
- **THEN** the load radius SHALL be large enough that the chunk is fully generated before any part of it enters the visible screen area

#### Scenario: Multiple chunks load as player moves
- **WHEN** the player moves through the world
- **THEN** all chunks within the load radius SHALL be loaded
- **THEN** the system SHALL evaluate which chunks need loading on every frame

### Requirement: Chunk Unloading on Player Distance
Chunks SHALL be unloaded and their entity contents despawned when the player exceeds the unload radius from the chunk. The unload radius MUST be strictly larger than the load radius to prevent thrashing at boundaries where a chunk would repeatedly load and unload as the player moves near the boundary.

#### Scenario: Chunks unload when player moves away
- **WHEN** the player moves away from a loaded chunk
- **WHEN** the player's distance to the chunk center exceeds the unload radius
- **THEN** all entities belonging to that chunk SHALL be despawned
- **THEN** the chunk SHALL be marked as unloaded

#### Scenario: Unload radius prevents boundary thrashing
- **WHEN** the player is near the load radius boundary of a chunk
- **THEN** the unload radius SHALL be far enough beyond the load radius that small player movements do not cause repeated load/unload cycles
- **THEN** the unload radius MUST be strictly greater than the load radius

### Requirement: Chunk State Tracking
The system SHALL track which chunks are currently loaded. A chunk MUST only be generated once while it remains loaded; duplicate generation of the same chunk SHALL NOT occur. When a chunk is unloaded and then re-entered, it SHALL be regenerated deterministically to produce the same content.

#### Scenario: No duplicate chunk generation
- **WHEN** a chunk is already loaded
- **WHEN** the player remains within the load radius of that chunk
- **THEN** the chunk SHALL NOT be generated again
- **THEN** the existing chunk entities SHALL remain unchanged

#### Scenario: Reloaded chunk produces identical content
- **WHEN** a chunk was previously loaded and then unloaded
- **WHEN** the player returns and the chunk is loaded again
- **THEN** the regenerated chunk content SHALL be identical to the original content
