use anyhow::{Result, anyhow};
use immutable::Im;

use crate::id::EdgeId;

/// Surface of the solid. Each face is some of a surface
#[derive(Clone, Debug)]
pub enum Surface {
    Planar(PlanarSurface),
}

/// A planar surface type
#[derive(Clone, Debug)]
pub struct PlanarSurface {
    /// The boundaries of the Surface
    pub boundaries: Im<Vec<EdgeId>>,
}

impl PlanarSurface {
    /// Get new planar surface
    pub fn new(boundaries: &[EdgeId]) -> Result<Self> {
        if boundaries.len() != 4 {
            return Err(anyhow!("Boundaries of planar must be 4"));
        }

        Ok(PlanarSurface {
            boundaries: Vec::from(boundaries).into(),
        })
    }
}

// simple factory
impl From<PlanarSurface> for Surface {
    fn from(planar: PlanarSurface) -> Self {
        Surface::Planar(planar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::id::IdStore;

    fn make_edge_ids(count: usize) -> Vec<EdgeId> {
        let mut store: IdStore<EdgeId> = IdStore::of();
        (0..count).map(|_| store.generate()).collect()
    }

    #[test]
    fn new_planar_surface_with_4_boundaries() {
        // Arrange
        let edges = make_edge_ids(4);

        // Act
        let surface = PlanarSurface::new(&edges);

        // Assert
        let surface = surface.unwrap();
        assert_eq!(surface.boundaries.len(), 4);
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    fn new_planar_surface_fails_with_non_4_boundaries(#[case] count: usize) {
        // Arrange
        let edges = make_edge_ids(count);

        // Act
        let result = PlanarSurface::new(&edges);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn new_planar_surface_fails_with_5_boundaries() {
        // Arrange
        let edges = make_edge_ids(5);

        // Act
        let result = PlanarSurface::new(&edges);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn new_planar_surface_preserves_edge_order() {
        // Arrange
        let edges = make_edge_ids(4);

        // Act
        let surface = PlanarSurface::new(&edges).unwrap();

        // Assert
        assert_eq!(*surface.boundaries, edges);
    }

    #[test]
    fn from_planar_surface_creates_surface_variant() {
        // Arrange
        let edges = make_edge_ids(4);
        let planar = PlanarSurface::new(&edges).unwrap();

        // Act
        let surface: Surface = planar.into();

        // Assert
        assert!(matches!(surface, Surface::Planar(_)));
    }
}
