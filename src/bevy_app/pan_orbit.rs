use bevy::{
    core_pipeline::core_3d::Camera3d,
    ecs::{bundle::Bundle, component::Component},
    input::keyboard::KeyCode,
    math::Vec3,
};

/// This module provides component and system for pan-orbit controller for App.

/// Bundre to spawn custom camera with pan-orbit controller.
#[derive(Bundle, Default)]
pub struct PanOrbitCameraBundle {
    pub camera: Camera3d,
    pub state: PanOrbitState,
    pub settings: PanOrbitSettings,
}

/// Internal state of the pan-orbit controller
#[derive(Component)]
pub struct PanOrbitState {
    pub center: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Component)]
pub struct PanOrbitSettings {
    /// World units per pixel of mouse motion
    pub pan_sensitivity: f32,

    /// Radians pre pixel of mouse motion
    pub orbit_sensitivity: f32,

    /// Exponent per mouse scroll units
    pub zoom_sensitivity: f32,

    /// key to hold for panning
    pub pan_key: Option<KeyCode>,
    /// Key to hold for orbiting
    pub orbit_key: Option<KeyCode>,
    /// Key to hold for zooming
    pub zoom_key: Option<KeyCode>,
    /// Action is bound to the scroll wheel
    pub scroll_action: Option<PanOrbitAction>,
    /// For devices with a notched scroll wheel
    pub scroll_line_sensitivity: f32,
    /// For devices with smooth scrolling
    pub scroll_pixel_sensitivity: f32,
}

/// Actions of controller
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanOrbitAction {
    Pan,
    Orbit,
    Zoom,
}

impl Default for PanOrbitState {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            radius: 1.0,
            upside_down: false,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl Default for PanOrbitSettings {
    fn default() -> Self {
        Self {
            // 1000 pixels per world unit
            pan_sensitivity: 0.001,
            // 0.1 degree per pixel
            orbit_sensitivity: 0.1f32.to_radians(),
            zoom_sensitivity: 0.01,
            pan_key: Some(KeyCode::ControlLeft),
            orbit_key: Some(KeyCode::AltLeft),
            zoom_key: Some(KeyCode::ShiftLeft),
            scroll_action: Some(PanOrbitAction::Zoom),
            // 1 line = 16 pixels of motion
            scroll_line_sensitivity: 16.0,
            scroll_pixel_sensitivity: 1.0,
        }
    }
}
