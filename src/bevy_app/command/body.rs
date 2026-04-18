use bevy::ecs::{error::BevyError, message::MessageWriter, observer::On};
use bevy::prelude::ResMut;
use cad_base::body::BodyPerspective;
use ui_event::{
    command::CreateBodyCommand,
    notification::{BodyCreatedNotification, Notifications},
};

use crate::bevy_app::resource::EngineState;

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

#[cfg(test)]
mod tests {
    use bevy::ecs::{message::Messages, world::World};
    use cad_base::body::BodyPerspective;
    use eyre::Result;
    use pretty_assertions::assert_eq;
    use ui_event::{
        CommandId,
        command::CreateBodyCommand,
        notification::{BodyCreatedNotification, Notification, Notifications},
    };

    use crate::bevy_app::resource::EngineState;

    use super::*;

    fn make_world() -> World {
        let mut world = World::new();
        world.init_resource::<Messages<Notifications>>();
        world.init_resource::<EngineState>();
        world.add_observer(on_create_body);
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
}
