mod body;

use bevy::ecs::{error::BevyError, message::MessageReader};
use bevy::prelude::{App, Commands as BevyCommands, Update};
use ui_event::command::Commands;

use body::on_create_body;

pub trait CommandAppExt {
    fn register_commands(&mut self) -> &mut Self;
}

impl CommandAppExt for App {
    fn register_commands(&mut self) -> &mut Self {
        self.add_systems(Update, dispatch_commands)
            .add_observer(on_create_body)
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
        }
    }
    Ok(())
}
