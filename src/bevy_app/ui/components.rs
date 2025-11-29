use bevy::{ecs::component::Component, render::view::RenderLayers};

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
