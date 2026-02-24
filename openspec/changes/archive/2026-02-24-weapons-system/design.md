## Context

The game has a ship with WASD controls, drifting asteroids with AABB collision, and a health system. Velocity is generic (any entity with `Velocity` moves via `position_integration_system`). Asteroids currently have 9999 HP, making them indestructible. There is no way for the player to fight back.

## Goals / Non-Goals

**Goals:**
- Player can fire projectiles by pressing Space
- Projectiles damage and destroy asteroids
- Simple, satisfying shooting that fits the existing physics feel
- Weapon has a cooldown to prevent spam

**Non-Goals:**
- Multiple weapon types or upgrades
- Projectile-to-projectile collisions
- Visual effects (explosions, muzzle flash) — can be added later
- Sound effects
- Ammo limits

## Decisions

**New `Projectile` component + `WeaponsPlugin` in `src/weapons.rs`**

Keep weapons logic in its own module following the existing plugin pattern. The `Projectile` component tracks remaining lifetime. Projectiles reuse the existing `Velocity` component for movement — no new movement system needed.

Alternative: Adding fire logic directly to `ShipPlugin` — rejected to keep modules focused on single concerns.

**Weapon cooldown via `WeaponCooldown` component on the ship**

A simple timer component on the ship entity tracks time until next shot. The fire system checks cooldown before spawning. This avoids global resources and keeps state local to the entity.

Alternative: Global `Resource` for cooldown — rejected because it wouldn't scale if we later add multiple ships or weapon-bearing entities.

**Projectile spawns at ship nose with additive velocity**

Projectile starts at `ship_position + facing_direction * offset` to avoid self-collision. Its velocity is `ship_velocity + facing_direction * projectile_speed`. This feels natural — shooting while moving forward makes bullets faster.

**Asteroid HP lowered to a reasonable value**

Change asteroid HP from 9999 to a value proportional to size (e.g., `half_size.x * half_size.y * 0.1`), so larger asteroids take more hits. Projectile damage is a flat amount per hit.

**Despawn-at-zero-HP system**

A generic system despawns any entity whose `Health.current` reaches 0. This handles both asteroids (destroyed by projectiles) and could later handle the ship (game over). Projectiles despawn via their own lifetime system, not health.

**Projectile-wall collision: despawn projectile**

Projectiles hitting walls simply despawn. No bounce, no wall damage.

**Projectile-asteroid collision: deal damage + despawn projectile**

On AABB overlap, apply projectile damage to asteroid health, then despawn the projectile. No bounce — simple bullets.

## Risks / Trade-offs

- [Many projectiles on screen] Fast firing could create many entities. → Mitigation: Cooldown limits fire rate; lifetime ensures cleanup. Can add max-projectile cap later if needed.
- [Projectile tunneling] Fast, small projectiles could pass through thin asteroids in one frame. → Mitigation: Acceptable for now; projectile speed should be tuned to avoid this with typical asteroid sizes (~30-90 units wide).
- [Asteroid HP balance] Size-based HP is a rough heuristic. → Mitigation: Easy to tune constants later; good enough for first iteration.
