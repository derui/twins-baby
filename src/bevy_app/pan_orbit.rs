use std::f32::consts::{FRAC_1_PI, FRAC_2_PI, FRAC_PI_2, PI, TAU};

use bevy::{
    asset::transformer::{self, TransformedAsset},
    core_pipeline::core_3d::Camera3d,
    ecs::{
        bundle::Bundle,
        change_detection::DetectChanges,
        component::Component,
        error::BevyError,
        event::EventReader,
        system::{Query, Res},
    },
    image::TranscodeFormat,
    input::{
        ButtonInput,
        keyboard::KeyCode,
        mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    },
    math::{EulerRot, Quat, Vec2, Vec2Swizzles, Vec3},
    transform::components::Transform,
};

/// This module provides component and system for pan-orbit controller for App.
/// based on https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html

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

pub fn pan_orbit_camera(
    kbd: Res<ButtonInput<KeyCode>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut q_camere: Query<(&PanOrbitSettings, &mut PanOrbitState, &mut Transform)>,
) -> Result<(), BevyError> {
    let mut total_motion: Vec2 = evr_motion.read().map(|ev| ev.delta).sum();

    // Reverse Y. (Worldscpace coodinate system has Y up, but mouse Y goes down)
    total_motion.y = -total_motion.y;

    let mut total_scroll_lines = Vec2::ZERO;
    let mut total_scroll_pixels = Vec2::ZERO;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                total_scroll_lines.x += ev.x;
                total_scroll_lines.y -= ev.y;
            }
            MouseScrollUnit::Pixel => {
                total_scroll_pixels.x += ev.x;
                total_scroll_pixels.y -= ev.y;
            }
        }
    }

    for (settings, mut state, mut transform) in &mut q_camere {
        // calculate pan/orbit/zoom with key and mouse
        let mut total_pan = Vec2::ZERO;
        if settings
            .pan_key
            .map(|key| kbd.pressed(key))
            .unwrap_or(false)
        {
            total_pan -= total_motion * settings.pan_sensitivity;
        }
        if settings.scroll_action == Some(PanOrbitAction::Pan) {
            total_pan -=
                total_scroll_lines * settings.scroll_line_sensitivity * settings.pan_sensitivity;
            total_pan -=
                total_scroll_pixels * settings.scroll_pixel_sensitivity * settings.pan_sensitivity;
        }

        let mut total_orbit = Vec2::ZERO;
        if settings
            .orbit_key
            .map(|key| kbd.pressed(key))
            .unwrap_or(false)
        {
            total_orbit -= total_motion * settings.orbit_sensitivity;
        }
        if settings.scroll_action == Some(PanOrbitAction::Orbit) {
            total_orbit -=
                total_scroll_lines * settings.scroll_line_sensitivity * settings.orbit_sensitivity;
            total_orbit -= total_scroll_pixels
                * settings.scroll_pixel_sensitivity
                * settings.orbit_sensitivity;
        }

        let mut total_zoom = Vec2::ZERO;
        if settings
            .zoom_key
            .map(|key| kbd.pressed(key))
            .unwrap_or(false)
        {
            total_zoom -= total_motion * settings.zoom_sensitivity;
        }
        if settings.scroll_action == Some(PanOrbitAction::Zoom) {
            total_zoom -=
                total_scroll_lines * settings.scroll_line_sensitivity * settings.zoom_sensitivity;
            total_zoom -=
                total_scroll_pixels * settings.scroll_pixel_sensitivity * settings.zoom_sensitivity;
        }

        // Upon starting a new orbit maneuver
        if settings
            .orbit_key
            .map(|key| kbd.just_pressed(key))
            .unwrap_or(false)
        {
            state.upside_down = state.pitch < -FRAC_PI_2 || state.pitch > FRAC_PI_2;
        }

        if state.upside_down {
            total_orbit.x = -total_orbit.x;
        }

        let mut any = false;
        if total_zoom != Vec2::ZERO {
            any = true;

            // in order for zoom to feel intuitive, everything needs to be exponential
            state.radius *= (-total_zoom.y).exp();
        }

        if total_orbit != Vec2::ZERO {
            any = true;

            state.yaw += total_orbit.x;
            state.pitch += total_orbit.y;

            // yaw/pitch wrap around to stay between +-180 degrees
            if state.yaw > PI {
                // 2 * PI
                state.yaw -= TAU;
            }
            if state.yaw < -PI {
                state.yaw += TAU;
            }
            if state.pitch > PI {
                // 2 * PI
                state.pitch -= TAU;
            }
            if state.pitch < -PI {
                state.pitch += TAU;
            }
        }

        if total_pan != Vec2::ZERO {
            any = true;
            let radius = state.radius;
            state.center += transform.right() * total_pan.x * radius;
            state.center += transform.up() * total_pan.y * radius;
        }

        if any || state.is_added() {
            // rotation performs yaw/pitch/roll via quatanion.
            transform.rotation = Quat::from_euler(EulerRot::YXZ, state.yaw, state.pitch, 0.0);
            // using back direction vector to stay the camera at the desired radius from the center
            transform.translation = state.center + transform.back() * state.radius;
        }
    }

    Ok(())
}
