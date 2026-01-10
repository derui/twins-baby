use bevy::prelude::*;

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
            "Top" => Some(TextureType::Top),
            "Bottom" => Some(TextureType::Bottom),
            "Left" => Some(TextureType::Left),
            "Right" => Some(TextureType::Right),
            "Front" => Some(TextureType::Front),
            "Back" => Some(TextureType::Back),
            _ => None,
        }
    }

    /// get texture path of the texture type
    pub fn texture_path(&self) -> String {
        let texture_path = match self {
            TextureType::Top => "textures/navigation-cube/top_ja.png",
            TextureType::Bottom => "textures/navigation-cube/bottom_ja.png",
            TextureType::Left => "textures/navigation-cube/left_ja.png",
            TextureType::Right => "textures/navigation-cube/right_ja.png",
            TextureType::Front => "textures/navigation-cube/front_ja.png",
            TextureType::Back => "textures/navigation-cube/back_ja.png",
        };

        texture_path.to_string()
    }
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
    use bevy::{mesh::PrimitiveTopology, pbr::UvChannel};

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

    #[test]
    fn test_texture_path() {
        assert_eq!(
            TextureType::Top.texture_path(),
            "textures/navigation-cube/top_ja.png"
        );
        assert_eq!(
            TextureType::Bottom.texture_path(),
            "textures/navigation-cube/bottom_ja.png"
        );
        assert_eq!(
            TextureType::Left.texture_path(),
            "textures/navigation-cube/left_ja.png"
        );
        assert_eq!(
            TextureType::Right.texture_path(),
            "textures/navigation-cube/right_ja.png"
        );
        assert_eq!(
            TextureType::Front.texture_path(),
            "textures/navigation-cube/front_ja.png"
        );
        assert_eq!(
            TextureType::Back.texture_path(),
            "textures/navigation-cube/back_ja.png"
        );
    }

    // ignore test
    // #[test]
    fn test_setup_navigation_texture_happy_path() {
        // Arrange
        let mut app = App::new();
        app.add_plugins((TaskPoolPlugin::default(), AssetPlugin::default()))
            .init_asset::<Image>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<Mesh>>();

        let material_handle = app
            .world_mut()
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial::default());
        let mesh_handle = app
            .world_mut()
            .resource_mut::<Assets<Mesh>>()
            .add(Mesh::new(
                PrimitiveTopology::TriangleList,
                Default::default(),
            ));

        app.world_mut().spawn((
            Name::new("Top".to_string()),
            MeshMaterial3d(material_handle.clone()),
            Mesh3d(mesh_handle.clone()),
            NeedsTextureSetup,
        ));

        // Act
        app.add_systems(Update, setup_navigation_texture);
        app.update();

        // Assert
        let materials = app.world().resource::<Assets<StandardMaterial>>();
        let material = materials.get(&material_handle).unwrap();
        assert!(material.base_color_texture.is_some());
        assert_eq!(material.base_color_channel, UvChannel::Uv0);

        let meshes = app.world().resource::<Assets<Mesh>>();
        let mesh = meshes.get(&mesh_handle).unwrap();
        assert!(mesh.attribute(Mesh::ATTRIBUTE_UV_0).is_some());
    }
}
