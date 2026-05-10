use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::{
    camera::Camera,
    ecs::{
        bundle::Bundle,
        change_detection::DetectChanges,
        component::Component,
        entity::Entity,
        error::BevyError,
        message::MessageReader,
        query::With,
        system::{Commands, Query, Res},
    },
    input::{
        ButtonInput,
        keyboard::Key,
        mouse::{MouseButton, MouseMotion, MouseWheel},
    },
    math::{EulerRot, Quat, Vec2, Vec3, primitives::InfinitePlane3d},
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

use crate::bevy_app::camera::{
    CameraMoveExpectation, CameraMoveHandle, CameraMoveOperation, MainCamera, PanOrbitOperation,
};

/// Starter button. For pan/rotate, not zoom.
const STARTER_BUTTON: MouseButton = MouseButton::Middle;

// This module provides component and system for pan-orbit controller for App.
// based on https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html

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
    pub rotation_sensitivity: f32,

    /// Exponent per mouse scroll units
    pub zoom_sensitivity: f32,

    /// Input method for panning
    pub pan_input: Option<InputMethod>,
    /// Input method for orbiting
    pub rotation_input: Option<InputMethod>,
    /// Input method for zooming
    pub zoom_input: Option<InputMethod>,

    /// For devices with a notched scroll wheel
    pub scroll_line_sensitivity: f32,
    /// For devices with smooth scrolling
    pub scroll_pixel_sensitivity: f32,
}

/// Input method for an action
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputMethod {
    /// Pressing key with mouse motion.
    Key(Key),

    /// Scrolling only.
    Scroll,
}

impl Default for PanOrbitSettings {
    fn default() -> Self {
        Self {
            // 1000 pixels per world unit
            pan_sensitivity: 0.001,
            // 0.1 degree per pixel
            rotation_sensitivity: 0.1f32.to_radians(),
            zoom_sensitivity: 0.01,
            pan_input: Some(InputMethod::Key(Key::Control)),
            rotation_input: None,
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

    pan_orbit.state.center = Vec3::new(0.0, 0.0, 0.0);
    pan_orbit.state.radius = 50.0;
    pan_orbit.state.pitch = 45.0f32.to_radians();
    pan_orbit.state.yaw = 45.0f32.to_radians();

    commands.spawn(pan_orbit);

    Ok(())
}

pub fn pan_orbit_camera(
    mut commands: Commands,
    kbd: Res<ButtonInput<Key>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut evr_motion: MessageReader<MouseMotion>,
    mut evr_wheel: MessageReader<MouseWheel>,
    mut q_camere: Query<(&PanOrbitSettings, &mut PanOrbitOperation)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_transform: Query<(&Camera, &Transform, &GlobalTransform), With<MainCamera>>,
    mut q_handle: Query<(Entity, &mut CameraMoveHandle)>,
) {
    let mut total_motion: Vec2 = evr_motion.read().map(|ev| ev.delta).sum();

    // Reverse Y. (Worldscpace coodinate system has Y up, but mouse Y goes down)
    total_motion.y = -total_motion.y;

    let mut total_scroll_pixels = Vec2::ZERO;
    for ev in evr_wheel.read() {
        total_scroll_pixels.x += ev.x;
        total_scroll_pixels.y -= ev.y;
    }

    let activated = mouse.pressed(STARTER_BUTTON);
    let just_activated = mouse.just_pressed(STARTER_BUTTON);

    // Helper function to check if input method is active
    let is_input_active = |input_method: Option<InputMethod>| -> bool {
        match input_method {
            Some(InputMethod::Key(key)) => activated && kbd.pressed(key),
            Some(InputMethod::Scroll) => false,
            None => false,
        }
    };

    for (settings, mut state) in &mut q_camere {
        // Calculate pan/orbit/zoom based on configured input methods
        let mut total_pan = Vec2::ZERO;
        if is_input_active(settings.pan_input.clone()) {
            total_pan -= total_motion * settings.pan_sensitivity;
        }

        let mut total_orbit = Vec2::ZERO;
        if activated && !is_input_active(settings.pan_input.clone()) {
            total_orbit -= total_motion * settings.rotation_sensitivity;
        }

        let mut total_zoom = Vec2::ZERO;
        if settings.zoom_input == Some(InputMethod::Scroll) {
            total_zoom -=
                total_scroll_pixels * settings.scroll_line_sensitivity * settings.zoom_sensitivity;
        }

        // Upon starting a new orbit maneuver
        if just_activated {
            // check if the cursor is inside the window and get its position
            let Some(cursor_position) = q_window.single().expect("Should get").cursor_position()
            else {
                return;
            };
            state.viewpoint = cursor_position;
        }

        // normarize orbit
        if state.pitch < -FRAC_PI_2 || state.pitch > FRAC_PI_2 {
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
                // ex. 190 degree => -170 degree
                state.yaw -= TAU;
            }
            if state.yaw < -PI {
                // ex. -190 degree => 170 degree
                state.yaw += TAU;
            }
            if state.pitch > PI {
                // 2 * PI
                state.pitch -= TAU;
            }
            if state.pitch < -PI {
                state.pitch += TAU;
            }

            let (camera, transform, global_transform) = q_transform.single().unwrap();
            // In orbit, get the point of the same plane of center of orbit
            let Ok(pivot) = camera.viewport_to_world(global_transform, state.viewpoint) else {
                return;
            };
            let Some(pivot) = pivot.plane_intersection_point(
                state.center,
                InfinitePlane3d {
                    normal: transform.back(),
                },
            ) else {
                return;
            };
            tracing::info!(
                "current center / pivot => {:?} / {:?}",
                &state.center,
                &pivot
            );

            // rotate around the pivot, instead of center. Resulting of that is rotation is same, but
            // center and camera translate based of pivot.
            let rotation = Quat::from_euler(EulerRot::YXZ, total_orbit.x, total_orbit.y, 0.0);
            let center = state.center;
            state.center = pivot + rotation * (center - pivot);
        }

        if total_pan != Vec2::ZERO {
            any = true;
            let radius = state.radius;
            let (_, transform, _) = q_transform.single().unwrap();
            state.center += transform.right() * total_pan.x * radius;
            state.center += transform.up() * total_pan.y * radius;
        }

        if (any || state.is_added())
            && let Ok((entity, mut handle)) = q_handle.single_mut()
        {
            commands.entity(entity).remove::<CameraMoveExpectation>();
            commands
                .entity(entity)
                .insert(CameraMoveOperation::ByOrbit(state.clone()));
            handle.reset();
        }
    }
}
