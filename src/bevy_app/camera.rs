use bevy::{
    log::tracing,
    platform::collections::HashSet,
    prelude::*,
    render::{
        camera::{ScalingMode, Viewport},
        render_resource::AsBindGroupShaderType,
        view::RenderLayers,
    },
};

pub const CAMERA_3D_LAYER: usize = 0;
pub const CAMERA_2D_LAYER: usize = 1;

/// This module provides 3D camera basic functionally in Bevy.
#[derive(Component)]
pub struct UiCamera;

#[derive(Component)]
pub struct MainCamera;

/// Internal state for the pan-orbit operation
#[derive(Component, Clone, PartialEq, Debug)]
pub struct PanOrbitOperation {
    pub center: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for PanOrbitOperation {
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

/// Request to move camera to a new position
#[derive(Debug, Clone, PartialEq)]
pub enum CameraMoveOperation {
    /// Do operation by orbit,
    ByOrbit(PanOrbitOperation),

    /// Do operation by system. This operation requires absolute position of camera.
    BySystem {
        target: Vec3,
        position: Vec3,
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
        if self.expected.is_none() {
            match &self.operation {
                CameraMoveOperation::ByOrbit(state) => {
                    self.expected = Some(CameraMoveExpectation {
                        translation: Some(state.center + (state.radius * transform.back())),
                        rotation: Some(Quat::from_euler(
                            EulerRot::YXZ,
                            state.yaw,
                            state.pitch,
                            0.0,
                        )),
                    });
                }
                CameraMoveOperation::BySystem {
                    target,
                    position,
                    pitch,
                    yaw,
                } => {
                    self.expected = Some(CameraMoveExpectation {
                        translation: Some(target + position),
                        rotation: match (pitch, yaw) {
                            (Some(pitch), Some(yaw)) => {
                                Some(Quat::from_euler(EulerRot::YXZ, *yaw, *pitch, 0.0))
                            }
                            (None, Some(yaw)) => {
                                Some(Quat::from_euler(EulerRot::YXZ, *yaw, 0.0, 0.0))
                            }
                            (Some(pitch), None) => {
                                Some(Quat::from_euler(EulerRot::YXZ, 0.0, *pitch, 0.0))
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
    mut q_main_transform: Query<&mut Transform, With<MainCamera>>,
    mut q_ui_transform: Query<&mut Transform, (With<UiCamera>, Without<MainCamera>)>,
) -> Result<(), BevyError> {
    let len = q_request.iter().len();
    let mut removed = HashSet::<Entity>::new();
    if len > 1 {
        for (entity, _) in q_request.iter_mut().take(len - 1) {
            commands.entity(entity).remove::<CameraMoveRequest>();
            removed.insert(entity);
        }
    }

    for (entity, mut request) in q_request.iter_mut() {
        if removed.contains(&entity) {
            continue;
        }

        request.elapsed += time.delta_secs();
        let t = match request.duration {
            CameraMoveDuration::Immediate => 1.0,
            CameraMoveDuration::Duration(duration) => (request.elapsed / duration).min(1.0),
        };

        for mut transform in q_main_transform.iter_mut() {
            request.slerp_trasform(ease_in_out_cubic(t), &mut transform);
            tracing::info!("transformed {:?}", transform)
        }

        for mut transform in q_ui_transform.iter_mut() {
            request.slerp_trasform(ease_in_out_cubic(t), &mut transform);
            // fixed distance in UI
            transform.translation = transform.translation.normalize() * 3.0;
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
