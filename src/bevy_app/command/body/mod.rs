use bevy::asset::{Assets, Handle};
use bevy::camera::visibility::{RenderLayers, Visibility};
use bevy::color::palettes::tailwind::CYAN_500;
use bevy::color::{Alpha, Color};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EntityEvent as _;
use bevy::ecs::message::MessageReader;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::ecs::{error::BevyError, message::MessageWriter, observer::On};
use bevy::math::Dir3;
use bevy::math::primitives::Plane3d;
use bevy::mesh::{Mesh, Mesh3d, Meshable};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::picking::events::{Click, Out, Over, Pointer};
use bevy::prelude::ResMut;
use bevy::transform::components::Transform;
use cad_base::body::{BodyPerspective, PlaneRef};
use cad_base::id::BodyId;
use ui_event::command::SwitchActiveBodyCommand;
use ui_event::{
    command::CreateBodyCommand,
    notification::{BodyActivatedNotification, BodyCreatedNotification, Notifications},
};

use crate::bevy_app::camera::CAMERA_3D_LAYER;
use crate::bevy_app::command::body::event::InternalSelectObject;
use crate::bevy_app::component::ObjectType;
use crate::bevy_app::resource::{EngineAppState, EngineState};

#[cfg(test)]
mod tests;

mod event;

// components

/// A marker compoment
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct BodyBasePlane(pub PlaneRef);

/// Return a obverver for [Pointer<Over>] event to update plane material to `material_over`
fn update_plane_over(
    material_over: Handle<StandardMaterial>,
) -> impl Fn(On<Pointer<Over>>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = material_over.clone()
        }
    }
}

/// Return a obverver for [Pointer<Out>] event to update plane material to `material_normal`
fn update_plane_out(
    material_normal: Handle<StandardMaterial>,
) -> impl Fn(On<Pointer<Out>>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = material_normal.clone()
        }
    }
}

/// Return a obverver for [Pointer<Out>] event to update plane material to `material_normal`
fn update_plane_selection(
    material_normal: Handle<StandardMaterial>,
) -> impl Fn(On<Pointer<Click>>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = material_normal.clone()
        }
    }
}

fn update_plane_click(
    event: On<Pointer<Click>>,
    mut commands: MessageWriter<InternalSelectObject>,
) {
    commands.write(InternalSelectObject {
        entity: event.event_target().into(),
    });
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
    let mat_normal = materials.add(Color::from(CYAN_500).with_alpha(0.3));
    let mat_over = materials.add(Color::from(CYAN_500).with_alpha(0.5));
    let mat_select = materials.add(Color::from(CYAN_500).with_alpha(0.8));
    let ref_x = bodies
        .to_x_plane_ref(body_id)
        .ok_or(color_eyre::eyre::eyre!("Should get X ref"))?;
    let ref_y = bodies
        .to_y_plane_ref(body_id)
        .ok_or(color_eyre::eyre::eyre!("Should get Y ref"))?;
    let ref_z = bodies
        .to_z_plane_ref(body_id)
        .ok_or(color_eyre::eyre::eyre!("Should get Z ref"))?;

    // normal vector will use for culling, this simple fix to avoid disappearing of planes
    for dir in [Dir3::Z, Dir3::NEG_Z] {
        let plane = meshes.add(Plane3d::default().mesh().size(10.0, 10.0).normal(dir));
        let mut entity = commands.spawn((
            Mesh3d(plane),
            MeshMaterial3d(mat_normal.clone()),
            Transform::from_xyz(0., 0., 0.),
            RenderLayers::layer(CAMERA_3D_LAYER),
            Visibility::Hidden,
            BodyBasePlane(ref_z),
            ObjectType::Plane(ref_z),
        ));
        entity.observe(update_plane_over(mat_over.clone()));
        entity.observe(update_plane_out(mat_normal.clone()));
        entity.observe(update_plane_selection(mat_select.clone()));
        entity.observe(update_plane_click);
        entities.push(entity.id());
    }

    for dir in [Dir3::X, Dir3::NEG_X] {
        let plane = meshes.add(Plane3d::default().mesh().size(10.0, 10.0).normal(dir));
        let mut entity = commands.spawn((
            Mesh3d(plane),
            MeshMaterial3d(mat_normal.clone()),
            Transform::from_xyz(0., 0., 0.),
            RenderLayers::layer(CAMERA_3D_LAYER),
            Visibility::Hidden,
            BodyBasePlane(ref_x),
            ObjectType::Plane(ref_x),
        ));
        entity.observe(update_plane_over(mat_over.clone()));
        entity.observe(update_plane_out(mat_normal.clone()));
        entity.observe(update_plane_selection(mat_select.clone()));
        entity.observe(update_plane_click);
        entities.push(entity.id());
    }

    for dir in [Dir3::Y, Dir3::NEG_Y] {
        let plane = meshes.add(Plane3d::default().mesh().size(10.0, 10.0).normal(dir));
        let mut entity = commands.spawn((
            Mesh3d(plane),
            MeshMaterial3d(mat_normal.clone()),
            Transform::from_xyz(0., 0., 0.),
            RenderLayers::layer(CAMERA_3D_LAYER),
            Visibility::Hidden,
            BodyBasePlane(ref_y),
            ObjectType::Plane(ref_y),
        ));
        entity.observe(update_plane_over(mat_over.clone()));
        entity.observe(update_plane_out(mat_normal.clone()));
        entity.observe(update_plane_selection(mat_select.clone()));
        entity.observe(update_plane_click);
        entities.push(entity.id());
    }

    Ok(entities)
}

pub(super) fn on_create_body(
    trigger: On<CreateBodyCommand>,
    mut engine: ResMut<EngineState>,
    mut app_state: ResMut<EngineAppState>,
    mut writer: MessageWriter<Notifications>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        // TODO should fallback notification when get error.
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

    if let Ok(entities) =
        register_body_base_planes(body, &body_id, &mut commands, &mut meshes, &mut materials)
    {
        app_state.body_planes_map.insert(body_id, entities);
    }

    transaction.commit();

    Ok(())
}

/// A command handler of [SwitchActiveBodyCommand]
pub(super) fn on_switch_active_body(
    trigger: On<SwitchActiveBodyCommand>,
    mut engine: ResMut<EngineState>,
    mut app_state: ResMut<EngineAppState>,
    mut writer: MessageWriter<Notifications>,
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
            BodyActivatedNotification {
                correlation_id: command.id.clone(),
                body_id: command.body_id.clone(),
            }
            .into(),
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

/// Update selections of something of body
pub(super) fn update_toggling_selection(
    mut reader: MessageReader<InternalSelectObject>,
    mut app_state: ResMut<EngineAppState>,
    query: Query<&ObjectType>,
) {
    for event in reader.read() {
        if let Some(p) = app_state
            .selections
            .iter()
            .position(|(e, _)| *e == *event.entity)
        {
            app_state.selections.remove(p);
        } else {
            let Ok(object_type) = query.get(*event.entity) else {
                tracing::warn!("Can not get object type from selectable entity");
                continue;
            };
            app_state
                .selections
                .push((*event.entity, object_type.clone()));
        }
    }
}
