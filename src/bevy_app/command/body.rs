use bevy::ecs::{error::BevyError, message::MessageWriter};
use cad_base::{CadEngine, body::BodyPerspective};
use ui_event::{
    command::{Command, Commands, CreateBodyCommand},
    notification::{BodyCreatedNotification, Notifications},
};

use crate::bevy_app::command::Handler;

#[derive(Debug)]
pub struct CreateBodyCommandHandler;

impl Handler for CreateBodyCommandHandler {
    fn handle(
        &self,
        command: &Commands,
        engine: &mut CadEngine,
        writer: &mut MessageWriter<Notifications>,
    ) -> eyre::Result<(), BevyError> {
        let Some(command) = command.select_ref::<CreateBodyCommand>() else {
            return Ok(());
        };

        let mut transaction = engine.begin();

        {
            let Some(body) = transaction.modify::<BodyPerspective>() else {
                return Err(color_eyre::eyre::eyre!("Can not get body perspective").into());
            };

            let body_id = body.add_body();
            let mut name = (*command.name).clone();
            if let Err(_) = body.rename_body(&body_id, &name) {
                name = format!("{}{}", &name, "_new");
                body.rename_body(&body_id, &name)?;
            }

            writer.write(
                BodyCreatedNotification {
                    correlation_id: command.id.clone(),
                    body_id: body_id.into(),
                    name: name.into(),
                }
                .into(),
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::{message::Messages, system::RunSystemOnce, world::World};
    use cad_base::id::BodyId;
    use cad_base::{CadEngine, body::BodyPerspective};
    use pretty_assertions::assert_eq;
    use ui_event::{
        CommandId,
        command::{Commands, CreateBodyCommand, InitiateSketchCreationCommand},
        notification::{BodyCreatedNotification, Notification, Notifications},
    };

    use super::*;

    fn make_world() -> World {
        let mut world = World::new();
        world.init_resource::<Messages<Notifications>>();
        world
    }

    #[test]
    fn returns_ok_without_notification_for_non_matching_command() {
        // Arrange
        let handler = CreateBodyCommandHandler;
        let mut engine = CadEngine::new();
        let command = Commands::InitiateSketchCreation(InitiateSketchCreationCommand {
            id: CommandId::new(1).into(),
            body: BodyId::new(1).into(),
        });
        let mut world = make_world();

        // Act
        let result: eyre::Result<(), BevyError> = world
            .run_system_once(move |mut writer: MessageWriter<Notifications>| {
                handler.handle(&command, &mut engine, &mut writer)
            })
            .unwrap();

        // Assert
        assert!(result.is_ok());
        let messages = world.resource::<Messages<Notifications>>();
        let mut cursor = messages.get_cursor();
        assert_eq!(cursor.read(messages).count(), 0);
    }

    #[test]
    fn writes_notification_with_given_name_for_unique_name() {
        // Arrange
        let handler = CreateBodyCommandHandler;
        let mut engine = CadEngine::new();
        let command = Commands::CreateBody(CreateBodyCommand {
            id: CommandId::new(1).into(),
            name: "body1".to_string().into(),
        });
        let mut world = make_world();

        // Act
        let system_result: eyre::Result<(), BevyError> = world
            .run_system_once(
                move |mut writer: MessageWriter<Notifications>| -> eyre::Result<(), BevyError> {
                    handler.handle(&command, &mut engine, &mut writer)
                },
            )
            .unwrap();
        system_result.unwrap();

        // Assert
        let messages = world.resource::<Messages<Notifications>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        assert_eq!(notifications.len(), 1);
        let notif = notifications[0]
            .select_ref::<BodyCreatedNotification>()
            .unwrap();
        assert_eq!(*notif.name, "body1");
    }

    #[test]
    fn writes_notification_with_fallback_name_when_name_already_exists() {
        // Arrange
        let handler = CreateBodyCommandHandler;
        let mut engine = CadEngine::new();
        {
            let mut tx = engine.begin();
            let bodies = tx.modify::<BodyPerspective>().unwrap();
            let id = bodies.add_body();
            bodies.rename_body(&id, "body1").unwrap();
            tx.commit();
        }
        let command = Commands::CreateBody(CreateBodyCommand {
            id: CommandId::new(1).into(),
            name: "body1".to_string().into(),
        });
        let mut world = make_world();

        // Act
        let system_result: eyre::Result<(), BevyError> = world
            .run_system_once(
                move |mut writer: MessageWriter<Notifications>| -> eyre::Result<(), BevyError> {
                    handler.handle(&command, &mut engine, &mut writer)
                },
            )
            .unwrap();
        system_result.unwrap();

        // Assert
        let messages = world.resource::<Messages<Notifications>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        let notif = notifications[0]
            .select_ref::<BodyCreatedNotification>()
            .unwrap();
        assert_eq!(*notif.name, "body1_new");
    }
}
