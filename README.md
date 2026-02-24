# Spacegame

A 2D space flight game built with Rust and Bevy 0.18. Fly a ship through a procedurally generated universe, dodge asteroids, and shoot your way through.

**Play it now:** https://neosam.github.io/open-spec-space-game/

## Features

- Procedurally generated infinite world with deterministic seeding
- Chunk-based world streaming (rooms, stations, asteroid fields)
- Momentum-based ship physics with thrust, brake, and drag
- Drifting asteroids with bounce collision physics
- Weapons system with projectile shooting and cooldown
- Destructible asteroids (size-based health)
- Ship health system with visual feedback (color tint + health bar HUD)
- Minimap showing nearby asteroids and walls
- Parallax star background
- Runs natively and in the browser (WASM)

## Controls

| Key | Action |
|-----|--------|
| W / Arrow Up | Thrust forward |
| S / Arrow Down | Brake |
| A / Arrow Left | Rotate left |
| D / Arrow Right | Rotate right |
| Space | Fire |

## Building

### Prerequisites

This project uses [Nix](https://nixos.org/) for development environment management.

### Dev environment

```sh
nix develop
```

This provides Rust (with `wasm32-unknown-unknown` target), trunk, and wasm-bindgen-cli.

### Native build

```sh
nix develop --command cargo run
```

Optional: pass a world seed for deterministic generation:

```sh
nix develop --command cargo run -- --seed 42
```

### WASM build (local)

```sh
nix develop --command trunk serve
```

Opens at `http://127.0.0.1:8080`.

### WASM release build

```sh
nix develop --command trunk build --release
```

Output is in `dist/`.

## Deployment

Pushes to `main` automatically build and deploy to GitHub Pages via GitHub Actions.
