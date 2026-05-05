use bevy::{
    camera::{Viewport, visibility::RenderLayers},
    prelude::*,
};
use eyre::Result;

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
        duraton: f32,
    },

    /// Special pattern, no-op
    Noop,
}

impl CameraMoveOperation {
    /// Calculate expected camera transform by operation
    pub fn calculate_expectation(
        &self,
        camera_transform: &Transform,
        ui_transform: &Transform,
    ) -> Option<CameraMoveExpectation> {
        match self {
            CameraMoveOperation::ByOrbit(state) => {
                // state must be transformed by pivot, so these calculation only apply camera rotation
                let rotation = Quat::from_euler(EulerRot::YXZ, state.yaw, state.pitch, 0.0);
                let expected_direction = (*camera_transform).with_rotation(rotation).back();

                Some(CameraMoveExpectation {
                    camera_transform: *camera_transform,
                    ui_transform: *ui_transform,
                    duration: None,
                    translation: (state.center + (state.radius * expected_direction)),
                    rotation: rotation,
                    ui_translation: state.radius * expected_direction,
                    ui_rotation: rotation,
                })
            }
            CameraMoveOperation::BySystem { .. } => todo!("not yet implementation"),
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
    ui_transform: Transform,

    /// Expected duration. when minus or less than the frame, apply transform immediately
    duration: Option<f32>,
    translation: Vec3,
    rotation: Quat,
    ui_translation: Vec3,
    ui_rotation: Quat,
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

    fn apply_slerp_to_ui(&self, transform: &mut Transform, time: &SlerpTime) {
        let lerped_time = time.to_lerped_time();

        transform.rotation = self
            .ui_transform
            .rotation
            .lerp(self.ui_rotation, lerped_time);
        transform.translation = self
            .ui_transform
            .translation
            .lerp(self.ui_translation, lerped_time);
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
            viewport.physical_position = window.resolution.physical_size();
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
    mut q_ui_transform: Query<&mut Transform, (With<UiCamera>, Without<MainCamera>)>,
) {
    if let Ok((entity, mut handle, op)) = q_request.single_mut() {
        let expectation = if let Ok(expect) = q_expectation.get(entity) {
            expect.clone()
        } else {
            let Ok(main_transform) = q_main_transform.single() else {
                tracing::warn!("No main camera yet");
                return;
            };
            let Ok(ui_transform) = q_ui_transform.single() else {
                tracing::warn!("No UI camera yet");
                return;
            };
            let Some(expect) = op.calculate_expectation(main_transform, ui_transform) else {
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

        for mut transform in q_ui_transform.iter_mut() {
            expectation.apply_slerp_to_ui(&mut transform, &t);
            // fixed distance in UI
            transform.translation = transform.translation.normalize() * 1.;
        }

        if !t.should_continue() {
            commands.entity(entity).remove::<CameraMoveExpectation>();
            commands.entity(entity).insert(CameraMoveOperation::Noop);
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use bevy::window::WindowResolution;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn did_setup_cameras() {
        // arrange
        let mut app = App::new();
        app.world_mut().spawn(Window {
            resolution: WindowResolution::new(800, 600),
            ..default()
        });
        app.add_systems(Startup, setup_camera);

        // act
        app.update();

        // assert
        assert!(
            app.world_mut()
                .query::<&MainCamera>()
                .single(app.world())
                .is_ok()
        );
        assert_eq!(
            2,
            app.world_mut()
                .query::<&UiCamera>()
                .iter(app.world())
                .count(),
        );
        let (camera, _) = app
            .world_mut()
            .query::<(&Camera, &UiCamera)>()
            .iter(app.world())
            .next()
            .unwrap();
        assert_eq!(camera.order, 1);
        assert!(matches!(camera.clear_color, ClearColorConfig::None));
        assert!(matches!(
            camera.viewport,
            Some(Viewport {
                physical_position: UVec2 { x: 704, y: 0 },
                physical_size: UVec2 { x: 96, y: 96 },
                ..
            })
        ));
    }

    #[test]
    fn move_camera_with_45_degree_pitch() {
        // arrange
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let main_camera = app
            .world_mut()
            .spawn((MainCamera, Transform::default()))
            .id();
        let ui_camera = app.world_mut().spawn((UiCamera, Transform::default())).id();

        app.world_mut().spawn((
            CameraMoveHandle::default(),
            CameraMoveOperation::ByOrbit(PanOrbitOperation {
                center: Vec3::ZERO,
                radius: 5.0,
                upside_down: false,
                pitch: 45_f32.to_radians(),
                yaw: 0.0,
                viewpoint: Vec2::ZERO,
            }),
        ));
        app.add_systems(Update, move_camera_with_request);

        // act
        app.update();

        // assert
        let expected_rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 45_f32.to_radians(), 0.0);

        let main_transform = app.world().get::<Transform>(main_camera).unwrap();
        assert_relative_eq!(
            main_transform.rotation.x,
            expected_rotation.x,
            epsilon = 0.001
        );
        assert_relative_eq!(
            main_transform.rotation.y,
            expected_rotation.y,
            epsilon = 0.001
        );

        let ui_transform = app.world().get::<Transform>(ui_camera).unwrap();
        assert_relative_eq!(
            ui_transform.rotation.x,
            expected_rotation.x,
            epsilon = 0.001
        );
        assert_relative_eq!(
            ui_transform.rotation.y,
            expected_rotation.y,
            epsilon = 0.001
        );
    }

    #[test]
    fn reposition_ui_cameras_on_resize() {
        // arrange
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.world_mut().spawn(Window {
            resolution: WindowResolution::new(1000, 800),
            ..default()
        });

        let nav_cube_entity = app
            .world_mut()
            .spawn((
                Camera {
                    viewport: Some(Viewport {
                        physical_position: UVec2::new(0, 0),
                        physical_size: UVec2::new(96, 96),
                        ..default()
                    }),
                    ..default()
                },
                RenderLayers::layer(CAMERA_UI_LAYER),
                UiCamera,
            ))
            .id();

        let gizmo_entity = app
            .world_mut()
            .spawn((
                Camera {
                    viewport: Some(Viewport {
                        physical_position: UVec2::new(0, 0),
                        physical_size: UVec2::new(96, 96),
                        ..default()
                    }),
                    ..default()
                },
                RenderLayers::layer(CAMERA_UI_LAYER),
                UiCamera,
            ))
            .id();

        app.insert_resource(LastWindowSize::default());
        app.add_systems(Update, reposition_ui_cameras);

        // act
        app.update();

        // assert
        let nav_cube_camera = app.world().get::<Camera>(nav_cube_entity).unwrap();
        assert!(matches!(
            nav_cube_camera.viewport,
            Some(Viewport {
                physical_position: UVec2 { x: 904, y: 0 },
                physical_size: UVec2 { x: 96, y: 96 },
                ..
            })
        ));

        let gizmo_camera = app.world().get::<Camera>(gizmo_entity).unwrap();
        assert!(matches!(
            gizmo_camera.viewport,
            Some(Viewport {
                physical_position: UVec2 { x: 904, y: 704 },
                physical_size: UVec2 { x: 96, y: 96 },
                ..
            })
        ));
    }
}
