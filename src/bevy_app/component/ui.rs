use std::ops::Deref;

use bevy::{camera::visibility::RenderLayers, prelude::*};

#[derive(Component)]
pub struct NavigationCube;

/// Marker component to set up texture for glTF materials
#[derive(Component)]
pub struct NeedsTextureSetup;

#[derive(Component)]
pub struct NeedsRenderLayers(pub RenderLayers);

/// marker component of AxesGizmo
#[derive(Component)]
pub struct AxesGizmo;

#[derive(Component)]
pub struct SketchBaseGizmo;

/// Tag for sketch operation
#[derive(Component)]
pub struct SketchOperationTag;

/// Tag component for cursor icon
#[derive(Component)]
pub struct CursorIconTag;

/// A component of HUD Anchor point.
#[derive(Debug, Clone, Component)]
pub enum HudAnchor {
    NavigationCube,
    Axes,
}

/// A component to handle HUD. This is global entity
#[derive(Debug, Component, Default)]
pub struct HudRotation(Quat);

impl HudRotation {
    /// Update the transform
    pub fn update(&mut self, transform: Quat) {
        self.0 = transform;
    }
}

/// Convenience deref
impl Deref for HudRotation {
    type Target = Quat;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
