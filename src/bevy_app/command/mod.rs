mod body;

use bevy::ecs::{error::BevyError, message::MessageReader};
use bevy::prelude::{App, Commands as BevyCommands, Update};
use ui_event::command::Commands;

use body::on_create_body;

use crate::bevy_app::command::body::{on_switch_active_body, update_plane_visibilities};

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
    }
}

fn dispatch_commands(
    mut reader: MessageReader<Commands>,
    mut commands: BevyCommands,
) -> Result<(), BevyError> {
    for cmd in reader.read() {
        match cmd {
            Commands::InitiateSketchCreation(c) => commands.trigger(c.clone()),
            Commands::SelectSketchPlane(c) => commands.trigger(c.clone()),
            Commands::CancelSketchCreation(c) => commands.trigger(c.clone()),
            Commands::ConfirmSketchCreation(c) => commands.trigger(c.clone()),
            Commands::CreateBody(c) => commands.trigger(c.clone()),
            Commands::SwitchActiveBody(c) => commands.trigger(c.clone()),
        }
    }
    Ok(())
}
