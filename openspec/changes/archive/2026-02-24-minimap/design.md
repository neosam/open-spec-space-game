## Context

The game has a 2D procedurally generated world with chunks (1024x1024 units). The player sees roughly 1280x720 units on screen. Structures (walls, asteroids) exist beyond the viewport but are invisible until approached. The existing HUD has a health bar in the top-left using Bevy UI nodes.

## Goals / Non-Goals

**Goals:**
- Minimap in the bottom-right showing a wider area than the viewport
- Ship always centered on the minimap
- Asteroids and walls shown as distinct colored dots
- Opaque dark panel with a border, consistent with the existing HUD style

**Non-Goals:**
- Interactable minimap (clicking to navigate)
- Fog of war or discovery
- Showing chunk boundaries
- Showing projectiles on the minimap

## Decisions

**Use Bevy UI nodes for the minimap panel and indicators**

Build the minimap using Bevy's UI system (`Node`, `BackgroundColor`), the same approach as the health bar. The minimap panel is a fixed-size `Node` in the bottom-right. Entity indicators are small child `Node` elements positioned absolutely within the panel using `left`/`top` pixel offsets.

Alternative: A second camera rendering to a texture — rejected as significantly more complex for minimal visual benefit. Dot indicators on a UI panel are sufficient for spatial awareness.

**New `src/minimap.rs` module with `MinimapPlugin`**

Follows the existing plugin pattern. Keeps minimap logic separate from `visuals.rs` which is already large.

**Minimap scale and radius**

The minimap covers a configurable world radius (e.g., 2000 units) mapped to the panel size (e.g., 160x160 pixels). Entities beyond this radius are not shown. The scale factor is `panel_size / (2 * world_radius)`.

**Entity positions calculated relative to ship**

Each frame, query the ship's world position. For each asteroid/wall within the minimap radius, compute `(entity_pos - ship_pos) * scale + panel_center` to get the indicator's position within the panel. Clamp or skip indicators that fall outside the panel bounds.

**Indicator recycling via marker components**

Spawn a pool of indicator nodes at startup (e.g., 64). Each frame, assign them to nearby entities. Unused indicators are hidden by setting their visibility or moving them off-panel. This avoids spawn/despawn churn every frame.

## Risks / Trade-offs

- [Performance with many entities] Iterating all asteroids/walls every frame could be costly if chunk count is high. → Mitigation: The chunk system limits loaded entities to a ~5x5 grid (~25 chunks). At 12 asteroids per chunk + walls, we're looking at ~300 entities max. Iterating 300 entities per frame is negligible.
- [Indicator pool exhaustion] If more entities exist than indicator slots, some won't show. → Mitigation: 64 indicators is generous for the typical loaded area. Can increase later if needed.
- [UI node positioning precision] Bevy UI uses pixel values which may not be sub-pixel accurate. → Mitigation: Acceptable for dot indicators; exact positioning is not critical for a minimap.
