## Context

This is a greenfield Rust game project targeting NixOS. The codebase currently contains only
a hello-world `main.rs` and a basic `flake.nix`. We are building a 2D space flight prototype
using Bevy (latest stable) to validate the core flight model before investing in any other
game systems.

Bevy is an ECS (Entity Component System) game engine. All game state lives in components,
and behavior is defined by systems that query and mutate those components. This maps well
to the prototype's needs: physics, input, rendering, and camera are all independent systems
operating on shared component data.

The project must build and run on NixOS, which requires explicit configuration of native
library paths for Bevy's graphics and audio backends.

## Goals / Non-Goals

**Goals:**
- Implement a Newtonian flight model that feels satisfying (thrust, drift, rotation, drag)
- Create a scrollable 2D world with static structures the player can fly through
- Produce all visuals procedurally with Bevy's built-in shape primitives
- Ensure the project builds and runs on NixOS with `cargo run` inside the dev shell

**Non-Goals:**
- Combat, enemies, AI, health, damage, inventory, upgrades
- Sound or music
- UI/HUD of any kind
- Save/load, level transitions
- Production-quality art or assets
- Mobile or web platform support

## Decisions

### 1. Game engine: Bevy

**Choice**: Bevy (latest stable, 0.18.x)

**Rationale**: Bevy is the most actively developed Rust game engine with a mature ECS,
built-in 2D rendering, input handling, and transform hierarchy. It fits the prototype's
needs without pulling in a separate physics library.

**Alternatives considered**:
- *macroquad*: Simpler API, but no ECS — would require manual state management that becomes
  painful as the game grows.
- *ggez*: Mature but less actively maintained; weaker ecosystem for future expansion.

### 2. Physics: Hand-rolled Newtonian model (no physics engine)

**Choice**: Implement velocity/acceleration integration and AABB collision manually.

**Rationale**: The flight model is simple enough (one body, static obstacles) that a physics
engine adds complexity without benefit. We need direct control over drag, thrust feel, and
collision response to tune the "game feel." A full physics engine (rapier, avian) would
impose its own collision resolution and contact model, making it harder to get the exact
feel we want.

**Alternatives considered**:
- *bevy_rapier / avian2d*: Full rigid-body simulation. Overkill for static-obstacle
  collision and would fight us on custom drag/thrust behavior.

### 3. Collision: Axis-Aligned Bounding Boxes (AABB)

**Choice**: AABB collision between ship and structure walls.

**Rationale**: All structures are axis-aligned rectangles. AABB is trivial to implement,
fast, and sufficient for the prototype. The ship is small relative to corridors, so treating
it as a point or small AABB is accurate enough.

**Collision response**: On collision, zero the velocity component perpendicular to the wall
and push the ship out of overlap. This prevents tunneling and feels clean without bounce.

### 4. World structure definition: Hardcoded in code

**Choice**: Define the 2-3 example structures as constants/functions in a `world.rs` module.

**Rationale**: For a prototype with only a few structures, a data format (JSON, RON) adds
serialization complexity for no benefit. Hardcoding lets us iterate fastest. A data-driven
approach can be introduced later when there are more than a handful of structures.

### 5. Camera: Smooth lerp follow

**Choice**: Camera position lerps toward the ship position each frame.

**Rationale**: Instant camera snap feels jarring. A lerp factor (e.g., `camera_pos = lerp(camera_pos, ship_pos, 5.0 * dt)`) gives a smooth, slightly trailing feel that enhances
the sense of speed without disorienting the player.

### 6. Project structure

```
src/
  main.rs          — App setup, plugin registration
  ship.rs          — Ship components, physics system, input system
  world.rs         — Structure definitions, wall spawning
  camera.rs        — Camera follow system
  visuals.rs       — Ship mesh, structure meshes, star background
```

Each file is a Bevy plugin. `main.rs` adds them all to the App. This keeps things modular
without over-engineering (no trait abstractions, no generic systems).

### 7. NixOS dev environment

**Choice**: Update `flake.nix` with a devShell that provides:
- Rust stable toolchain (via rust-overlay or fenix)
- Native libraries: vulkan-loader, libxkbcommon, wayland, xorg libs, alsa-lib, udev
- `LD_LIBRARY_PATH` set to include all native library paths

**Rationale**: Bevy requires Vulkan (or wgpu-compatible backend) and several system libraries.
NixOS doesn't have `/usr/lib`, so these must be explicitly provided.

## Risks / Trade-offs

- **[AABB-only collision limits structure shapes]** → Acceptable for prototype. If we need
  angled walls later, we can upgrade to SAT or a physics engine.
- **[No physics engine means manual integration]** → Euler integration is sufficient at
  game-scale timesteps. If we see tunneling at high speeds, we can add substeps.
- **[Bevy version churn]** → Pin to a specific version in Cargo.toml. Migration cost is
  acceptable for a prototype.
- **[NixOS-specific flake may not work on other distros]** → Out of scope. The flake is
  for the developer's environment. Others can use standard cargo tooling.
- **[No fixed timestep]** → Using Bevy's `FixedUpdate` schedule for physics to avoid
  frame-rate-dependent behavior. This is built into Bevy.
