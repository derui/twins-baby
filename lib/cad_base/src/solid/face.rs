use anyhow::{Result, anyhow};
use immutable::Im;

use crate::{id::EdgeId, plane::Plane};

/// Surface of the solid. Each face is some of a surface
#[derive(Clone, Debug)]
pub enum Face {
    Planar(PlanarSurface),
}

/// A planar surface type
#[derive(Clone, Debug)]
pub struct PlanarSurface {
    /// The boundaries of the Surface
    pub boundaries: Im<Vec<EdgeId>>,

    /// The plane of the Surface
    pub plane: Im<Plane>,
}

impl PlanarSurface {
    /// Get new planar surface
    pub fn new(boundaries: &[EdgeId], plane: &Plane) -> Result<Self> {
        if boundaries.len() != 4 {
            return Err(anyhow!("Boundaries of planar must be 4"));
        }

        Ok(PlanarSurface {
            boundaries: Vec::from(boundaries).into(),
            plane: plane.clone().into()
        })
    }
}

// simple factory
impl From<PlanarSurface> for Face {
    fn from(planar: PlanarSurface) -> Self {
        Face::Planar(planar)
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
        let plane = Plane::new_xy();

        // Act
        let surface = PlanarSurface::new(&edges, &plane);

        // Assert
        let surface = surface.expect("should create planar surface with 4 boundaries");
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
        let plane = Plane::new_xy();

        // Act
        let result = PlanarSurface::new(&edges, &plane);

        // Assert
        result.expect_err("should fail with non-4 boundaries");
    }

    #[test]
    fn new_planar_surface_fails_with_5_boundaries() {
        // Arrange
        let edges = make_edge_ids(5);
        let plane = Plane::new_xy();

        // Act
        let result = PlanarSurface::new(&edges, &plane);

        // Assert
        result.expect_err("should fail with 5 boundaries");
    }

    #[test]
    fn new_planar_surface_preserves_edge_order() {
        // Arrange
        let edges = make_edge_ids(4);
        let plane = Plane::new_xy();

        // Act
        let surface = PlanarSurface::new(&edges, &plane).expect("should create planar surface");

        // Assert
        assert_eq!(*surface.boundaries, edges);
    }

    #[test]
    fn from_planar_surface_creates_surface_variant() {
        // Arrange
        let edges = make_edge_ids(4);
        let plane = Plane::new_xy();
        let planar = PlanarSurface::new(&edges, &plane).expect("should create planar surface");

        // Act
        let surface: Face = planar.into();

        // Assert
        assert!(matches!(surface, Face::Planar(_)));
    }
}
