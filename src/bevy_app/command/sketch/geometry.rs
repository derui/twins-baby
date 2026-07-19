// Mouse handler for sketch commands.
use bevy::{prelude::*, window::PrimaryWindow};
use cad_base::sketch::{Geometry, LineSegment, Point2, SketchPerspective};
use ui_event::SketchGeometryOperation;

use crate::bevy_app::{
    camera::MainCamera,
    component::{
        RequestedGeometryOperation,
        sketch::{GeometryOperation, StepResult},
    },
    resource::{AppActiveSketch, AppCursorIcon, EngineState},
    support::Vec3Ext,
};

/// The event to notify that a geometry operation is completed.
#[derive(Debug, Clone, Event)]
pub struct GeometryOperationCompletedEvent {
    /// A opelation that completed
    pub operation: SketchGeometryOperation,
    /// All points to create geometry
    pub points: Vec<Vec3>,
}

/// The systemt that handle mouse events while geometry creation operation.
///
/// this handles:
/// - convert click point in the window to the point on the attachable target
/// - Step forward the operation.
/// - finalize operation if it completed.
pub fn handle_geometry_operation(
    mouse: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut processing: Query<(
        Entity,
        &mut RequestedGeometryOperation,
        &mut GeometryOperation,
    )>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let just_activated = mouse.just_pressed(MouseButton::Left);

    // handle only the button just pressed
    if !just_activated {
        return;
    }

    let Ok((camera, global_transform)) = q_camera.single() else {
        return;
    };
    // get global intersection position between sketch target and cursor
    let Some(cursor_position) = q_window.single().expect("Should get").cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(global_transform, cursor_position) else {
        return;
    };
    let Ok((e, ope, mut geo)) = processing.single_mut() else {
        return;
    };

    if let Some(point) = ray.plane_intersection_point(
        geo.plane.r0.to_vec3(),
        InfinitePlane3d::new(geo.plane.normal.to_vec3()),
    ) {
        // convert 2D.
        let point = point - geo.plane.r0.to_vec3();

        if let StepResult::Completed = geo.forward_step(point) {
            // after operation finished, send event.
            commands.entity(e).despawn();

            commands.trigger(GeometryOperationCompletedEvent {
                operation: ope.0.clone(),
                points: geo.step_result().clone(),
            });
        }
    }
}

/**
 * Handle [GeometryOperationCompletedEvent] after.
 *
 * This handler will create geometry to current sketch
 */
pub fn on_geometory_operation_completed(
    trigger: On<GeometryOperationCompletedEvent>,
    mut engine: ResMut<EngineState>,
    active_sketch: Res<AppActiveSketch>,
    mut cursor: ResMut<AppCursorIcon>,
) {
    let event = trigger.event();
    let mut t = engine.0.begin();

    let Some(sketch) = t.modify::<SketchPerspective>() else {
        return;
    };

    let Some(active_sketch) = active_sketch.0.and_then(|s| sketch.get_mut(&s)) else {
        return;
    };

    match event.operation {
        SketchGeometryOperation::LineSegment => {
            let [start, end] = event.points.as_slice() else {
                panic!("line segment operation should contain exactly two points");
            };

            let start = Point2::new(start.x, start.y);
            let end = Point2::new(end.x, end.y);
            active_sketch.add_geometry(|scope| {
                Geometry::LineSegment(LineSegment::from_points(&start, &end, scope))
            });
        }
        SketchGeometryOperation::Rectangle => todo!("rectangle completion is not implemented yet"),
    }

    cursor.0 = None;

    t.commit();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::input::ButtonInput;
    use bevy::math::DVec2;
    use bevy::prelude::{Entity, MouseButton, Schedule, With, World};
    use bevy::window::{PrimaryWindow, Window};
    use cad_base::{
        body::BodyPerspective,
        plane::Plane,
        sketch::{AttachableTarget, Point2},
    };
    use eyre::Result;
    use pretty_assertions::assert_eq;
    use ui_event::SketchGeometryOperation;

    use crate::bevy_app::test_support::WindowOp as _;
    use crate::bevy_app::{
        component::{RequestedGeometryOperation, sketch::GeometryOperation},
        resource::IconType,
        test_support::TestEnv as _,
    };

    fn default_plane() -> Plane {
        Plane::new_xy()
    }

    fn make_world() -> World {
        let mut world = World::new();
        world.init_resource::<ButtonInput<MouseButton>>();
        world
    }

    fn make_app() -> App {
        let mut app = App::new();
        app.setup_test_env();
        app
    }

    fn make_geometry_completed_app() -> App {
        let mut app = App::new();
        app.setup_test_env()
            .add_observer(on_geometory_operation_completed);
        app
    }

    fn create_sketch(world: &mut World) -> cad_base::id::SketchId {
        let mut engine = world.resource_mut::<EngineState>();
        let mut tx = engine.0.begin();
        let bodies = tx.modify::<BodyPerspective>().unwrap();
        let body_id = bodies.add_body();
        let plane_ref = bodies.to_x_plane_ref(&body_id).unwrap();
        let sketch_id = tx
            .modify::<SketchPerspective>()
            .unwrap()
            .add_sketch(body_id, &AttachableTarget::Plane(plane_ref));
        tx.commit();
        sketch_id
    }

    #[test]
    fn system_does_not_despawn_entity_when_mouse_not_pressed() -> Result<()> {
        // Arrange
        let mut world = make_world();
        let plane = default_plane();
        let entity = world
            .spawn((
                RequestedGeometryOperation(SketchGeometryOperation::LineSegment),
                GeometryOperation::from_geometry(SketchGeometryOperation::LineSegment, &plane),
            ))
            .id();

        // Act
        let mut schedule = Schedule::default();
        schedule.add_systems(handle_geometry_operation);
        schedule.run(&mut world);

        // Assert - entity must still exist because mouse was not pressed
        assert!(world.get_entity(entity).is_ok());
        Ok(())
    }

    #[test]
    fn system_does_not_despawn_entity_when_no_camera_exists() -> Result<()> {
        // Arrange
        let mut app = make_app();
        let plane = default_plane();
        let entity = app
            .world_mut()
            .spawn((
                RequestedGeometryOperation(SketchGeometryOperation::LineSegment),
                GeometryOperation::from_geometry(SketchGeometryOperation::LineSegment, &plane),
            ))
            .id();
        let camera_entity = app
            .world_mut()
            .query_filtered::<Entity, With<MainCamera>>()
            .single(app.world())
            .unwrap();
        app.world_mut().entity_mut(camera_entity).despawn();

        // Act
        app.click_at(DVec2::new(400.0, 300.0), |w| {
            w.run_system_once(handle_geometry_operation).unwrap()
        });

        // Assert - entity still exists because no camera was found
        assert!(app.world().get_entity(entity).is_ok());
        Ok(())
    }

    #[test]
    fn completion_adds_line_segment_to_active_sketch() -> Result<()> {
        // Arrange
        let mut app = make_geometry_completed_app();
        let world = app.world_mut();
        let sketch_id = create_sketch(world);
        world.resource_mut::<AppActiveSketch>().0 = Some(sketch_id);
        world.resource_mut::<AppCursorIcon>().0 = Some(IconType::SketchLine);

        // Act
        world.trigger(GeometryOperationCompletedEvent {
            operation: SketchGeometryOperation::LineSegment,
            points: vec![Vec3::new(1.0, 2.0, 0.0), Vec3::new(4.0, 5.0, 0.0)],
        });
        world.flush();

        // Assert
        {
            let mut engine = world.resource_mut::<EngineState>();
            let tx = engine.0.begin();
            let sketch = tx
                .read::<SketchPerspective>()
                .unwrap()
                .get(&sketch_id)
                .unwrap();
            let edges = sketch.resolve_edges()?;
            assert_eq!(edges.len(), 1);
            assert_eq!((*edges[0].start).clone(), Point2::new(1.0, 2.0));
            assert_eq!((*edges[0].end).clone(), Point2::new(4.0, 5.0));
        }
        assert_eq!(world.resource::<AppCursorIcon>().0, None);
        Ok(())
    }

    #[test]
    fn completion_does_nothing_when_no_active_sketch_exists() -> Result<()> {
        // Arrange
        let mut app = make_geometry_completed_app();
        let world = app.world_mut();
        let sketch_id = create_sketch(world);

        // Act
        world.trigger(GeometryOperationCompletedEvent {
            operation: SketchGeometryOperation::LineSegment,
            points: vec![Vec3::new(1.0, 2.0, 0.0), Vec3::new(4.0, 5.0, 0.0)],
        });
        world.flush();

        // Assert
        let mut engine = world.resource_mut::<EngineState>();
        let tx = engine.0.begin();
        let sketch = tx
            .read::<SketchPerspective>()
            .unwrap()
            .get(&sketch_id)
            .unwrap();
        assert_eq!(sketch.resolve_edges()?.len(), 0);
        Ok(())
    }
}
