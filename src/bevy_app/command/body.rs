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
