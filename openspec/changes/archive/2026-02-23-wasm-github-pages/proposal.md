## Why

The game can only be played by building from source on a Linux machine. Adding a WASM build lets anyone play it in a browser, and deploying to GitHub Pages makes it shareable with a single link.

## What Changes

- Extend the Nix flake with `wasm32-unknown-unknown` target, `trunk`, and `wasm-bindgen-cli`
- Add `index.html` for trunk to build and serve the WASM app
- Handle `std::env::args()` gracefully on WASM (not available in browsers) using `cfg(target_arch)`
- Add a GitHub Actions workflow that builds with `trunk build --release` and deploys to GitHub Pages
- Both native and WASM builds remain supported

### Not in scope
- Changing game features or gameplay
- WASM-specific performance optimizations
- Custom domain setup for GitHub Pages

## Capabilities

### New Capabilities
- `wasm-build`: Nix toolchain setup, trunk config, and HTML shell for building and serving the game as WASM
- `github-pages-deploy`: GitHub Actions workflow for automated WASM build and deployment to GitHub Pages

### Modified Capabilities
- `world-seed`: Seed parsing via `std::env::args()` must degrade gracefully on WASM where CLI args are unavailable

## Impact

- `flake.nix`: Rust toolchain override for WASM target, new build tool dependencies
- `index.html`: New file, trunk entry point
- `src/world.rs`: Conditional compilation for seed arg parsing on WASM
- `.github/workflows/deploy.yml`: New file, CI/CD pipeline
- No changes to game logic, physics, visuals, or health systems
