## ADDED Requirements

### Requirement: Nix Flake Configuration
The project SHALL provide a `flake.nix` file at the repository root that defines the complete build and development environment. The flake MUST be a valid Nix flake that can be consumed by `nix develop` to enter the development shell.

#### Scenario: Flake is valid and evaluable
- **WHEN** a developer runs `nix flake check` in the repository root
- **THEN** the flake SHALL evaluate without errors
- **THEN** the flake SHALL expose a devShell output

### Requirement: Rust Toolchain
The flake SHALL include a stable Rust toolchain provisioned via rust-overlay or fenix. The toolchain MUST include `rustc`, `cargo`, and `rust-std` at a minimum. The Rust version MUST be a stable release.

#### Scenario: Rust toolchain is available in devShell
- **WHEN** a developer enters the devShell via `nix develop`
- **THEN** `rustc --version` SHALL return a stable Rust version
- **THEN** `cargo --version` SHALL return a valid Cargo version

### Requirement: Bevy Native Dependencies
The flake SHALL include all native system dependencies required by the Bevy game engine. This MUST include at minimum: vulkan-loader, libxkbcommon, wayland libraries, X11 libraries, alsa-lib, and udev.

#### Scenario: All Bevy dependencies are present
- **WHEN** a developer enters the devShell via `nix develop`
- **WHEN** the developer runs `cargo build`
- **THEN** the build SHALL NOT fail due to missing native libraries
- **THEN** all Bevy-required system libraries SHALL be linkable

### Requirement: LD_LIBRARY_PATH Configuration
The devShell SHALL configure `LD_LIBRARY_PATH` to include the paths to all required shared libraries so that Bevy can find graphics drivers and other native dependencies at runtime. The `LD_LIBRARY_PATH` MUST include the path to vulkan-loader and all other required runtime libraries.

#### Scenario: Runtime libraries are discoverable
- **WHEN** a developer enters the devShell via `nix develop`
- **THEN** the `LD_LIBRARY_PATH` environment variable SHALL be set
- **THEN** `LD_LIBRARY_PATH` SHALL include the path to vulkan-loader's shared library directory
- **THEN** `LD_LIBRARY_PATH` SHALL include paths to wayland and X11 shared libraries

### Requirement: Zero-Setup Build and Run
The devShell MUST enable `cargo build` and `cargo run` to work without any additional manual setup steps. A developer on NixOS MUST be able to clone the repository, enter the devShell, and immediately build and run the project.

#### Scenario: cargo build succeeds in devShell
- **WHEN** a developer enters the devShell via `nix develop` on a fresh clone
- **WHEN** the developer runs `cargo build`
- **THEN** the build SHALL complete successfully with no errors

#### Scenario: cargo run launches the game
- **WHEN** a developer enters the devShell via `nix develop`
- **WHEN** the developer runs `cargo run`
- **THEN** the game application SHALL launch and display the game window
- **THEN** no errors related to missing libraries or drivers SHALL occur
