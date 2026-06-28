use bevy::{
    app::App,
    ecs::{system::RunSystemOnce, world::World},
};

use crate::bevy_app::{
    camera::setup_camera, component::ui::HudRotation, resource::AppResourceExt as _, ui::AppUiExt,
};

/// A trait for setting up the twins-baby app for testing purposes.
///
/// This setup should be same as default configuration. Use this trait when the system is too complex, such as need
/// GlobalTransform and camera, and other application resources needs.
/// But keep setup simple as possible, just only need some resources, does not need use this.
pub trait AppTester {
    /// Setup whole resources for twins-baby app.
    fn setup(&mut self) -> &mut Self;
}

impl AppTester for App {
    fn setup(&mut self) -> &mut Self {
        self.init_app_resources();

        let world = self.world_mut();
        world
            .run_system_once(setup_camera)
            .expect("should be success to setup camera");

        self.init_ui()
    }
}
