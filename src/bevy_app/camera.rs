use bevy::{
    camera::{Viewport, visibility::RenderLayers},
    platform::collections::HashSet,
    prelude::*,
};

pub const CAMERA_3D_LAYER: usize = 0;
pub const CAMERA_CUBE_LAYER: usize = 1;
pub const CAMERA_GIZMO_LAYER: usize = 2;

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
    ui_translation: Option<Vec3>,
    ui_rotation: Option<Quat>,
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
    fn slerp_trasform(&mut self, lerped_time: f32, transform: &mut Transform, ui: bool) {
        if self.expected.is_none() {
            match &self.operation {
                CameraMoveOperation::ByOrbit(state) => {
                    let expected_direction = (*transform)
                        .with_rotation(Quat::from_euler(EulerRot::YXZ, state.yaw, state.pitch, 0.0))
                        .back();
                    self.expected = Some(CameraMoveExpectation {
                        translation: Some(state.center + (state.radius * expected_direction)),
                        rotation: Some(Quat::from_euler(
                            EulerRot::YXZ,
                            state.yaw,
                            state.pitch,
                            0.0,
                        )),
                        ui_translation: Some(state.radius * expected_direction),
                        ui_rotation: Some(Quat::from_euler(
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
                    let rotation = match (pitch, yaw) {
                        (Some(pitch), Some(yaw)) => {
                            Some(Quat::from_euler(EulerRot::YXZ, *yaw, *pitch, 0.0))
                        }
                        (None, Some(yaw)) => Some(Quat::from_euler(EulerRot::YXZ, *yaw, 0.0, 0.0)),
                        (Some(pitch), None) => {
                            Some(Quat::from_euler(EulerRot::YXZ, 0.0, *pitch, 0.0))
                        }
                        (None, None) => None,
                    };
                    self.expected = Some(CameraMoveExpectation {
                        translation: Some(target + position),
                        rotation,
                        ui_translation: Some(*position),
                        ui_rotation: rotation,
                    });
                }
            }
        }

        if !ui {
            if let Some(rotation) = self.expected.as_ref().unwrap().rotation {
                transform.rotation = transform.rotation.lerp(rotation, lerped_time);
            }

            if let Some(translation) = self.expected.as_ref().unwrap().translation {
                transform.translation = transform.translation.lerp(translation, lerped_time);
            }
        } else {
            if let Some(rotation) = self.expected.as_ref().unwrap().ui_rotation {
                transform.rotation = transform.rotation.lerp(rotation, lerped_time);
            }

            if let Some(translation) = self.expected.as_ref().unwrap().ui_translation {
                transform.translation = transform.translation.lerp(translation, lerped_time);
            }
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
    let right = window.resolution.physical_width() - 96;

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
                physical_position: UVec2::new(right, 0),
                physical_size: UVec2::new(96, 96),
                ..default()
            }),
            ..default()
        },
        RenderLayers::from_layers(&[CAMERA_CUBE_LAYER]),
        UiCamera,
    ));

    let bottom = window.resolution.physical_height() - 96;
    // camera for gizmo
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
            order: 2,
            viewport: Some(Viewport {
                physical_position: UVec2::new(right, bottom),
                physical_size: UVec2::new(96, 96),
                ..default()
            }),
            ..default()
        },
        RenderLayers::from_layers(&[CAMERA_GIZMO_LAYER]),
        UiCamera,
    ));

    Ok(())
}

/// Repositions navigation cube and gizmo camera viewports
/// to their fixed screen corners when the window resolution changes.
pub fn reposition_ui_cameras(
    window: Query<&Window>,
    mut cameras: Query<(&mut Camera, &RenderLayers), With<UiCamera>>,
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

    let right = width - 96;
    let bottom = height - 96;

    for (mut camera, render_layers) in &mut cameras {
        if let Some(ref mut viewport) = camera.viewport {
            if render_layers.intersects(&RenderLayers::layer(CAMERA_CUBE_LAYER)) {
                viewport.physical_position = UVec2::new(right, 0);
            } else if render_layers.intersects(&RenderLayers::layer(CAMERA_GIZMO_LAYER)) {
                viewport.physical_position = UVec2::new(right, bottom);
            }
        }
    }

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
            request.slerp_trasform(ease_in_out_cubic(t), &mut transform, false);
        }

        for mut transform in q_ui_transform.iter_mut() {
            request.slerp_trasform(ease_in_out_cubic(t), &mut transform, true);
            // fixed distance in UI
            transform.translation = transform.translation.normalize() * 1.;
        }

        if t >= 1.0 {
            commands.entity(entity).despawn();
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

#[cfg(test)]
mod tests {
    use bevy::window::WindowResolution;

    use super::*;

    #[test]
    fn did_setup_cameres() {
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
        assert!(match camera.clear_color {
            ClearColorConfig::None => true,
            _ => false,
        });
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

        let operation = CameraMoveOperation::ByOrbit(PanOrbitOperation {
            center: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
            pitch: 45_f32.to_radians(),
            yaw: 0.0,
        });

        app.world_mut().spawn(CameraMoveRequest::new(
            operation,
            CameraMoveDuration::Immediate,
        ));

        app.add_systems(Update, move_camera_with_request);

        // act
        app.update();

        // assert
        let main_transform = app.world().get::<Transform>(main_camera).unwrap();
        let expected_rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 45_f32.to_radians(), 0.0);

        assert!(
            (main_transform.rotation.x - expected_rotation.x).abs() < 0.001,
            "rotation.x mismatch: got {}, expected {}",
            main_transform.rotation.x,
            expected_rotation.x
        );
        assert!(
            (main_transform.rotation.y - expected_rotation.y).abs() < 0.001,
            "rotation.y mismatch: got {}, expected {}",
            main_transform.rotation.y,
            expected_rotation.y
        );

        let ui_transform = app.world().get::<Transform>(ui_camera).unwrap();
        assert!(
            (ui_transform.rotation.x - expected_rotation.x).abs() < 0.001,
            "UI rotation.x mismatch: got {}, expected {}",
            ui_transform.rotation.x,
            expected_rotation.x
        );
        assert!(
            (ui_transform.rotation.y - expected_rotation.y).abs() < 0.001,
            "UI rotation.y mismatch: got {}, expected {}",
            ui_transform.rotation.y,
            expected_rotation.y
        );
    }

    #[test]
    fn move_camera_by_vector() {
        // arrange
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let main_camera = app
            .world_mut()
            .spawn((MainCamera, Transform::default()))
            .id();
        let _ui_camera = app.world_mut().spawn((UiCamera, Transform::default())).id();

        let operation = CameraMoveOperation::BySystem {
            target: Vec3::ZERO,
            position: Vec3::new(3.0, 0.0, 0.0),
            pitch: None,
            yaw: None,
        };

        app.world_mut().spawn(CameraMoveRequest::new(
            operation,
            CameraMoveDuration::Immediate,
        ));

        app.add_systems(Update, move_camera_with_request);

        // act
        app.update();

        // assert
        let main_transform = app.world().get::<Transform>(main_camera).unwrap();
        let expected_translation = Vec3::new(3.0, 0.0, 0.0);

        assert!(
            (main_transform.translation.x - expected_translation.x).abs() < 0.001,
            "translation.x mismatch: got {}, expected {}",
            main_transform.translation.x,
            expected_translation.x
        );
        assert!(
            (main_transform.translation.y - expected_translation.y).abs() < 0.001,
            "translation.y mismatch: got {}, expected {}",
            main_transform.translation.y,
            expected_translation.y
        );
        assert!(
            (main_transform.translation.z - expected_translation.z).abs() < 0.001,
            "translation.z mismatch: got {}, expected {}",
            main_transform.translation.z,
            expected_translation.z
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
                RenderLayers::layer(CAMERA_CUBE_LAYER),
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
                RenderLayers::layer(CAMERA_GIZMO_LAYER),
                UiCamera,
            ))
            .id();

        app.insert_resource(LastWindowSize::default());
        app.add_systems(Update, reposition_ui_cameras);

        // act
        app.update();

        // assert
        // Nav cube: top-right corner (1000 - 96 = 904, 0)
        let nav_cube_camera = app.world().get::<Camera>(nav_cube_entity).unwrap();
        assert!(matches!(
            nav_cube_camera.viewport,
            Some(Viewport {
                physical_position: UVec2 { x: 904, y: 0 },
                physical_size: UVec2 { x: 96, y: 96 },
                ..
            })
        ));

        // Gizmo: bottom-right corner (1000 - 96 = 904, 800 - 96 = 704)
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
