use bevy::{prelude::*, window::PrimaryWindow};

use crate::bevy_app::ui::components::{HudAnchor, HudRotation};

const UI_SCALE_2: f32 = 0.1 / 2.;

/// The system to keep updating HUD at fixed position.
pub fn transform_ui_anchors(
    mut q_hud_anchor: Query<(&mut Transform, &HudAnchor)>,
    q_hud_rotation: Query<&HudRotation>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(rotation) = q_hud_rotation.single() else {
        tracing::warn!("Need HudRotation in setup");
        return;
    };

    let Ok(window) = q_window.single() else {
        return;
    };
    let size = window.resolution.physical_size();

    for (mut transform, anchor) in &mut q_hud_anchor {
        // define each translation
        let translation = match anchor {
            HudAnchor::NavigationCube => {
                // Navigation Cube must be upper right.
                // The cube's root is not origin.
                Vec3::new(
                    UI_SCALE_2 * (size.x - 96) as f32,
                    UI_SCALE_2 * (size.y - 96) as f32,
                    0.,
                )
            }
            HudAnchor::Axes => {
                // Axes must be lower right. `96 / 2` is need to show it
                Vec3::new(
                    UI_SCALE_2 * (size.x - 96) as f32,
                    -UI_SCALE_2 * (size.y - 96) as f32,
                    0.,
                )
            }
        };

        transform.translation = translation;
        transform.rotation = *(*rotation);
    }
}
