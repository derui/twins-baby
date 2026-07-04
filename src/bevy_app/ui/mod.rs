pub mod anchor;
mod cursor;
mod gizmo;
mod navigation_cube;

use bevy::ecs::{error::BevyError, system::Commands};
use bevy::prelude::*;

use crate::bevy_app::component::ui::HudRotation;
use crate::bevy_app::ui::cursor::{setup_cursor_icon, update_cursor_icon};
use crate::bevy_app::ui::gizmo::setup_gizmos;
use crate::bevy_app::ui::navigation_cube::{
    insert_render_layer, setup_navigation_cube, setup_navigation_texture,
};

pub use gizmo::AxesGizmoGroup;
pub use gizmo::SketchBaseGizmoGroup;
pub use gizmo::draw_gizmos;
pub use gizmo::draw_sketch_gizmos;

pub trait AppUiExt {
    /// Init UI resources
    fn init_ui(&mut self) -> &mut Self;
}

impl AppUiExt for App {
    fn init_ui(&mut self) -> &mut Self {
        self.add_systems(
            Startup,
            (
                setup_ui,
                setup_navigation_cube,
                setup_gizmos,
                setup_cursor_icon,
            ),
        )
        .init_gizmo_group::<AxesGizmoGroup>()
        .init_gizmo_group::<SketchBaseGizmoGroup>()
        .add_systems(Update, (setup_navigation_texture, insert_render_layer))
        .add_systems(Update, update_cursor_icon)
    }
}

/// Setup the twins-baby UI elements
fn setup_ui(mut commands: Commands) -> Result<(), BevyError> {
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 600.,
        ..default()
    });

    commands.spawn(HudRotation::default());

    Ok(())
}
