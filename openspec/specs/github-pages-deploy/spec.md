## GitHub Pages Deploy

### Requirement: GitHub Actions Workflow for WASM Build
A GitHub Actions workflow SHALL exist that builds the game as WASM using `trunk build --release` on push to the main branch.

#### Scenario: Workflow triggers on push to main
- **WHEN** code is pushed to the `main` branch
- **THEN** the GitHub Actions workflow SHALL trigger
- **THEN** it SHALL build the WASM target using `trunk build --release`

### Requirement: Deploy to GitHub Pages
The GitHub Actions workflow SHALL deploy the `trunk build --release` output (`dist/` directory) to GitHub Pages using the official GitHub Pages deployment actions.

#### Scenario: Successful deployment
- **WHEN** the WASM build completes successfully in CI
- **THEN** the contents of `dist/` SHALL be deployed to GitHub Pages
- **THEN** the game SHALL be accessible at the repository's GitHub Pages URL

### Requirement: Workflow Installs Required Tooling
The GitHub Actions workflow SHALL install the Rust toolchain with `wasm32-unknown-unknown` target and `trunk` without requiring Nix in CI.

#### Scenario: CI environment setup
- **WHEN** the workflow runs
- **THEN** it SHALL install Rust stable with `wasm32-unknown-unknown` target
- **THEN** it SHALL install `trunk`
- **THEN** the build SHALL complete without missing tool errors
