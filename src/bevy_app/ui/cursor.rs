//! For cursor-related UI

use bevy::{prelude::*, window::PrimaryWindow};

use crate::bevy_app::{component::ui::CursorIconTag, resource::AppCursorIcon};

pub(super) const ICON_SIZE: f32 = 24.0;

/// Setup the cursor icon UI element. It will be updated by `update_cursor_icon` system.
pub fn setup_cursor_icon(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: px(ICON_SIZE),
            height: px(ICON_SIZE),
            ..default()
        },
        // Hide initially
        Visibility::Hidden,
        CursorIconTag,
    ));
}

/// Update the cursor icon UI element. It will be shown at the cursor position when `AppCursorIcon` resource is set.
pub fn update_cursor_icon(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    icon: Single<(Entity, &mut Node, &mut Visibility), With<CursorIconTag>>,
    assert_server: Res<AssetServer>,
    current_icon: Res<AppCursorIcon>,
) {
    let (e, mut node, mut vis) = icon.into_inner();

    let asset = current_icon.0.map(|f| f.to_asset_url());

    match (window.cursor_position(), asset) {
        (Some(pos), Some(url)) => {
            // insert
            commands.entity(e).remove::<ImageNode>();
            commands
                .entity(e)
                .insert(ImageNode::new(assert_server.load(url)));

            node.left = px(pos.x + ICON_SIZE);
            node.top = px(pos.y + ICON_SIZE);
            *vis = Visibility::Visible;
        }
        _ => *vis = Visibility::Hidden,
    }
}

#[cfg(test)]
mod tests {
    use bevy::{
        asset::AssetPlugin, ecs::system::RunSystemOnce, math::DVec2, prelude::*,
        window::PrimaryWindow,
    };
    use pretty_assertions::assert_eq;

    use crate::bevy_app::{
        component::ui::CursorIconTag,
        resource::{AppCursorIcon, IconType},
    };

    use super::*;

    fn setup_cursor_icon_world() -> (World, Entity) {
        let mut world = World::new();
        world.run_system_once(setup_cursor_icon).unwrap();
        let entity = world
            .query::<(Entity, &CursorIconTag)>()
            .single(&world)
            .unwrap()
            .0;
        (world, entity)
    }

    fn make_app() -> (App, Entity, Entity) {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<Image>();
        app.insert_resource(AppCursorIcon(None));
        let window_entity = app
            .world_mut()
            .spawn((Window::default(), PrimaryWindow))
            .id();
        let icon_entity = app
            .world_mut()
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(ICON_SIZE),
                    height: Val::Px(ICON_SIZE),
                    ..default()
                },
                Visibility::Hidden,
                CursorIconTag,
            ))
            .id();
        (app, window_entity, icon_entity)
    }

    #[test]
    fn setup_cursor_icon_spawns_entity_with_cursor_icon_tag() {
        // Arrange + Act
        let (mut world, _) = setup_cursor_icon_world();

        // Assert
        let count = world.query::<&CursorIconTag>().iter(&world).count();
        assert_eq!(count, 1);
    }

    #[test]
    fn setup_cursor_icon_spawns_entity_hidden() {
        // Arrange + Act
        let (world, entity) = setup_cursor_icon_world();

        // Assert
        let vis = world.get::<Visibility>(entity).unwrap();
        assert_eq!(*vis, Visibility::Hidden);
    }

    #[test]
    fn setup_cursor_icon_spawns_entity_with_absolute_position() {
        // Arrange + Act
        let (world, entity) = setup_cursor_icon_world();

        // Assert
        let node = world.get::<Node>(entity).unwrap();
        assert_eq!(node.position_type, PositionType::Absolute);
        assert_eq!(node.width, Val::Px(ICON_SIZE));
        assert_eq!(node.height, Val::Px(ICON_SIZE));
    }

    #[test]
    fn update_cursor_icon_hides_when_no_cursor_and_no_icon() {
        // Arrange
        let (mut app, _, icon_entity) = make_app();
        // Default Window has no cursor position; AppCursorIcon is None

        // Act
        app.world_mut().run_system_once(update_cursor_icon).unwrap();

        // Assert
        let vis = app.world().get::<Visibility>(icon_entity).unwrap();
        assert_eq!(*vis, Visibility::Hidden);
    }

    #[test]
    fn update_cursor_icon_hides_when_cursor_exists_but_no_icon() {
        // Arrange
        let (mut app, window_entity, icon_entity) = make_app();
        app.world_mut()
            .get_mut::<Window>(window_entity)
            .unwrap()
            .set_physical_cursor_position(Some(DVec2::new(100.0, 100.0)));
        // AppCursorIcon remains None

        // Act
        app.world_mut().run_system_once(update_cursor_icon).unwrap();

        // Assert
        let vis = app.world().get::<Visibility>(icon_entity).unwrap();
        assert_eq!(*vis, Visibility::Hidden);
    }

    #[test]
    fn update_cursor_icon_hides_when_no_cursor_but_icon_set() {
        // Arrange
        let (mut app, _, icon_entity) = make_app();
        app.insert_resource(AppCursorIcon(Some(IconType::SketchLine)));
        // Default Window has no cursor position

        // Act
        app.world_mut().run_system_once(update_cursor_icon).unwrap();

        // Assert
        let vis = app.world().get::<Visibility>(icon_entity).unwrap();
        assert_eq!(*vis, Visibility::Hidden);
    }

    #[test]
    fn update_cursor_icon_shows_and_positions_when_cursor_and_icon() {
        // Arrange
        let (mut app, window_entity, icon_entity) = make_app();
        let cursor_x = 100.0_f64;
        let cursor_y = 200.0_f64;
        app.world_mut()
            .get_mut::<Window>(window_entity)
            .unwrap()
            .set_physical_cursor_position(Some(DVec2::new(cursor_x, cursor_y)));
        app.insert_resource(AppCursorIcon(Some(IconType::SketchLine)));

        // Act
        app.world_mut().run_system_once(update_cursor_icon).unwrap();

        // Assert
        let vis = app.world().get::<Visibility>(icon_entity).unwrap();
        assert_eq!(*vis, Visibility::Visible);
        let node = app.world().get::<Node>(icon_entity).unwrap();
        assert_eq!(node.left, Val::Px(cursor_x as f32 + ICON_SIZE));
        assert_eq!(node.top, Val::Px(cursor_y as f32 + ICON_SIZE));
    }
}
