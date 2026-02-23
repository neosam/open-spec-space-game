## ADDED Requirements

### Requirement: Deterministic Chunk Seeding
Each chunk's procedural content SHALL be determined by a seed derived from the combination of the world seed and the chunk's grid coordinates. The seed derivation MUST be deterministic such that the same world seed and chunk coordinates always produce the same seed value.

#### Scenario: Same coordinates produce same seed
- **WHEN** a chunk at coordinates (chunk_x, chunk_y) is generated with a given world seed
- **THEN** the derived chunk seed SHALL always be the same value for that combination of world seed and coordinates
- **THEN** generating the chunk multiple times SHALL produce identical results

#### Scenario: Different coordinates produce different seeds
- **WHEN** two chunks at different coordinates are generated with the same world seed
- **THEN** their derived seeds SHALL be different
- **THEN** their generated content SHALL generally differ

### Requirement: Probability-Based Structure Type Selection
Each chunk SHALL select a structure type based on a flat probability distribution using the chunk's deterministic seed. The probabilities MUST be approximately: 70% empty (no structure), 15% asteroid field, 10% room, and 5% station. The selection MUST be deterministic given the same seed.

#### Scenario: Empty chunks are the most common
- **WHEN** a large number of chunks are generated
- **THEN** approximately 70% of chunks SHALL contain no structure (empty)
- **THEN** the remaining 30% SHALL contain a generated structure

#### Scenario: Structure types follow probability distribution
- **WHEN** a large number of non-empty chunks are generated
- **THEN** approximately 50% of non-empty chunks SHALL be asteroid fields (~15% of all chunks)
- **THEN** approximately 33% of non-empty chunks SHALL be rooms (~10% of all chunks)
- **THEN** approximately 17% of non-empty chunks SHALL be stations (~5% of all chunks)

#### Scenario: Structure selection is deterministic
- **WHEN** a chunk is generated at given coordinates with a given world seed
- **THEN** the selected structure type SHALL always be the same for those inputs
- **THEN** regenerating the chunk SHALL produce the same structure type

### Requirement: Structure Containment Within Chunk
All generated structures MUST be fully contained within the boundaries of their chunk. Structures MUST be inset from the chunk edges by a margin so that no part of a structure extends beyond the chunk boundary. This ensures structures do not overlap across chunk boundaries.

#### Scenario: Structure does not exceed chunk boundaries
- **WHEN** a structure is generated within a chunk
- **THEN** all geometry of the structure SHALL be within the chunk's world-space boundaries
- **THEN** there SHALL be a margin between the structure geometry and the chunk edges

#### Scenario: Adjacent chunk structures do not overlap
- **WHEN** two adjacent chunks each contain a structure
- **THEN** the structures SHALL NOT overlap or intersect each other
- **THEN** the inset margin SHALL provide clear separation between structures in neighboring chunks

### Requirement: Deterministic Structure Placement Within Chunk
The position, size, and detailed layout of a structure within its chunk SHALL be determined by the chunk's seed. The same seed MUST always produce the same structure at the same position with the same geometry.

#### Scenario: Structure position is deterministic
- **WHEN** a chunk is generated with a given seed
- **THEN** the structure's position within the chunk SHALL always be the same for that seed
- **THEN** the structure's geometry and layout SHALL always be the same for that seed

#### Scenario: Regenerated chunk has identical structure placement
- **WHEN** a chunk is unloaded and then regenerated
- **THEN** the structure SHALL appear at the same position with the same geometry as before
