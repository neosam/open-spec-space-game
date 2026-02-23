## 1. NixOS Development Environment

- [x] 1.1 Update flake.nix with Rust stable toolchain (via rust-overlay or fenix)
- [x] 1.2 Add Bevy native dependencies to flake.nix (vulkan-loader, libxkbcommon, wayland, x11 libs, alsa-lib, udev)
- [x] 1.3 Configure LD_LIBRARY_PATH in devShell for runtime library discovery
- [x] 1.4 Verify `cargo build` and `cargo run` work inside the devShell

## 2. Bevy Project Scaffolding

- [x] 2.1 Add Bevy 0.18.x dependency to Cargo.toml and fix edition to valid Rust edition
- [x] 2.2 Replace hello-world main.rs with minimal Bevy app (window, default plugins, 2D camera)
- [x] 2.3 Set up module structure: ship.rs, world.rs, camera.rs, visuals.rs as Bevy plugins

## 3. Ship Physics

- [x] 3.1 Define ship components: position (Transform), velocity (Vec2), rotation, thrust magnitude, drag coefficient
- [x] 3.2 Implement thrust system: apply acceleration in facing direction when thrust input active
- [x] 3.3 Implement drag system: reduce velocity by configurable drag coefficient each FixedUpdate step
- [x] 3.4 Implement brake system: apply counter-thrust opposing current velocity, clamping to zero
- [x] 3.5 Implement position integration system: update transform from velocity each FixedUpdate step
- [x] 3.6 Implement rotation system: update ship rotation from rotation input at defined angular speed
- [x] 3.7 Add tests for physics: momentum conservation, thrust direction, drag decay, brake clamping

## 4. Ship Controls

- [x] 4.1 Implement input system: map A/Left to rotate-left, D/Right to rotate-right, W/Up to thrust, S/Down to brake
- [x] 4.2 Wire input system to ship physics (set thrust/brake/rotation flags each frame)
- [x] 4.3 Verify simultaneous inputs work (thrust + rotate, brake + rotate)

## 5. Camera Follow

- [x] 5.1 Implement smooth camera follow system: lerp camera position toward ship position each frame
- [x] 5.2 Add configurable smoothing factor constant
- [x] 5.3 Verify camera tracks ship without snapping or oscillation

## 6. World Structures

- [x] 6.1 Define wall/structure data model: rectangles with position, size, and an opening flag
- [x] 6.2 Implement AABB collision detection between ship and structure walls
- [x] 6.3 Implement collision response: zero perpendicular velocity component, push ship out of overlap
- [x] 6.4 Build small structure: simple room with two openings
- [x] 6.5 Build medium structure: station with multiple connected corridors
- [x] 6.6 Build large structure: asteroid field with narrow passages
- [x] 6.7 Add tests for collision: wall blocking, opening pass-through, no-tunneling

## 7. Procedural Visuals

- [x] 7.1 Create ship mesh: triangle/arrow shape with color accent on front
- [x] 7.2 Create structure meshes: rectangles and lines in contrasting color for walls
- [x] 7.3 Generate star background: scattered small dots/circles on dark background
- [x] 7.4 Optional: add parallax scrolling effect to star background layer (skipped for prototype)
- [x] 7.5 Optional: add thrust particle effect behind ship when accelerating

## 8. Integration and Polish

- [x] 8.1 Spawn ship at a defined starting position near the first structure
- [x] 8.2 Tune physics constants (thrust, drag, rotation speed, brake force) until flight feels satisfying
- [x] 8.3 Tune camera smoothing factor
- [x] 8.4 Verify the player can fly through all structure openings without false collisions
- [x] 8.5 Verify the world feels explorable and the ship feels good to fly
