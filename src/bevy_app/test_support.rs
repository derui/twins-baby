use bevy::{
    camera::{CameraPlugin, CameraProjection as _},
    ecs::system::RunSystemOnce as _,
    input::InputPlugin,
    math::DVec2,
    prelude::*,
    window::PrimaryWindow,
};

use crate::bevy_app::{
    camera::{MainCamera, setup_camera},
    resource::AppResourceExt as _,
};

/// A trait for setting up the twins-baby app for testing purposes.
///
/// This setup should be same as default configuration. Use this trait when the system is too complex, such as need
/// GlobalTransform and camera, and other application resources needs.
/// But keep setup simple as possible, just only need some resources, does not need use this.
///
/// Support this trait are:
/// - Fixed size primary window and main camera configuration
/// - setup application resources and some required resources
/// - add some plugins minimal for test.
pub trait TestEnv {
    /// Setup whole resources for twins-baby app.
    fn setup_test_env(&mut self) -> &mut Self;
}

impl TestEnv for App {
    fn setup_test_env(&mut self) -> &mut Self {
        self.init_app_resources();

        self.add_plugins(InputPlugin)
            .add_plugins(bevy::window::WindowPlugin {
                primary_window: Some(Window {
                    resolution: bevy::window::WindowResolution::new(800, 600),
                    ..Default::default()
                }),
                ..default()
            })
            .add_plugins((TransformPlugin, CameraPlugin))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>();

        let world = self.world_mut();
        world
            .run_system_once(setup_camera)
            .expect("should be success to setup camera");

        {
            let camera_entity = world
                .query_filtered::<Entity, With<MainCamera>>()
                .single(world)
                .unwrap();

            // initialize with physical view
            let mut camera = world.get_mut::<Camera>(camera_entity).unwrap();
            camera.viewport = Some(bevy::camera::Viewport {
                physical_size: UVec2::new(800, 600),
                ..default()
            });
            camera.computed.target_info = Some(bevy::camera::RenderTargetInfo {
                physical_size: UVec2::new(800, 600),
                scale_factor: 1.0,
            });

            let mut projection = PerspectiveProjection::default();
            projection.update(800.0, 600.0);
            camera.computed.clip_from_view = projection.get_clip_from_view();
        }

        self
    }
}

/// This trait provides common operation on test.
pub trait WindowOp {
    /// Move primary window cursor to the [position]
    fn move_cursor_to(&mut self, position: DVec2);

    /// Move cursor on primary window to [position] and click left mouse button, then run [system] and then release that.
    ///
    ///
    fn click_at<T>(&mut self, position: DVec2, system: T)
    where
        T: FnMut(&mut World) -> ();
}

impl WindowOp for App {
    fn move_cursor_to(&mut self, position: DVec2) {
        let world = self.world_mut();
        let window_entity = world
            .query_filtered::<Entity, With<PrimaryWindow>>()
            .single(world)
            .unwrap();
        world
            .get_mut::<Window>(window_entity)
            .unwrap()
            .set_physical_cursor_position(Some(position));
    }

    fn click_at<T>(&mut self, position: DVec2, mut system: T)
    where
        T: FnMut(&mut World) -> (),
    {
        self.move_cursor_to(position);

        let mut world = self.world_mut();
        world
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);

        system(&mut world);
        world.flush();
        world
            .resource_mut::<ButtonInput<MouseButton>>()
            .release(MouseButton::Left);
    }
}
