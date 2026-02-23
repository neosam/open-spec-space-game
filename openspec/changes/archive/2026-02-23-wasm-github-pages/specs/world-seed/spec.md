## MODIFIED Requirements

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
