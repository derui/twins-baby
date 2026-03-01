use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use bevy::ecs::{
    error::BevyError,
    message::{MessageReader, MessageWriter},
    resource::Resource,
    system::{Res, ResMut},
};
use cad_base::CadEngine;
use eyre::{Result, eyre};
use ui_event::{
    command::{Command, Commands},
    notification::Notifications,
};

use crate::bevy_app::resource::EngineState;

pub trait Handler {
    /// Handle the command with mutable engine.
    ///
    /// # Summary
    /// All handler handles a command, mutate engine or send notification for the command or not.
    fn handle(
        &self,
        command: &Commands,
        engine: &mut CadEngine,
        writer: &mut MessageWriter<Notifications>,
    ) -> Result<(), BevyError>;
}

/// Resources of the command handler.
#[derive(Resource)]
pub struct HandlerRegistrar {
    /// handlers for the command. Each handler must be tightly coupled a enum in [Commands]
    handlers: HashMap<TypeId, Box<dyn Handler + Send + Sync>>,
}

impl HandlerRegistrar {
    /// Make a new [HandlerRegistrar]
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Register a handler for the command. Do not avoid to register for [Commands]. It will not dispatch ever.
    pub fn register<T: Command + 'static>(&mut self, handler: Box<dyn Handler + Send + Sync>) {
        self.handlers.insert(TypeId::of::<T>(), handler);
    }

    /// Dispatch the command to the handler. If no handler is found, return an error.
    fn dispatch(
        &self,
        command: &Commands,
        engine: &mut CadEngine,
        writer: &mut MessageWriter<Notifications>,
    ) -> Result<(), BevyError> {
        let type_id = command.type_id();

        if let Some(handler) = self.handlers.get(&type_id) {
            handler.handle(command, engine, writer)
        } else {
            Err(eyre!("No handler found for the command: {:?}", command).into())
        }
    }
}

/// System to setup command handlers. This system should be run once at the startup of the app.
pub fn setup_command_handlers(_registrar: ResMut<HandlerRegistrar>) {
    // Register handlers for commands here.
    // Example:
    // registrar.register::<MyCommand>(Box::new(MyCommandHandler));
}

/// System to handle the command.
pub fn command_system(
    registrar: Res<HandlerRegistrar>,
    mut engine: ResMut<EngineState>,
    mut writer: MessageWriter<Notifications>,
    mut commands: MessageReader<Commands>,
) -> Result<(), BevyError> {
    for command in commands.read() {
        registrar.dispatch(command, &mut engine.0, &mut writer)?
    }

    Ok(())
}
