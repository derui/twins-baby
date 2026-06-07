mod body;
mod sketch;

use bevy::ecs::{error::BevyError, message::MessageReader};
use bevy::prelude::{App, Commands as BevyCommands, Update};
use ui_event::Correlation;
use ui_event::command::Commands;

use body::on_create_body;

use crate::bevy_app::command::body::{on_switch_active_body, update_plane_visibilities};
use crate::bevy_app::command::sketch::{
    on_activate_sketch, on_create_sketch_on_plane, on_request_geometry_creation_command,
};

pub trait CommandAppExt {
    /// Register all commands to the App
    fn register_commands(&mut self) -> &mut Self;
}

impl CommandAppExt for App {
    fn register_commands(&mut self) -> &mut Self {
        self.add_systems(Update, dispatch_commands)
            .add_systems(Update, update_plane_visibilities)
            .add_observer(on_create_body)
            .add_observer(on_switch_active_body)
            .add_observer(on_create_sketch_on_plane)
            .add_observer(on_activate_sketch)
            .add_observer(on_request_geometry_creation_command)
    }
}

fn dispatch_commands(
    mut reader: MessageReader<Correlation<Commands>>,
    mut commands: BevyCommands,
) -> Result<(), BevyError> {
    for cmd in reader.read() {
        match &*cmd.data {
            Commands::CreateSketchOnSelected(c) => commands.trigger(cmd.correlate(c.clone())),
            Commands::CreateBody(c) => commands.trigger(cmd.correlate(c.clone())),
            Commands::SwitchActiveBody(c) => commands.trigger(cmd.correlate(c.clone())),
            Commands::ActivateSketch(c) => commands.trigger(cmd.correlate(c.clone())),
            Commands::RequestGeometryCreation(c) => commands.trigger(cmd.correlate(c.clone())),
            Commands::CancelCurrentGeometryCreation(c) => {
                commands.trigger(cmd.correlate(c.clone()))
            }
        }
    }
    Ok(())
}
