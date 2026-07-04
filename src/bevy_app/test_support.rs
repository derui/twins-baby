use bevy::{camera::CameraPlugin, ecs::system::RunSystemOnce as _, input::InputPlugin, prelude::*};

use crate::bevy_app::{camera::setup_camera, resource::AppResourceExt as _};

/// A trait for setting up the twins-baby app for testing purposes.
///
/// This setup should be same as default configuration. Use this trait when the system is too complex, such as need
/// GlobalTransform and camera, and other application resources needs.
/// But keep setup simple as possible, just only need some resources, does not need use this.
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

        self
    }
}
