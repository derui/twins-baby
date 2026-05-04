use std::ops::Neg;

use bevy::asset::Assets;
use bevy::camera::visibility::{RenderLayers, Visibility};
use bevy::color::palettes::tailwind::BLUE_500;
use bevy::color::palettes::tailwind::GREEN_500;
use bevy::color::palettes::tailwind::RED_500;
use bevy::color::{Alpha, Color};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::ecs::{message::MessageWriter, observer::On};
use bevy::math::Dir3;
use bevy::math::primitives::Plane3d;
use bevy::mesh::{Mesh, Mesh3d, Meshable};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::ResMut;
use bevy::transform::components::Transform;
use cad_base::body::BodyPerspective;
use cad_base::id::BodyId;
use ui_event::Correlation;
use ui_event::command::SwitchActiveBodyCommand;
use ui_event::{
    ObjectType,
    command::CreateBodyCommand,
    notification::{BodyActivatedNotification, BodyCreatedNotification, Notifications},
};

use crate::bevy_app::camera::CAMERA_3D_LAYER;
use crate::bevy_app::command::body::component::BodyBasePlane;
use crate::bevy_app::component::BodyPartType;
use crate::bevy_app::picking::{
    PickingMaterials, update_pointer_click, update_pointer_out, update_pointer_over,
};
use crate::bevy_app::resource::{EngineAppState, EngineState};

#[cfg(test)]
mod tests;

mod component;

/// make a [PickingMaterials] of plane with color
fn make_picking_materials(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    color: Color,
) -> PickingMaterials {
    let mat_normal = materials.add(color.with_alpha(0.3));
    let mat_over = materials.add(color.with_alpha(0.5));

    PickingMaterials {
        normal: mat_normal,
        over: mat_over,
    }
}

/// Register 3 planes for the body.
///
/// # Return
/// The entities of planes. The order is XY, YZ, ZX plane.
fn register_body_base_planes(
    bodies: &BodyPerspective,
    body_id: &BodyId,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> eyre::Result<Vec<Entity>> {
    // all sizes are 1 = 1m
    let mut entities = Vec::new();
    let ref_x = bodies.to_x_plane_ref(body_id).expect("Should get X ref");
    let ref_y = bodies.to_y_plane_ref(body_id).expect("Should get Y ref");
    let ref_z = bodies.to_z_plane_ref(body_id).expect("Should get Z ref");

    let defs = [
        (Dir3::Z, ref_z, Color::from(BLUE_500)),
        (Dir3::X, ref_x, Color::from(RED_500)),
        (Dir3::Y, ref_y, Color::from(GREEN_500)),
    ];

    for (dir, plane_ref, color) in defs {
        let mat = make_picking_materials(materials, color);

        for dir in [dir, dir.neg()] {
            let mat = mat.clone();
            let plane = meshes.add(Plane3d::default().mesh().size(10.0, 10.0).normal(dir));
            let mut entity = commands.spawn((
                Mesh3d(plane),
                MeshMaterial3d(mat.normal.clone()),
                Transform::from_xyz(0., 0., 0.),
                RenderLayers::layer(CAMERA_3D_LAYER),
                Visibility::Hidden,
                BodyBasePlane(plane_ref),
                BodyPartType(ObjectType::Plane(plane_ref)),
                mat,
            ));
            entity.observe(update_pointer_over);
            entity.observe(update_pointer_out);
            entity.observe(update_pointer_click);
            entities.push(entity.id());
        }
    }

    Ok(entities)
}

pub(super) fn on_create_body(
    trigger: On<Correlation<CreateBodyCommand>>,
    mut engine: ResMut<EngineState>,
    mut app_state: ResMut<EngineAppState>,
    mut writer: MessageWriter<Correlation<Notifications>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut transaction = engine.0.begin();

    let Some(body) = transaction.modify::<BodyPerspective>() else {
        tracing::warn!("Can not get body perspective");
        return;
    };

    let body_id = body.add_body();
    let mut name = "Body".to_string();
    if body.rename_body(&body_id, &name).is_err() {
        let count = body.bodies().count();
        name = format!("{}{:03}", &name, count + 1);
        if let Err(e) = body.rename_body(&body_id, &name) {
            tracing::warn!("Failed to rename body: {:?}", e);
            return;
        }
    }

    if let Ok(entities) =
        register_body_base_planes(body, &body_id, &mut commands, &mut meshes, &mut materials)
    {
        app_state.body_planes_map.insert(body_id, entities);
    }

    transaction.commit();

    writer.write(
        trigger.event().correlate(
            BodyCreatedNotification {
                body_id: body_id.into(),
                name: name.into(),
            }
            .into(),
        ),
    );
}

/// A command handler of [SwitchActiveBodyCommand]
pub(super) fn on_switch_active_body(
    trigger: On<Correlation<SwitchActiveBodyCommand>>,
    mut engine: ResMut<EngineState>,
    mut app_state: ResMut<EngineAppState>,
    mut writer: MessageWriter<Correlation<Notifications>>,
) {
    let command = trigger.event();
    let transaction = engine.0.begin();

    let Some(body) = transaction.read::<BodyPerspective>() else {
        tracing::info!("Can not get body perspective");
        return;
    };

    if body.get(&command.body_id).is_some() {
        app_state.active_body = Some(*command.body_id.clone());

        writer.write(
            trigger.event().correlate(
                BodyActivatedNotification {
                    body_id: command.body_id.clone(),
                }
                .into(),
            ),
        );
    } else {
        // This case occurs sometimes. Do not do anything in this
    }
}

/// Update all plane visibilities of the app
pub(super) fn update_plane_visibilities(
    mut commands: Commands,
    app_state: Res<EngineAppState>,
    mut cad_state: ResMut<EngineState>,
    q_planes: Query<Entity, With<BodyBasePlane>>,
) {
    // Make all entities hidden
    for plane in q_planes {
        commands.entity(plane).insert(Visibility::Hidden);
    }

    // When app has active body, active the planes
    let Some(body_id) = app_state.active_body else {
        return;
    };

    let transaction = cad_state.0.begin();

    // No need to show planes when the body already have any feature/sketch
    if let Some(v) = transaction.read::<BodyPerspective>()
        && let Some(body) = v.get(&body_id)
        && body.has_feature()
    {
        return;
    }

    for &plane in app_state
        .body_planes_map
        .get(&body_id)
        .unwrap_or(&Vec::<Entity>::new())
    {
        commands.entity(plane).insert(Visibility::Visible);
    }
}
