## WASM Build

### Requirement: WASM Target in Nix Toolchain
The Nix flake SHALL provide the `wasm32-unknown-unknown` Rust target alongside the existing native target. The `trunk` and `wasm-bindgen-cli` packages SHALL be included in the dev shell's `buildInputs`.

#### Scenario: WASM toolchain available in dev shell
- **WHEN** a developer enters the Nix dev shell
- **THEN** `rustup target list --installed` or equivalent SHALL show `wasm32-unknown-unknown`
- **THEN** `trunk --version` SHALL return a version string
- **THEN** `wasm-bindgen --version` SHALL return a version string

### Requirement: Trunk Builds the Game as WASM
Running `trunk build` in the project root SHALL produce a working WASM build in the `dist/` directory. The build output SHALL include the compiled `.wasm` file, JS glue code, and the HTML shell.

#### Scenario: Successful trunk build
- **WHEN** `trunk build` is run in the project root
- **THEN** the `dist/` directory SHALL be created
- **THEN** it SHALL contain an HTML file, a `.wasm` file, and JS glue code

#### Scenario: Trunk serve runs the game locally
- **WHEN** `trunk serve` is run in the project root
- **THEN** a local web server SHALL start
- **THEN** the game SHALL be playable in a browser at the served URL

### Requirement: HTML Shell for WASM
An `index.html` file SHALL exist in the project root. It SHALL contain a `<canvas>` element for Bevy to render into. The page background SHALL be dark to match the space theme.

#### Scenario: HTML shell provides canvas
- **WHEN** the WASM build loads in a browser
- **THEN** the game SHALL render into a full-viewport canvas
- **THEN** the page background SHALL be dark (black or near-black)

### Requirement: Native Build Unaffected
The existing native build via `cargo run` SHALL continue to work without changes. Adding WASM support MUST NOT break the native build.

#### Scenario: Native build still works
- **WHEN** `cargo run` is executed
- **THEN** the game SHALL compile and run as a native application
- **THEN** all existing features SHALL function identically
