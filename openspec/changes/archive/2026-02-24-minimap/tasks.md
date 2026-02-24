## 1. Module Setup

- [x] 1.1 Create `src/minimap.rs` with `MinimapPlugin` and constants (panel size, world radius, indicator count, colors)
- [x] 1.2 Register `MinimapPlugin` in `main.rs`

## 2. Minimap Panel

- [x] 2.1 Implement `spawn_minimap` startup system: create the minimap panel `Node` (bottom-right, fixed size, dark background, border) with a ship indicator child node at center
- [x] 2.2 Spawn a pool of indicator child nodes (hidden initially) inside the panel, marked with `MinimapIndicator` component

## 3. Minimap Update

- [x] 3.1 Implement `update_minimap_system`: each frame, query ship position, then iterate asteroids and walls within world radius. Assign each to an indicator node, set its position (`left`/`top`) and color. Hide unused indicators.
- [x] 3.2 Add test: indicator position calculation maps world offset to panel pixel offset correctly
