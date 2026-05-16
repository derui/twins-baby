use bevy::{
    camera::{Viewport, visibility::RenderLayers},
    prelude::*,
};
use eyre::Result;

#[cfg(test)]
mod tests;

use crate::bevy_app::ui::components::HudRotation;

pub const CAMERA_3D_LAYER: usize = 0;
pub const CAMERA_UI_LAYER: usize = 1;

/// This module provides 3D camera basic functionally in Bevy.
#[derive(Component)]
pub struct UiCamera;

#[derive(Component)]
pub struct MainCamera;

/// Tracks the previous window size to detect resolution changes
/// for repositioning UI camera viewports.
#[derive(Resource, Default)]
pub(crate) struct LastWindowSize {
    width: u32,
    height: u32,
}

/// Internal state for the pan-orbit operation
#[derive(Component, Clone, PartialEq, Debug)]
pub struct PanOrbitOperation {
    pub center: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
    /// A point to start point to orbit
    pub viewpoint: Vec2,
}

impl Default for PanOrbitOperation {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            radius: 1.0,
            upside_down: false,
            pitch: 0.0,
            yaw: 0.0,
            viewpoint: Vec2::ZERO,
        }
    }
}

/// Request to move camera to a new position
#[derive(Debug, Clone, PartialEq, Component)]
pub enum CameraMoveOperation {
    /// Do operation by orbit,
    ByOrbit(PanOrbitOperation),

    /// Do operation by system. This operation requires absolute position of camera.
    BySystem {
        target: Vec3,
        position: Vec3,
        pitch: Option<f32>,
        yaw: Option<f32>,
        duration: f32,
    },

    /// Special pattern, no-op
    Noop,
}

impl CameraMoveOperation {
    /// Calculate expected camera transform by operation
    pub fn calculate_expectation(
        &self,
        camera_transform: &Transform,
    ) -> Option<CameraMoveExpectation> {
        match self {
            CameraMoveOperation::ByOrbit(state) => {
                // state must be transformed by pivot, so these calculation only apply camera rotation
                let rotation = Quat::from_euler(EulerRot::YXZ, state.yaw, state.pitch, 0.0);
                let expected_direction = (*camera_transform).with_rotation(rotation).back();

                Some(CameraMoveExpectation {
                    camera_transform: *camera_transform,
                    duration: None,
                    translation: (state.center + (state.radius * expected_direction)),
                    rotation,
                })
            }
            CameraMoveOperation::BySystem {
                target,
                position,
                duration,
                ..
            } => {
                // make rotation of camera's look angle
                let rotation = Quat::from_rotation_arc(Vec3::Y, (*target - *position).normalize());

                Some(CameraMoveExpectation {
                    camera_transform: *camera_transform,
                    duration: Some(*duration),
                    // TODO: need to define radius by scene
                    translation: *position,
                    rotation,
                })
            }
            CameraMoveOperation::Noop => None,
        }
    }
}

/// Expectation of camera movement calculation
#[derive(Component, Debug, Clone)]
pub struct CameraMoveExpectation {
    /// transforms for expected. These are used to move camera while duration.
    /// When duration is `1.0`, these will be placed naturally translation + rotation applied.
    camera_transform: Transform,

    /// Expected duration. when minus or less than the frame, apply transform immediately
    duration: Option<f32>,
    translation: Vec3,
    rotation: Quat,
}

enum SlerpTime {
    Immedietely,
    InSlerp(f32),
    Finished,
}

impl SlerpTime {
    /// Return true if the camera movement should continue, otherwise false.
    fn should_continue(&self) -> bool {
        match self {
            Self::Immedietely => false,
            Self::InSlerp(_) => true,
            Self::Finished => false,
        }
    }

    fn to_lerped_time(&self) -> f32 {
        match self {
            Self::Immedietely => 1.0,
            Self::InSlerp(e) => ease_in_out_cubic(*e),
            Self::Finished => 1.0,
        }
    }
}

fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let f = 2.0 * t - 2.0;
        1.0 + f * f * f / 2.0
    }
}

impl CameraMoveExpectation {
    fn to_slerp_time(&self, elapsed: f32) -> SlerpTime {
        let duration = self.duration.unwrap_or(-1.0);
        if duration <= 0.0 {
            SlerpTime::Immedietely
        } else if elapsed < duration {
            SlerpTime::InSlerp((elapsed / duration).min(1.0))
        } else {
            SlerpTime::Finished
        }
    }

    fn apply_slerp(&self, transform: &mut Transform, time: &SlerpTime) {
        let lerped_time = time.to_lerped_time();

        transform.rotation = self
            .camera_transform
            .rotation
            .lerp(self.rotation, lerped_time);
        transform.translation = self
            .camera_transform
            .translation
            .lerp(self.translation, lerped_time);
    }

    fn apply_slerp_to_ui(&self, time: &SlerpTime) -> Quat {
        let lerped_time = time.to_lerped_time();

        self.camera_transform
            .rotation
            .lerp(self.rotation, lerped_time)
    }
}

/// The component of handling camera movement.
#[derive(Component, Debug)]
pub struct CameraMoveHandle {
    elapsed: f32,
}

impl Default for CameraMoveHandle {
    fn default() -> Self {
        Self { elapsed: 0.0 }
    }
}

impl CameraMoveHandle {
    /// Reset for next movement.
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
    }
}

/// Setup camera with pan-orbit controller
pub fn setup_camera(mut commands: Commands, window: Query<&Window>) {
    // spawn move handle
    commands.spawn((CameraMoveHandle::default(), CameraMoveOperation::Noop));

    commands.spawn((
        Camera3d::default(),
        RenderLayers::from_layers(&[CAMERA_3D_LAYER]),
        MainCamera,
    ));

    let window = window.single().unwrap();

    // UI camera
    commands.spawn((
        Camera3d::default(),
        // use this camera as 2D
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: Default::default(),
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
                physical_size: window.resolution.physical_size(),
                ..default()
            }),
            ..default()
        },
        RenderLayers::from_layers(&[CAMERA_UI_LAYER]),
        UiCamera,
    ));
}

/// Repositions navigation cube and gizmo camera viewports
/// to their fixed screen corners when the window resolution changes.
pub fn reposition_ui_cameras(
    window: Query<&Window>,
    mut cameras: Query<&mut Camera, With<UiCamera>>,
    mut last_size: ResMut<LastWindowSize>,
) -> Result<(), BevyError> {
    let window = window.single()?;
    let width = window.resolution.physical_width();
    let height = window.resolution.physical_height();

    if width == last_size.width && height == last_size.height {
        return Ok(());
    }
    last_size.width = width;
    last_size.height = height;

    for mut camera in &mut cameras {
        if let Some(ref mut viewport) = camera.viewport {
            viewport.physical_size = window.resolution.physical_size();
        }
    }

    Ok(())
}

/// Moving camere with MoveRequest. MoveRequest must have only one
pub fn move_camera_with_request(
    time: Res<Time>,
    mut commands: Commands,
    mut q_request: Query<(Entity, &mut CameraMoveHandle, &CameraMoveOperation)>,
    q_expectation: Query<&CameraMoveExpectation>,
    mut q_main_transform: Query<&mut Transform, With<MainCamera>>,
    mut q_hud_rotation: Query<&mut HudRotation>,
) {
    if let Ok((entity, mut handle, op)) = q_request.single_mut() {
        let expectation = if let Ok(expect) = q_expectation.get(entity) {
            expect.clone()
        } else {
            let Ok(main_transform) = q_main_transform.single() else {
                tracing::warn!("No main camera yet");
                return;
            };
            let Some(expect) = op.calculate_expectation(main_transform) else {
                return;
            };

            commands.entity(entity).insert(expect.clone());
            expect
        };

        handle.elapsed += time.delta_secs();

        let t = expectation.to_slerp_time(handle.elapsed);

        for mut transform in q_main_transform.iter_mut() {
            expectation.apply_slerp(&mut transform, &t);
        }

        for mut transform in q_hud_rotation.iter_mut() {
            transform.update(expectation.apply_slerp_to_ui(&t));
        }

        if !t.should_continue() {
            commands.entity(entity).remove::<CameraMoveExpectation>();
            commands.entity(entity).insert(CameraMoveOperation::Noop);
        }
    }
}
