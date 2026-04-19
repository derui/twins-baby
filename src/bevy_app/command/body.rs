use bevy::ecs::{error::BevyError, message::MessageWriter, observer::On};
use bevy::prelude::ResMut;
use cad_base::body::BodyPerspective;
use ui_event::command::SwitchActiveBodyCommand;
use ui_event::{
    command::CreateBodyCommand,
    notification::{BodyActivatedNotification, BodyCreatedNotification, Notifications},
};

use crate::bevy_app::resource::{EngineAppState, EngineState};

pub(super) fn on_create_body(
    trigger: On<CreateBodyCommand>,
    mut engine: ResMut<EngineState>,
    mut writer: MessageWriter<Notifications>,
) -> Result<(), BevyError> {
    let command = trigger.event();
    let mut transaction = engine.0.begin();

    let Some(body) = transaction.modify::<BodyPerspective>() else {
        return Err(color_eyre::eyre::eyre!("Can not get body perspective").into());
    };

    let body_id = body.add_body();
    let mut name = (*command.name).clone();
    if body.rename_body(&body_id, &name).is_err() {
        let count = body.bodies().count();
        name = format!("{}{:03}", &name, count + 1);
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
    transaction.commit();

    Ok(())
}

/// A command handler of [SwitchActiveBodyCommand]
pub(super) fn on_switch_active_body(
    trigger: On<SwitchActiveBodyCommand>,
    mut engine: ResMut<EngineState>,
    mut app_state: ResMut<EngineAppState>,
    mut writer: MessageWriter<Notifications>,
) -> Result<(), BevyError> {
    let command = trigger.event();
    let transaction = engine.0.begin();

    let Some(body) = transaction.read::<BodyPerspective>() else {
        return Err(color_eyre::eyre::eyre!("Can not get body perspective").into());
    };

    if body.get(&(command.body_id)).is_some() {
        app_state.active_body = Some(*command.body_id.clone());

        writer.write(
            BodyActivatedNotification {
                correlation_id: command.id.clone(),
                body_id: command.body_id.clone(),
            }
            .into(),
        );
    } else {
        // This case occurs sometimes. Do not do anything in this
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use bevy::ecs::{message::Messages, world::World};
    use cad_base::body::BodyPerspective;
    use eyre::Result;
    use pretty_assertions::assert_eq;
    use ui_event::{
        CommandId,
        command::{CreateBodyCommand, SwitchActiveBodyCommand},
        notification::{
            BodyActivatedNotification, BodyCreatedNotification, Notification, Notifications,
        },
    };

    use crate::bevy_app::resource::{EngineAppState, EngineState};

    use super::*;

    fn make_world() -> World {
        let mut world = World::new();
        world.init_resource::<Messages<Notifications>>();
        world.init_resource::<EngineState>();
        world.init_resource::<EngineAppState>();
        world.add_observer(on_create_body);
        world.add_observer(on_switch_active_body);
        world
    }

    #[test]
    fn writes_notification_with_given_name_for_unique_name() {
        // Arrange
        let mut world = make_world();

        // Act
        world.trigger(CreateBodyCommand {
            id: CommandId::new(1).into(),
            name: "body1".to_string().into(),
        });
        world.flush();

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
    fn writes_notification_with_fallback_name_when_name_already_exists() -> Result<()> {
        // Arrange
        let mut world = make_world();
        {
            let mut engine = world.resource_mut::<EngineState>();
            let mut tx = engine.0.begin();
            let bodies = tx.modify::<BodyPerspective>().unwrap();
            let id = bodies.add_body();
            bodies.rename_body(&id, "body1").unwrap();
            tx.commit();
        }

        // Act
        world.trigger(CreateBodyCommand {
            id: CommandId::new(1).into(),
            name: "body1".to_string().into(),
        });
        world.flush();

        // Assert
        let messages = world.resource::<Messages<Notifications>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        let notif = notifications[0]
            .select_ref::<BodyCreatedNotification>()
            .unwrap();
        assert_eq!(*notif.name, "body1003");
        Ok(())
    }

    #[test]
    fn switch_active_body_writes_notification_and_updates_app_state() -> Result<()> {
        // Arrange
        let mut world = make_world();
        let body_id = {
            let mut engine = world.resource_mut::<EngineState>();
            let mut tx = engine.0.begin();
            let bodies = tx.modify::<BodyPerspective>().unwrap();
            let id = bodies.add_body();
            bodies.rename_body(&id, "body1").unwrap();
            tx.commit();
            id
        };

        // Act
        world.trigger(SwitchActiveBodyCommand {
            id: CommandId::new(1).into(),
            body_id: body_id.into(),
        });
        world.flush();

        // Assert
        let messages = world.resource::<Messages<Notifications>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        assert_eq!(notifications.len(), 1);
        let notif = notifications[0]
            .select_ref::<BodyActivatedNotification>()
            .unwrap();
        assert_eq!(*notif.body_id, body_id);
        let app_state = world.resource::<EngineAppState>();
        assert_eq!(app_state.active_body, Some(body_id));
        Ok(())
    }

    #[test]
    fn switch_active_body_returns_error_when_body_not_found() {
        // Arrange
        let mut world = make_world();
        let nonexistent_body_id = cad_base::id::BodyId::from(9999);

        // Act
        world.trigger(SwitchActiveBodyCommand {
            id: CommandId::new(1).into(),
            body_id: nonexistent_body_id.into(),
        });
        world.flush();

        // Assert
        let messages = world.resource::<Messages<Notifications>>();
        let mut cursor = messages.get_cursor();
        let notifications: Vec<_> = cursor.read(messages).collect();
        assert_eq!(notifications.len(), 0);
        let app_state = world.resource::<EngineAppState>();
        assert_eq!(app_state.active_body, None);
    }
}
