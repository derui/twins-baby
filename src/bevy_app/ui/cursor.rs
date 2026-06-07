use bevy::{prelude::*, window::PrimaryWindow};

use crate::bevy_app::{component::ui::CursorIconTag, resource::AppCursorIcon};

///! For cursor-related UI

const ICON_SIZE: f32 = 24.0;

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
    mut icon: Single<(Entity, &mut Node, &mut Visibility), With<CursorIconTag>>,
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
