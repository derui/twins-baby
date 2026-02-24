use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::{
    ecs::{
        bundle::Bundle,
        change_detection::DetectChanges,
        component::Component,
        error::BevyError,
        message::MessageReader,
        query::With,
        system::{Commands, Query, Res},
    },
    input::{
        ButtonInput,
        keyboard::Key,
        mouse::MouseButton,
    },
    math::{Vec2, Vec3},
    transform::components::Transform,
};
use ui_event::{MouseMovementNotification, MouseWheelNotification};

use crate::bevy_app::camera::{
    CameraMoveDuration, CameraMoveOperation, CameraMoveRequest, MainCamera, PanOrbitOperation,
};

/// This module provides component and system for pan-orbit controller for App.
/// based on https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html

/// Bundre to spawn pan-orbit controller.
#[derive(Bundle, Default)]
pub struct PanOrbitCameraBundle {
    pub state: PanOrbitOperation,
    pub settings: PanOrbitSettings,
}

#[derive(Component)]
pub struct PanOrbitSettings {
    /// World units per pixel of mouse motion
    pub pan_sensitivity: f32,

    /// Radians pre pixel of mouse motion
    pub orbit_sensitivity: f32,

    /// Exponent per mouse scroll units
    pub zoom_sensitivity: f32,

    /// Input method for panning
    pub pan_input: Option<InputMethod>,
    /// Input method for orbiting
    pub orbit_input: Option<InputMethod>,
    /// Input method for zooming
    pub zoom_input: Option<InputMethod>,

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

/// Input method for an action
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputMethod {
    /// Pressing key with mouse motion.
    Key(Key),

    /// Pressing button with mouse motion
    MouseButton(MouseButton),

    /// Scrolling only.
    Scroll,
}

impl Default for PanOrbitSettings {
    fn default() -> Self {
        Self {
            // 1000 pixels per world unit
            pan_sensitivity: 0.001,
            // 0.1 degree per pixel
            orbit_sensitivity: 0.1f32.to_radians(),
            zoom_sensitivity: 0.01,
            pan_input: Some(InputMethod::Key(Key::Control)),
            orbit_input: Some(InputMethod::Key(Key::Alt)),
            zoom_input: Some(InputMethod::Scroll),
            // 1 line = 16 pixels of motion
            scroll_line_sensitivity: 16.0,
            scroll_pixel_sensitivity: 1.0,
        }
    }
}

/// Setup a pan-orbit controller
pub fn setup_pan_orbit(mut commands: Commands) -> Result<(), BevyError> {
    let mut pan_orbit = PanOrbitCameraBundle::default();

    pan_orbit.state.center = Vec3::new(1.0, 2.0, 3.0);
    pan_orbit.state.radius = 50.0;
    pan_orbit.state.pitch = 15.0f32.to_radians();
    pan_orbit.state.yaw = 30.0f32.to_radians();

    commands.spawn(pan_orbit);

    Ok(())
}

pub fn pan_orbit_camera(
    mut commands: Commands,
    kbd: Res<ButtonInput<Key>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut evr_motion: MessageReader<MouseMovementNotification>,
    mut evr_scroll: MessageReader<MouseWheelNotification>,
    mut q_camere: Query<(&PanOrbitSettings, &mut PanOrbitOperation)>,
    q_transform: Query<&Transform, With<MainCamera>>,
) -> Result<(), BevyError> {
    let mut total_motion: Vec2 = evr_motion
        .read()
        .map(|ev| Vec2::new(*ev.delta_x as f32, *ev.delta_y as f32))
        .sum();

    // Reverse Y. (Worldscpace coodinate system has Y up, but mouse Y goes down)
    total_motion.y = -total_motion.y;

    let mut total_scroll_pixels = Vec2::ZERO;
    for ev in evr_scroll.read() {
        total_scroll_pixels.x += *ev.delta_x;
        total_scroll_pixels.y -= *ev.delta_y;
    }

    for (settings, mut state) in &mut q_camere {
        // Helper function to check if input method is active
        let is_input_active = |input_method: Option<InputMethod>| -> bool {
            match input_method {
                Some(InputMethod::Key(key)) => kbd.pressed(key),
                Some(InputMethod::MouseButton(button)) => mouse.pressed(button),
                Some(InputMethod::Scroll) => false,
                None => false,
            }
        };

        // Helper function to check if input method was just pressed
        let is_input_just_pressed = |input_method: Option<InputMethod>| -> bool {
            match input_method {
                Some(InputMethod::Key(key)) => kbd.just_pressed(key),
                Some(InputMethod::MouseButton(button)) => mouse.just_pressed(button),
                Some(InputMethod::Scroll) => false,
                None => false,
            }
        };

        // Calculate pan/orbit/zoom based on configured input methods
        let mut total_pan = Vec2::ZERO;
        if is_input_active(settings.pan_input.clone()) {
            total_pan -= total_motion * settings.pan_sensitivity;
        }
        if settings.pan_input == Some(InputMethod::Scroll) {
            total_pan -=
                total_scroll_pixels * settings.scroll_pixel_sensitivity * settings.pan_sensitivity;
        }

        let mut total_orbit = Vec2::ZERO;
        if is_input_active(settings.orbit_input.clone()) {
            total_orbit -= total_motion * settings.orbit_sensitivity;
        }
        if settings.orbit_input == Some(InputMethod::Scroll) {
            total_orbit -= total_scroll_pixels
                * settings.scroll_pixel_sensitivity
                * settings.orbit_sensitivity;
        }

        let mut total_zoom = Vec2::ZERO;
        if is_input_active(settings.zoom_input.clone()) {
            total_zoom -= total_motion * settings.zoom_sensitivity;
        }
        if settings.zoom_input == Some(InputMethod::Scroll) {
            total_zoom -=
                total_scroll_pixels * settings.scroll_pixel_sensitivity * settings.zoom_sensitivity;
        }

        // Upon starting a new orbit maneuver
        if is_input_just_pressed(settings.orbit_input.clone()) {
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
            state.center += q_transform.single().unwrap().right() * total_pan.x * radius;
            state.center += q_transform.single().unwrap().up() * total_pan.y * radius;
        }

        if any || state.is_added() {
            commands.spawn(CameraMoveRequest::new(
                CameraMoveOperation::ByOrbit(state.clone()),
                CameraMoveDuration::Immediate,
            ));
        }
    }

    Ok(())
}
