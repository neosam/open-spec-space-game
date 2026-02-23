## Context

The spacegame is a Bevy 0.18 2D game built with Rust 2024 edition on NixOS. It currently only builds for native Linux (`x86_64-unknown-linux-gnu`). The Nix flake uses `rust-overlay` for toolchain management. The game uses `std::env::args()` for seed parsing and `std::time::SystemTime` for random seed fallback — neither is available on `wasm32-unknown-unknown` (`SystemTime::now()` panics, `env::args()` is empty).

Bevy 0.18 has first-class WASM support via WebGL2/WebGPU. The game uses only 2D shapes (no external assets), making it well-suited for WASM deployment.

## Goals / Non-Goals

**Goals:**
- Build and serve the game as WASM locally with `trunk serve`
- Automated deployment to GitHub Pages on push to main
- Both native (`cargo run`) and WASM (`trunk serve`) builds work from the same codebase
- Nix flake provides all tooling needed for WASM development

**Non-Goals:**
- WASM-specific performance tuning or bundle size optimization
- Custom domain for GitHub Pages
- Mobile/touch input support
- Asset preloading or loading screens

## Decisions

### 1. Trunk as the WASM build tool

**Decision**: Use `trunk` for WASM builds.

**Alternatives considered**:
- Raw `cargo build --target wasm32-unknown-unknown` + `wasm-bindgen`: manual, requires scripting for JS glue and HTML assembly
- `wasm-pack`: designed for libraries/NPM packages, not full applications

**Rationale**: `trunk` is the Bevy-recommended WASM build tool. It handles compilation, `wasm-bindgen`, HTML shell, and dev server in one command. Well-supported in the Bevy ecosystem.

### 2. Nix toolchain override for WASM target

**Decision**: Override the existing `rust-bin.stable.latest.default` with `targets = [ "wasm32-unknown-unknown" ]` and add `trunk` + `wasm-bindgen-cli` to `buildInputs`.

**Rationale**: `rust-overlay`'s `.override` cleanly extends the toolchain without duplicating it. Providing `wasm-bindgen-cli` explicitly avoids trunk trying to download it at build time, which fails on NixOS due to sandboxing.

### 3. Conditional compilation for CLI args and time on WASM

**Decision**: Use `#[cfg(not(target_arch = "wasm32"))]` to skip `std::env::args()` parsing and `std::time::SystemTime` on WASM. On WASM, use `js_sys::Date::now()` for random seed generation instead. Added `js-sys` as a WASM-only dependency (`[target.'cfg(target_arch = "wasm32")'.dependencies]`).

**Alternatives considered**:
- URL query parameter parsing via `web-sys`: adds a dependency for a minor feature
- Hardcoded seed on WASM: loses the random-world experience
- `web-time` crate (drop-in `SystemTime` replacement): heavier dependency for just one call

**Rationale**: `js_sys::Date::now()` is lightweight (already in the dependency tree via Bevy's WASM support), returns milliseconds since epoch which works well as a seed. `std::time::SystemTime::now()` panics on `wasm32-unknown-unknown` — it is not supported on that platform despite compiling. Seed reproducibility via `--seed` remains available on native.

### 4. GitHub Actions with trunk for deployment

**Decision**: A GitHub Actions workflow that installs Rust + wasm32 target + trunk, runs `trunk build --release`, and deploys the `dist/` directory to GitHub Pages using the official `actions/deploy-pages` action.

**Rationale**: Standard approach for Bevy WASM projects. The workflow only needs Rust, the WASM target, and trunk — no Nix required in CI. This avoids the complexity of a full Nix build in CI while matching what trunk produces locally.

### 5. Minimal index.html

**Decision**: A simple `index.html` with a full-viewport `<canvas>`, dark background, and basic CSS for the page. No loading screen or splash.

**Rationale**: Matches the minimalist aesthetic. Trunk injects the WASM bundle script automatically. Can be enhanced later if needed.

## Risks / Trade-offs

- **Trunk version compatibility with wasm-bindgen-cli**: Trunk expects a specific `wasm-bindgen-cli` version matching its built-in expectations. If the nixpkgs version drifts, builds may fail. → Pin or verify compatibility; `trunk` in nixpkgs is usually kept in sync.

- **WASM binary size**: Bevy WASM builds can be large (10-30MB uncompressed). GitHub Pages serves with gzip, which helps significantly. → Accept for now. Can add `wasm-opt` later for optimization.

- **WebGL2 requirement**: Some older browsers don't support WebGL2. → Accept; most modern browsers support it. Bevy's WASM rendering requires it.

- **No seed input on WASM**: Players can't choose a seed in the browser. → Acceptable tradeoff for simplicity. Could add a URL parameter or UI input later.
