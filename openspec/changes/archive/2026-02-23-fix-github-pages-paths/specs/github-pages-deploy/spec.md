## MODIFIED Requirements

### Requirement: GitHub Actions Workflow for WASM Build
A GitHub Actions workflow SHALL exist that builds the game as WASM using `trunk build --release` on push to the main branch. The build command SHALL include `--public-url` with the repository's base path so that all generated asset URLs are correctly prefixed for GitHub Pages subpath hosting.

#### Scenario: Workflow triggers on push to main
- **WHEN** code is pushed to the `main` branch
- **THEN** the GitHub Actions workflow SHALL trigger
- **THEN** it SHALL detect the repository's base path using `actions/configure-pages`
- **THEN** it SHALL build the WASM target using `trunk build --release --public-url <base_path>/`

### Requirement: Deploy to GitHub Pages
The GitHub Actions workflow SHALL deploy the `trunk build --release` output (`dist/` directory) to GitHub Pages using the official GitHub Pages deployment actions.

#### Scenario: Successful deployment
- **WHEN** the WASM build completes successfully in CI
- **THEN** the contents of `dist/` SHALL be deployed to GitHub Pages
- **THEN** the game SHALL be accessible at the repository's GitHub Pages URL
- **THEN** all asset URLs (JS, WASM) SHALL resolve correctly under the repository subpath
