## Why

The procedurally generated world is large, and the player has no spatial awareness beyond what's visible on screen. A minimap gives the player a bird's-eye view of nearby structures and hazards, improving navigation and situational awareness.

## What Changes

- Add a minimap HUD element in the bottom-right corner of the screen
- Opaque dark background panel with a clear border
- Show the ship as a centered indicator (always in the middle of the minimap)
- Show nearby asteroids as colored dots
- Show nearby walls as colored dots/marks
- The minimap view scales to show a larger area than the main camera viewport
- Minimap updates each frame to reflect entity positions relative to the ship

## Capabilities

### New Capabilities

- `minimap`: Minimap HUD display showing ship position, nearby asteroids, and walls as colored indicators on an opaque panel

### Modified Capabilities

(none)

## Impact

- `src/visuals.rs`: Add minimap spawn and update systems, or create a new `src/minimap.rs` module
- `src/main.rs`: Register minimap plugin/systems if using a new module
- No changes to game logic, physics, or existing visuals
