use std::f32::consts::{self, PI, TAU};

use bevy::{
    log::tracing,
    prelude::*,
    render::{
        camera::{ScalingMode, Viewport},
        view::RenderLayers,
    },
};

use crate::bevy_app::pan_orbit::{PanOrbitCameraBundle, PanOrbitState};

pub const CAMERA_3D_LAYER: usize = 0;
pub const CAMERA_2D_LAYER: usize = 1;

/// This module provides 3D camera basic functionally in Bevy.
#[derive(Component)]
pub struct UiCamera;

#[derive(Component)]
pub struct MainCamera;

// structure of camera position
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraPosition {
    pub center: Vec3,
    pub radius: f32,
    pub pitch: f32,
    pub yaw: f32,
}

/// Request to move camera to a new position
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraMoveOperation {
    /// Do operation relatively to current position
    Relative {
        pan: Option<Vec2>,
        pitch: Option<f32>,
        yaw: Option<f32>,
        zoom: Option<f32>,
    },

    /// Do operation absolutely to current position
    Absolute {
        center: Option<Vec3>,
        pitch: Option<f32>,
        yaw: Option<f32>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraMoveDuration {
    Immediate,
    Duration(f32),
}

/// Expectation of camera movement calculation
#[derive(Debug)]
struct CameraMoveExpectation {
    translation: Option<Vec3>,
    rotation: Option<Quat>,
}

#[derive(Component, Debug)]
pub struct CameraMoveRequest {
    operation: CameraMoveOperation,
    duration: CameraMoveDuration,
    elapsed: f32,
    expected: Option<CameraMoveExpectation>,
}

impl CameraMoveRequest {
    /// Construct new `CameraMoveRequest`. The constructed request is applied to each ticks
    ///
    /// # Arguments
    /// * `operation` : request operation to move camera.
    pub fn new(operation: CameraMoveOperation, duration: CameraMoveDuration) -> Self {
        Self {
            operation,
            duration,
            elapsed: 0.0,
            expected: None,
        }
    }

    /// transform with slerp interpolation.
    ///
    /// # Arguments
    /// * `lerped_time` - A value between 0.0 and 1.0 representing the interpolation factor.
    /// * `transform` - The current transform of the camera to be modified.
    fn slerp_trasform(&mut self, lerped_time: f32, transform: &mut Transform) {
        if let None = self.expected {
            match self.operation {
                CameraMoveOperation::Relative {
                    pan,
                    pitch,
                    yaw,
                    zoom,
                } => {
                    let (mut current_yaw, mut current_pitch, _) =
                        transform.rotation.to_euler(EulerRot::YXZ);
                    current_yaw += yaw.unwrap_or(0.0);
                    current_pitch += pitch.unwrap_or(0.0);

                    // yaw/pitch wrap around to stay between +-180 degrees
                    if current_yaw > PI {
                        // 2 * PI
                        current_yaw -= TAU;
                    }
                    if current_yaw < -PI {
                        current_yaw += TAU;
                    }
                    if current_pitch > PI {
                        // 2 * PI
                        current_pitch -= TAU;
                    }
                    if current_pitch < -PI {
                        current_pitch += TAU;
                    }

                    let diff_radius = zoom
                        .map(|r| {
                            if r < 0.0 {
                                -1.0 * r.abs().exp()
                            } else {
                                r.exp()
                            }
                        })
                        .unwrap_or(0.0);

                    let radius = transform.translation.length() * zoom.unwrap_or(1.0);

                    let mut center = transform.translation.clone();
                    let pan = pan.unwrap_or(Vec2::ZERO);
                    center += pan.x * radius * transform.right();
                    center += pan.y * radius * transform.up();

                    self.expected = Some(CameraMoveExpectation {
                        translation: Some(center + diff_radius * transform.back()),
                        rotation: Some(Quat::from_euler(
                            EulerRot::YXZ,
                            current_yaw,
                            current_pitch,
                            0.0,
                        )),
                    });
                }
                CameraMoveOperation::Absolute { center, pitch, yaw } => {
                    self.expected = Some(CameraMoveExpectation {
                        translation: center,
                        rotation: match (pitch, yaw) {
                            (Some(pitch), Some(yaw)) => {
                                Some(Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0))
                            }
                            (None, Some(yaw)) => {
                                Some(Quat::from_euler(EulerRot::YXZ, yaw, 0.0, 0.0))
                            }
                            (Some(pitch), None) => {
                                Some(Quat::from_euler(EulerRot::YXZ, 0.0, pitch, 0.0))
                            }
                            (None, None) => None,
                        },
                    });
                }
            }
        }

        if let Some(rotation) = self.expected.as_ref().unwrap().rotation {
            transform.rotation = transform.rotation.slerp(rotation, lerped_time);
        }

        if let Some(translation) = self.expected.as_ref().unwrap().translation {
            transform.translation = transform.translation.lerp(translation, lerped_time);
        }
    }
}

/// Setup camera with pan-orbit controller
pub fn setup_camera(mut commands: Commands, window: Query<&Window>) -> Result<(), BevyError> {
    commands.spawn((
        Camera3d::default(),
        RenderLayers::from_layers(&[CAMERA_3D_LAYER]),
        MainCamera,
    ));

    let window = window.single().unwrap();

    commands.spawn((
        Camera3d::default(),
        // use this camera as 2D
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            // 1unit-10px
            scale: 0.10,
            ..OrthographicProjection::default_2d()
        }),
        Camera {
            // clear color, use background
            clear_color: ClearColorConfig::None,
            order: 1,
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(96, 96),
                ..default()
            }),
            ..default()
        },
        RenderLayers::from_layers(&[CAMERA_2D_LAYER]),
        UiCamera,
    ));

    Ok(())
}

/// Moving camere with MoveRequest. MoveRequest must have only one
pub fn move_camera_with_request(
    time: Res<Time>,
    mut commands: Commands,
    mut q_request: Query<(Entity, &mut CameraMoveRequest)>,
    mut q_main_transform: Query<&mut Transform, (With<MainCamera>, Without<UiCamera>)>,
) -> Result<(), BevyError> {
    for (entity, mut request) in q_request.iter_mut() {
        request.elapsed += time.delta_secs();
        let t = match request.duration {
            CameraMoveDuration::Immediate => 1.0,
            CameraMoveDuration::Duration(duration) => (request.elapsed / duration).min(1.0),
        };

        for (mut transform) in q_main_transform.iter_mut() {
            request.slerp_trasform(ease_in_out_cubic(t), &mut transform);
            tracing::info!("transformed {:?}", transform)
        }

        if t >= 1.0 {
            commands.entity(entity).remove::<CameraMoveRequest>();
        }
    }

    Ok(())
}

fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let f = 2.0 * t - 2.0;
        1.0 + f * f * f / 2.0
    }
}
