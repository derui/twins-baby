use bevy::{log::tracing::span::Attributes, pbr::UvChannel, prelude::*};

use crate::bevy_app::ui::components::NeedsTextureSetup;

/// internal module for navigation cube

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
            "Face-Top" => Some(TextureType::Top),
            "Face-Bottom" => Some(TextureType::Bottom),
            "Face-Left" => Some(TextureType::Left),
            "Face-Right" => Some(TextureType::Right),
            "Face-Front" => Some(TextureType::Front),
            "Face-Back" => Some(TextureType::Back),
            _ => None,
        }
    }

    /// get texture path of the texture type
    pub fn texture_path(&self) -> String {
        let texture_path = match self {
            TextureType::Top => "textures/navigation_cube/top_ja.png",
            TextureType::Bottom => "textures/navigation_cube/bottom_ja.png",
            TextureType::Left => "textures/navigation_cube/left_ja.png",
            TextureType::Right => "textures/navigation_cube/right_ja.png",
            TextureType::Front => "textures/navigation_cube/front_ja.png",
            TextureType::Back => "textures/navigation_cube/back_ja.png",
        };

        texture_path.to_string()
    }
}

/// Setup textures for navigation cube materials
pub fn setup_navigation_texture(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    entities: Query<
        (Entity, &MeshMaterial3d<StandardMaterial>, &Name, &Mesh3d),
        With<NeedsTextureSetup>,
    >,
    assets: Res<AssetServer>,
) -> Result<(), BevyError> {
    for (entity, material, name, mesh) in &entities {
        let Some(texture) = TextureType::from_mesh_name(&name) else {
            commands.entity(entity).remove::<NeedsTextureSetup>();
            continue;
        };

        // Load image for the texture
        let texture: Handle<Image> = assets.load(&texture.texture_path());

        // set up material and mesh
        if let Some(material) = materials.get_mut(material) {
            material.base_color_texture = Some(texture);
            material.base_color_channel = UvChannel::Uv0;
        }

        if let Some(mesh) = meshes.get_mut(mesh) {
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_UV_0,
                vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            )
        }

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
            TextureType::from_mesh_name("Face-Top"),
            Some(TextureType::Top)
        ));
        assert!(matches!(
            TextureType::from_mesh_name("Face-Bottom"),
            Some(TextureType::Bottom)
        ));
        assert!(matches!(
            TextureType::from_mesh_name("Face-Left"),
            Some(TextureType::Left)
        ));
        assert!(matches!(
            TextureType::from_mesh_name("Face-Right"),
            Some(TextureType::Right)
        ));
        assert!(matches!(
            TextureType::from_mesh_name("Face-Front"),
            Some(TextureType::Front)
        ));
        assert!(matches!(
            TextureType::from_mesh_name("Face-Back"),
            Some(TextureType::Back)
        ));
    }

    #[test]
    fn test_from_mesh_name_invalid_names() {
        assert!(TextureType::from_mesh_name("Invalid").is_none());
        assert!(TextureType::from_mesh_name("Face-top").is_none());
        assert!(TextureType::from_mesh_name("Face-TOP").is_none());
        assert!(TextureType::from_mesh_name("Face").is_none());
        assert!(TextureType::from_mesh_name("Top").is_none());
        assert!(TextureType::from_mesh_name("").is_none());
        assert!(TextureType::from_mesh_name("Face-Middle").is_none());
        assert!(TextureType::from_mesh_name("Face-Top ").is_none());
        assert!(TextureType::from_mesh_name(" Face-Top").is_none());
    }

    #[test]
    fn test_from_mesh_name_case_sensitivity() {
        assert!(TextureType::from_mesh_name("face-top").is_none());
        assert!(TextureType::from_mesh_name("FACE-TOP").is_none());
        assert!(TextureType::from_mesh_name("Face-bottom").is_none());
        assert!(TextureType::from_mesh_name("Face-BOTTOM").is_none());
    }

    #[test]
    fn test_from_mesh_name_special_characters() {
        assert!(TextureType::from_mesh_name("Face_Top").is_none());
        assert!(TextureType::from_mesh_name("Face.Top").is_none());
        assert!(TextureType::from_mesh_name("Face--Top").is_none());
    }

    #[test]
    fn test_texture_path() {
        assert_eq!(
            TextureType::Top.texture_path(),
            "textures/navigation_cube/top_ja.png"
        );
        assert_eq!(
            TextureType::Bottom.texture_path(),
            "textures/navigation_cube/bottom_ja.png"
        );
        assert_eq!(
            TextureType::Left.texture_path(),
            "textures/navigation_cube/left_ja.png"
        );
        assert_eq!(
            TextureType::Right.texture_path(),
            "textures/navigation_cube/right_ja.png"
        );
        assert_eq!(
            TextureType::Front.texture_path(),
            "textures/navigation_cube/front_ja.png"
        );
        assert_eq!(
            TextureType::Back.texture_path(),
            "textures/navigation_cube/back_ja.png"
        );
    }
}
