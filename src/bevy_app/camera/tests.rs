use bevy::ecs::{system::RunSystemOnce, world::World};
use pretty_assertions::assert_eq;

use super::*;
use approx::assert_relative_eq;

fn make_world() -> World {
    let mut world = World::new();
    world.init_resource::<Time>();
    world
}

#[test]
fn system_noop_does_not_move_camera() {
    // Arrange
    let mut world = make_world();
    let initial = Transform::from_xyz(0.0, 0.0, 5.0);
    world.spawn((CameraMoveHandle::default(), CameraMoveOperation::Noop));
    let cam = world.spawn((MainCamera, initial)).id();
    world.spawn(HudRotation::default());

    // Act
    world.run_system_once(move_camera_with_request).unwrap();

    // Assert
    let transform = *world.get::<Transform>(cam).unwrap();
    assert_eq!(transform, initial);
}

#[test]
fn system_by_orbit_moves_camera_by_radius() {
    // Arrange
    let mut world = make_world();
    let state = PanOrbitOperation {
        center: Vec3::ZERO,
        radius: 5.0,
        ..Default::default()
    };
    world.spawn((
        CameraMoveHandle::default(),
        CameraMoveOperation::ByOrbit(state),
    ));
    let cam = world.spawn((MainCamera, Transform::default())).id();
    world.spawn(HudRotation::default());

    // Act
    world.run_system_once(move_camera_with_request).unwrap();

    // Assert - camera translation from center equals radius
    let transform = world.get::<Transform>(cam).unwrap();
    assert_relative_eq!(transform.translation.length(), 5.0, epsilon = 1e-5);
}

#[test]
fn system_by_orbit_resets_operation_to_noop_after_completion() {
    // Arrange
    let mut world = make_world();
    let state = PanOrbitOperation {
        center: Vec3::ZERO,
        radius: 3.0,
        ..Default::default()
    };
    let req = world
        .spawn((
            CameraMoveHandle::default(),
            CameraMoveOperation::ByOrbit(state),
        ))
        .id();
    world.spawn((MainCamera, Transform::default()));

    // Act
    world.run_system_once(move_camera_with_request).unwrap();

    // Assert - immediate completion resets to Noop
    let op = world.get::<CameraMoveOperation>(req).unwrap();
    assert_eq!(*op, CameraMoveOperation::Noop);
}

#[test]
fn system_by_orbit_updates_hud_rotation() {
    // Arrange
    let mut world = make_world();
    let state = PanOrbitOperation {
        center: Vec3::ZERO,
        radius: 5.0,
        yaw: std::f32::consts::FRAC_PI_4,
        ..Default::default()
    };
    world.spawn((
        CameraMoveHandle::default(),
        CameraMoveOperation::ByOrbit(state),
    ));
    world.spawn((MainCamera, Transform::default()));
    let hud = world.spawn(HudRotation::default()).id();

    // Act
    world.run_system_once(move_camera_with_request).unwrap();

    // Assert - HudRotation updated to non-identity
    let rotation = **world.get::<HudRotation>(hud).unwrap();
    assert!(rotation != Quat::IDENTITY);
}

#[test]
fn noop_returns_no_expectation() {
    // Arrange
    let op = CameraMoveOperation::Noop;
    let transform = Transform::default();

    // Act
    let result = op.calculate_expectation(&transform);

    // Assert
    assert!(result.is_none());
}

#[test]
fn by_orbit_returns_expectation() {
    // Arrange
    let state = PanOrbitOperation {
        center: Vec3::ZERO,
        radius: 5.0,
        ..Default::default()
    };
    let op = CameraMoveOperation::ByOrbit(state);
    let transform = Transform::default();

    // Act
    let result = op.calculate_expectation(&transform);

    // Assert
    assert!(result.is_some());
}

#[test]
fn system_by_system_places_camera_at_position() {
    // Arrange
    let mut world = make_world();
    let camera_pos = Vec3::new(0.0, 5.0, 0.0);
    world.spawn((
        CameraMoveHandle::default(),
        CameraMoveOperation::BySystem {
            target: Vec3::new(0.0, 0.0, 5.0),
            position: camera_pos,
            pitch: None,
            yaw: None,
            duration: 0.0,
        },
    ));
    let cam = world.spawn((MainCamera, Transform::default())).id();
    world.spawn(HudRotation::default());

    // Act
    world.run_system_once(move_camera_with_request).unwrap();

    // Assert
    let transform = world.get::<Transform>(cam).unwrap();
    assert_relative_eq!(transform.translation.x, camera_pos.x, epsilon = 1e-5);
    assert_relative_eq!(transform.translation.y, camera_pos.y, epsilon = 1e-5);
    assert_relative_eq!(transform.translation.z, camera_pos.z, epsilon = 1e-5);
}

#[test]
fn system_by_system_resets_to_noop_after_completion() {
    // Arrange
    let mut world = make_world();
    let req = world
        .spawn((
            CameraMoveHandle::default(),
            CameraMoveOperation::BySystem {
                target: Vec3::new(0.0, 0.0, 5.0),
                position: Vec3::new(0.0, 5.0, 0.0),
                pitch: None,
                yaw: None,
                duration: 0.0,
            },
        ))
        .id();
    world.spawn((MainCamera, Transform::default()));

    // Act
    world.run_system_once(move_camera_with_request).unwrap();

    // Assert
    let op = world.get::<CameraMoveOperation>(req).unwrap();
    assert_eq!(*op, CameraMoveOperation::Noop);
}

#[test]
fn system_by_system_does_not_reset_during_animation() {
    // Arrange
    let mut world = make_world();
    let req = world
        .spawn((
            CameraMoveHandle::default(),
            CameraMoveOperation::BySystem {
                target: Vec3::new(0.0, 0.0, 5.0),
                position: Vec3::new(0.0, 5.0, 0.0),
                pitch: None,
                yaw: None,
                duration: 1.0,
            },
        ))
        .id();
    world.spawn((MainCamera, Transform::default()));

    // Act
    world.run_system_once(move_camera_with_request).unwrap();

    // Assert - operation still in progress, not yet reset to Noop
    let op = world.get::<CameraMoveOperation>(req).unwrap();
    assert_ne!(*op, CameraMoveOperation::Noop);
}

#[test]
fn ease_in_out_cubic_at_boundaries() {
    // Arrange & Act & Assert
    assert_relative_eq!(ease_in_out_cubic(0.0), 0.0);
    assert_relative_eq!(ease_in_out_cubic(1.0), 1.0);
    assert_relative_eq!(ease_in_out_cubic(0.5), 0.5);
}

#[test]
fn ease_in_out_cubic_is_monotonic() {
    // Arrange
    let samples: Vec<f32> = (0..=10).map(|i| i as f32 / 10.0).collect();

    // Act
    let values: Vec<f32> = samples.iter().map(|&t| ease_in_out_cubic(t)).collect();

    // Assert
    for window in values.windows(2) {
        assert!(window[0] <= window[1], "{} > {}", window[0], window[1]);
    }
}
