## Why

The player currently has no way to fight back against asteroids. Adding a weapons system gives the player agency and transforms asteroids from passive hazards into threats that can be dealt with, making the game loop more engaging.

## What Changes

- Add a projectile shooting system: pressing Space fires a bullet in the ship's facing direction
- Projectiles are small, fast entities that travel straight and despawn on hit or after a timeout
- Projectiles deal damage to asteroids on contact
- Asteroids become destructible: lower HP so projectiles can destroy them; asteroids despawn when HP reaches zero
- Add a fire cooldown to prevent continuous fire spam

## Capabilities

### New Capabilities

- `projectile-system`: Projectile spawning, movement, lifetime, and despawn behavior
- `projectile-collisions`: Collision detection and response between projectiles and asteroids/walls

### Modified Capabilities

- `ship-controls`: Add Space bar as fire input binding
- `health-system`: Asteroids become destructible (lower HP), entities despawn at zero HP

## Impact

- New `src/weapons.rs` module with projectile spawning and weapon cooldown systems
- `src/ship.rs`: Add `fire` field to `ShipInput`, read Space key in input system
- `src/world.rs`: Add projectile collision systems, lower asteroid HP, add despawn-on-zero-health system
- `src/visuals.rs`: Add projectile mesh attachment
- `src/main.rs`: Register new plugin
