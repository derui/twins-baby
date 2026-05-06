//! internal module for navigation cube

use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::bevy_app::{
    camera::CAMERA_UI_LAYER,
    ui::components::{HudAnchor, NavigationCube, NeedsRenderLayers, NeedsTextureSetup},
};

const NAVIGATION_CUBE_SCALE: f32 = 4.8; // 4.8 to 4.8unit = 48px on UI

/// Texture type of the each mesh
enum TextureType {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

impl TextureType {
    /// Get Texture Type from mesh name. Mesh names are defined in gLTF
    pub fn from_mesh_name(s: &str) -> Option<Self> {
        match s {
            "Top" => Some(TextureType::Top),
            "Bottom" => Some(TextureType::Bottom),
            "Left" => Some(TextureType::Left),
            "Right" => Some(TextureType::Right),
            "Front" => Some(TextureType::Front),
            "Back" => Some(TextureType::Back),
            _ => None,
        }
    }
}

/// Setup the twins-baby UI elements
pub fn setup_navigation_cube(
    mut commands: Commands,
    asset: Res<AssetServer>,
) -> Result<(), BevyError> {
    // Navigation Cube
    let cube = asset.load(GltfAssetLabel::Scene(0).from_asset("navigation-cube.gltf"));

    commands.spawn((
        SceneRoot(cube),
        // current navigation cube model is located XY plane. so translate it a bit down to avoid z-fighting with grid.
        Transform::from_scale(Vec3::splat(NAVIGATION_CUBE_SCALE))
            .with_translation(Vec3::new(0.4, 0.4, 0.)),
        Visibility::Hidden,
        NavigationCube,
        NeedsRenderLayers(RenderLayers::layer(CAMERA_UI_LAYER)),
        HudAnchor::NavigationCube,
    ));

    Ok(())
}

/// Setup textures for navigation cube materials
pub fn setup_navigation_texture(
    mut commands: Commands,
    _materials: ResMut<Assets<StandardMaterial>>,
    _meshes: ResMut<Assets<Mesh>>,
    entities: Query<
        (Entity, &MeshMaterial3d<StandardMaterial>, &Name, &Mesh3d),
        With<NeedsTextureSetup>,
    >,
    _assets: Res<AssetServer>,
) -> Result<(), BevyError> {
    for (entity, _material, name, _mesh) in &entities {
        let Some(_texture) = TextureType::from_mesh_name(name) else {
            commands.entity(entity).remove::<NeedsTextureSetup>();
            continue;
        };

        commands.entity(entity).remove::<NeedsTextureSetup>();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_mesh_name_valid_faces() {
        // Test all valid face names
        assert!(matches!(
            TextureType::from_mesh_name("Top"),
            Some(TextureType::Top)
        ));
        assert!(matches!(
            TextureType::from_mesh_name("Bottom"),
            Some(TextureType::Bottom)
        ));
        assert!(matches!(
            TextureType::from_mesh_name("Left"),
            Some(TextureType::Left)
        ));
        assert!(matches!(
            TextureType::from_mesh_name("Right"),
            Some(TextureType::Right)
        ));
        assert!(matches!(
            TextureType::from_mesh_name("Front"),
            Some(TextureType::Front)
        ));
        assert!(matches!(
            TextureType::from_mesh_name("Back"),
            Some(TextureType::Back)
        ));
    }

    #[test]
    fn test_from_mesh_name_invalid_names() {
        assert!(TextureType::from_mesh_name("Invalid").is_none());
        assert!(TextureType::from_mesh_name("top").is_none());
        assert!(TextureType::from_mesh_name("TOP").is_none());
        assert!(TextureType::from_mesh_name("Face").is_none());
        assert!(TextureType::from_mesh_name("").is_none());
        assert!(TextureType::from_mesh_name("Middle").is_none());
        assert!(TextureType::from_mesh_name("Top ").is_none());
        assert!(TextureType::from_mesh_name(" Top").is_none());
    }

    #[test]
    fn test_from_mesh_name_case_sensitivity() {
        assert!(TextureType::from_mesh_name("face-top").is_none());
        assert!(TextureType::from_mesh_name("FACE-TOP").is_none());
        assert!(TextureType::from_mesh_name("bottom").is_none());
        assert!(TextureType::from_mesh_name("BOTTOM").is_none());
    }

    #[test]
    fn test_from_mesh_name_special_characters() {
        assert!(TextureType::from_mesh_name("Face_Top").is_none());
        assert!(TextureType::from_mesh_name("Face.Top").is_none());
        assert!(TextureType::from_mesh_name("-Top").is_none());
    }
}
