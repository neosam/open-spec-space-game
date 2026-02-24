use bevy::{color::palettes::css, prelude::*};

use crate::ship::Ship;
use crate::world::{Asteroid, Wall};

pub struct MinimapPlugin;

const PANEL_SIZE: f32 = 160.0;
const WORLD_RADIUS: f32 = 2000.0;
const INDICATOR_COUNT: usize = 64;
const INDICATOR_SIZE: f32 = 4.0;
const SHIP_INDICATOR_SIZE: f32 = 6.0;
const PANEL_MARGIN: f32 = 16.0;
const BORDER_WIDTH: f32 = 2.0;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_minimap)
            .add_systems(Update, update_minimap_system);
    }
}

#[derive(Component)]
struct MinimapPanel;

#[derive(Component)]
struct MinimapShipIndicator;

#[derive(Component)]
struct MinimapIndicator {
    index: usize,
}

/// Maps a world offset (relative to ship) to a pixel position within the minimap panel.
/// Returns (left, top) in pixels. Returns None if outside the panel.
pub fn world_offset_to_panel(offset: Vec2) -> Option<(f32, f32)> {
    let scale = PANEL_SIZE / (2.0 * WORLD_RADIUS);
    let px = offset.x * scale + PANEL_SIZE / 2.0;
    let py = -offset.y * scale + PANEL_SIZE / 2.0; // Y is flipped: world +Y is up, UI +Y is down
    if px >= 0.0 && px <= PANEL_SIZE && py >= 0.0 && py <= PANEL_SIZE {
        Some((px, py))
    } else {
        None
    }
}

fn spawn_minimap(mut commands: Commands) {
    // Border node (slightly larger than panel)
    commands
        .spawn((
            MinimapPanel,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(PANEL_MARGIN),
                bottom: Val::Px(PANEL_MARGIN),
                width: Val::Px(PANEL_SIZE + BORDER_WIDTH * 2.0),
                height: Val::Px(PANEL_SIZE + BORDER_WIDTH * 2.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 1.0)),
        ))
        .with_children(|border| {
            // Inner panel
            border
                .spawn((
                    Node {
                        width: Val::Px(PANEL_SIZE),
                        height: Val::Px(PANEL_SIZE),
                        margin: UiRect::all(Val::Px(BORDER_WIDTH)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 1.0)),
                ))
                .with_children(|panel| {
                    // Ship indicator at center
                    panel.spawn((
                        MinimapShipIndicator,
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(PANEL_SIZE / 2.0 - SHIP_INDICATOR_SIZE / 2.0),
                            top: Val::Px(PANEL_SIZE / 2.0 - SHIP_INDICATOR_SIZE / 2.0),
                            width: Val::Px(SHIP_INDICATOR_SIZE),
                            height: Val::Px(SHIP_INDICATOR_SIZE),
                            ..default()
                        },
                        BackgroundColor(Color::from(css::LIMEGREEN)),
                    ));

                    // Indicator pool
                    for i in 0..INDICATOR_COUNT {
                        panel.spawn((
                            MinimapIndicator { index: i },
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                top: Val::Px(0.0),
                                width: Val::Px(INDICATOR_SIZE),
                                height: Val::Px(INDICATOR_SIZE),
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                            Visibility::Hidden,
                        ));
                    }
                });
        });
}

fn update_minimap_system(
    ship_query: Query<&Transform, With<Ship>>,
    asteroid_query: Query<&Transform, (With<Asteroid>, Without<Ship>, Without<Wall>)>,
    wall_query: Query<&Transform, (With<Wall>, Without<Ship>, Without<Asteroid>)>,
    mut indicator_query: Query<
        (&MinimapIndicator, &mut Node, &mut BackgroundColor, &mut Visibility),
    >,
) {
    let Ok(ship_transform) = ship_query.single() else {
        return;
    };
    let ship_pos = ship_transform.translation.truncate();

    // Collect entities to show: (world_offset, color)
    let mut entries: Vec<(Vec2, Color)> = Vec::new();

    for ast_transform in &asteroid_query {
        let offset = ast_transform.translation.truncate() - ship_pos;
        if offset.length() <= WORLD_RADIUS {
            entries.push((offset, Color::from(css::SADDLE_BROWN)));
        }
    }

    for wall_transform in &wall_query {
        let offset = wall_transform.translation.truncate() - ship_pos;
        if offset.length() <= WORLD_RADIUS {
            entries.push((offset, Color::from(css::STEEL_BLUE)));
        }
    }

    // Assign entries to indicators
    for (indicator, mut node, mut bg, mut visibility) in &mut indicator_query {
        if let Some((offset, color)) = entries.get(indicator.index) {
            if let Some((px, py)) = world_offset_to_panel(*offset) {
                node.left = Val::Px(px - INDICATOR_SIZE / 2.0);
                node.top = Val::Px(py - INDICATOR_SIZE / 2.0);
                bg.0 = *color;
                *visibility = Visibility::Inherited;
            } else {
                *visibility = Visibility::Hidden;
            }
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_offset_to_panel_center() {
        // Zero offset should map to center of panel
        let (px, py) = world_offset_to_panel(Vec2::ZERO).unwrap();
        assert!((px - PANEL_SIZE / 2.0).abs() < 0.01, "px={px}");
        assert!((py - PANEL_SIZE / 2.0).abs() < 0.01, "py={py}");
    }

    #[test]
    fn test_world_offset_to_panel_right() {
        // Entity at world_radius to the right should be at the right edge
        let (px, _py) = world_offset_to_panel(Vec2::new(WORLD_RADIUS, 0.0)).unwrap();
        assert!((px - PANEL_SIZE).abs() < 0.01, "px={px}");
    }

    #[test]
    fn test_world_offset_to_panel_up() {
        // Entity at world_radius upward should map to the top edge (py=0)
        let (_px, py) = world_offset_to_panel(Vec2::new(0.0, WORLD_RADIUS)).unwrap();
        assert!(py.abs() < 0.01, "py={py}");
    }

    #[test]
    fn test_world_offset_to_panel_outside() {
        // Entity beyond world_radius should return None
        let result = world_offset_to_panel(Vec2::new(WORLD_RADIUS + 100.0, 0.0));
        assert!(result.is_none(), "Should be outside panel");
    }

    #[test]
    fn test_world_offset_to_panel_half_right() {
        // Entity at half the world_radius to the right
        let (px, py) = world_offset_to_panel(Vec2::new(WORLD_RADIUS / 2.0, 0.0)).unwrap();
        let expected_px = PANEL_SIZE * 3.0 / 4.0; // center + half of half-panel
        assert!((px - expected_px).abs() < 0.01, "px={px}, expected={expected_px}");
        assert!((py - PANEL_SIZE / 2.0).abs() < 0.01, "py={py}");
    }
}
