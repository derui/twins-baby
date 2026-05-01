use bevy::prelude::*;
use cad_base::{
    body::BodyPerspective,
    id::SketchId,
    sketch::{AttachableTarget, SketchPerspective},
};
use ui_event::{
    command::CreateSketchOnPlaneCommand,
    notification::{Notifications, SketchCreatedNotification},
};

use crate::bevy_app::resource::EngineState;

/// A command to create sketch on the plane.
pub(super) fn on_create_sketch_on_plane(
    trigger: On<CreateSketchOnPlaneCommand>,
    mut engine: ResMut<EngineState>,
    mut writer: MessageWriter<Notifications>,
) -> Result<(), BevyError> {
    let command = trigger.event();
    let mut transaction = engine.0.begin();

    let created_sketch: SketchId;
    let sketch_name: String;

    {
        let Some(sketch_p) = transaction.modify::<SketchPerspective>() else {
            return Err(color_eyre::eyre::eyre!("Can not get sketch perspective").into());
        };

        created_sketch = sketch_p.add_sketch(&AttachableTarget::Plane(*command.plane));
        sketch_name = sketch_p
            .get(&created_sketch)
            .map(|v| (*v.name).clone())
            .expect("Should be found");
    }

    {
        let Some(body_p) = transaction.modify::<BodyPerspective>() else {
            return Err(color_eyre::eyre::eyre!("Can not get sketch perspective").into());
        };

        let Some(body) = body_p.get_mut(&command.plane.body_id()) else {
            return Ok(());
        };

        body.add_sketch(&created_sketch);
    }

    writer.write(
        SketchCreatedNotification {
            correlation_id: command.id.clone(),
            sketch_id: created_sketch.into(),
            name: sketch_name.into(),
        }
        .into(),
    );

    transaction.commit();

    Ok(())
}
