# Proposal: Space Flight Prototype

## Summary

First playable prototype for a top-down 2D space shooter with Newtonian physics.
The player controls a spaceship in an open world, accelerating in any direction with
realistic inertia and drift. The world contains static structures (space stations,
asteroid formations) with openings that the player can fly through and explore.

No combat, no enemies, no UI — just the flight model and the world. The goal is to
nail the core feeling of flying a ship in space.

## Motivation

This is the foundation for everything else. If the flight physics feel good and the
world is fun to explore, the game works. If not, nothing else matters.

## What's Changing

This is a greenfield project. We're building from scratch using Rust and Bevy (latest stable).

### Development Environment

The project must build and run on NixOS. A `flake.nix` must be provided that includes:

- Rust toolchain (stable, via rust-overlay or fenix)
- All native dependencies required by Bevy (vulkan-loader, libxkbcommon, wayland, x11 libs, alsa-lib, udev)
- LD_LIBRARY_PATH configured so Bevy can find the graphics drivers at runtime
- A devShell that makes `cargo build` and `cargo run` work without any additional setup

## Core Mechanics

### Ship Physics

- The ship has position, velocity, and rotation
- Player input applies thrust in the direction the ship is facing (acceleration, not instant movement)
- The ship maintains momentum — releasing thrust means drifting, not stopping
- Rotation is independent of movement direction (the ship can face one way and drift another)
- A small amount of drag can be applied to keep the game playable (pure Newtonian physics with zero drag can feel frustrating)
- Optional: a "brake" input that applies counter-thrust to slow down

### Controls

- Rotate left/right: A/D or arrow keys
- Thrust forward: W or up arrow
- Brake/reverse thrust: S or down arrow
- The ship should feel responsive but not instant — there's a physicality to it

### World

- Open 2D space, larger than the screen
- Camera follows the ship smoothly
- Static structures made from simple geometry (rectangles, lines, polygons)
- Structures have walls (collision) and openings (gaps the player can fly through)
- At least 2-3 example structures: one small (a simple room with two openings), one medium (a station with corridors), one large (an asteroid field with narrow passages)

### Visuals (Prototype)

- All graphics are procedurally generated using Bevy's built-in shapes
- Ship: a triangle (or small arrow shape) with a color accent for the front
- Structures: rectangles and lines in a contrasting color
- Background: dark with scattered small dots/circles as stars
- Optional: a subtle parallax effect on the star background to enhance the sense of speed
- Optional: a faint thrust particle effect behind the ship when accelerating

## Out of Scope

- Combat / weapons
- Enemies / AI
- Health / damage system
- Inventory / upgrades
- Sound
- UI / HUD (no health bar, no minimap, no menus)
- Wormholes / level transitions
- Saving / loading

## Success Criteria

- The ship feels satisfying to fly — drift and inertia feel intentional, not sluggish
- The player can explore structures by flying through openings
- Collision with walls stops the ship without feeling jarring
- The world feels like a space you want to fly around in
