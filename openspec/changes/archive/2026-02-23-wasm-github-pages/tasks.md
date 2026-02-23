## 1. Nix Toolchain

- [x] 1.1 Override `rustToolchain` in `flake.nix` to add `wasm32-unknown-unknown` target via `.override { targets = [...]; }`
- [x] 1.2 Add `trunk` and `wasm-bindgen-cli` to `buildInputs` in `flake.nix`
- [x] 1.3 Verify `trunk --version` and `wasm-bindgen --version` work in the dev shell

## 2. WASM Compatibility

- [x] 2.1 Wrap `std::env::args()` call in `src/world.rs` with `#[cfg(not(target_arch = "wasm32"))]` so WASM skips CLI parsing and falls through to random seed
- [x] 2.2 Verify native build still compiles and runs with `cargo run`
- [x] 2.3 Add test verifying seed fallback behavior (no args → random seed)

## 3. Trunk Setup

- [x] 3.1 Create `index.html` with full-viewport `<canvas>`, dark background, and basic styling
- [x] 3.2 Verify `trunk build` produces `dist/` with HTML, WASM, and JS glue
- [x] 3.3 Verify `trunk serve` serves the game in a browser

## 4. GitHub Actions Deployment

- [x] 4.1 Create `.github/workflows/deploy.yml` with workflow that triggers on push to main
- [x] 4.2 Workflow installs Rust stable + `wasm32-unknown-unknown` target + `trunk`
- [x] 4.3 Workflow runs `trunk build --release` and deploys `dist/` to GitHub Pages via `actions/deploy-pages`
