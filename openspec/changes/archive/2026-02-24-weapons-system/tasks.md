## 1. Ship Input Extension

- [x] 1.1 Add `fire: bool` field to `ShipInput` component
- [x] 1.2 Read `KeyCode::Space` in `input_system` and set `fire` field
- [x] 1.3 Add test: fire input is set when Space is pressed

## 2. Weapons Module Setup

- [x] 2.1 Create `src/weapons.rs` with `WeaponsPlugin`, `Projectile` component (with `lifetime: f32`), and `WeaponCooldown` component (with `remaining: f32`)
- [x] 2.2 Add `WeaponCooldown` component to the ship entity on spawn
- [x] 2.3 Implement `weapon_cooldown_system` that ticks down `WeaponCooldown.remaining` each frame, clamped to zero
- [x] 2.4 Implement `fire_projectile_system` that spawns a projectile when `ShipInput.fire` is true and cooldown is zero. Projectile spawns at ship nose with velocity = ship velocity + facing direction * projectile speed. Reset cooldown after firing.
- [x] 2.5 Implement `projectile_lifetime_system` that decrements lifetime and despawns projectiles at zero
- [x] 2.6 Register `WeaponsPlugin` in `main.rs`
- [x] 2.7 Add tests: projectile spawns on fire, cooldown prevents rapid fire, projectile despawns after lifetime

## 3. Projectile Collisions

- [x] 3.1 Implement `projectile_asteroid_collision_system`: AABB overlap check, apply damage to asteroid, despawn projectile
- [x] 3.2 Implement `projectile_wall_collision_system`: AABB overlap check, despawn projectile
- [x] 3.3 Add tests: projectile damages asteroid on collision, projectile despawns on wall hit

## 4. Destructible Asteroids

- [x] 4.1 Change asteroid HP from 9999 to size-based value (e.g., `half_size.x * half_size.y * 0.1`)
- [x] 4.2 Implement `zero_health_despawn_system`: despawn any entity with `Health.current <= 0.0`
- [x] 4.3 Add test: entity with zero health is despawned

## 5. Projectile Visuals

- [x] 5.1 Add `attach_projectile_meshes` system in visuals: small bright-colored shape for projectiles without meshes
