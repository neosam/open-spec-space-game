## ADDED Requirements

### Requirement: Optional CLI Seed Argument
The game SHALL accept an optional `--seed <value>` command-line argument where `<value>` is a 64-bit unsigned integer. When provided, this value MUST be used as the global world seed for all procedural generation. On WASM targets where CLI arguments are unavailable, the game SHALL skip argument parsing and fall through to random seed generation.

#### Scenario: Seed provided via CLI
- **WHEN** the game is launched with `--seed 42` on a native target
- **THEN** the world seed SHALL be set to 42
- **THEN** all chunk generation SHALL use 42 as the base seed

#### Scenario: Seed argument is optional
- **WHEN** the game is launched without a `--seed` argument
- **THEN** the game SHALL start successfully without error

#### Scenario: WASM target skips CLI parsing
- **WHEN** the game is running as a WASM target
- **THEN** the game SHALL NOT attempt to parse CLI arguments
- **THEN** the game SHALL fall through to random seed generation

### Requirement: Random Seed When Not Provided
When no `--seed` argument is provided, the game SHALL generate a random seed at startup. The random seed MUST be derived from a source of runtime entropy (e.g., system time).

#### Scenario: Random seed generated on startup
- **WHEN** the game is launched without a `--seed` argument
- **THEN** a random seed SHALL be generated
- **THEN** the generated seed SHALL be used for all chunk generation

#### Scenario: Different runs produce different worlds
- **WHEN** the game is launched twice without a `--seed` argument
- **THEN** the generated seeds SHOULD differ between runs (not guaranteed but expected)

### Requirement: Seed Logged at Startup
The game SHALL log the active world seed at startup so the player can note it for later reuse. The log message MUST include the seed value in a format that can be directly copied and passed back as a `--seed` argument.

#### Scenario: Seed is visible in log output
- **WHEN** the game starts with any seed (provided or generated)
- **THEN** a log message SHALL be printed containing the seed value
- **THEN** the logged value SHALL be usable as input to `--seed` to reproduce the same world

### Requirement: Reproducible Worlds from Same Seed
When the same seed is used across different runs, the world MUST be identical. Every chunk at every coordinate MUST generate the same structure type and layout.

#### Scenario: Same seed produces same world
- **WHEN** the game is launched with `--seed 12345`
- **THEN** chunk (0, 0) SHALL contain the same structure type and layout as any other run with `--seed 12345`
- **THEN** chunk (5, -3) SHALL contain the same structure type and layout as any other run with `--seed 12345`
