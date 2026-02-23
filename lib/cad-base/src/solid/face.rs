use color_eyre::eyre::{Result, eyre};
use immutable::Im;

use crate::{id::EdgeId, plane::Plane};

/// Surface of the solid. Each face is some of a surface
#[derive(Clone, Debug, PartialEq)]
pub enum Face {
    Planar(PlanarSurface),
}

/// A planar surface type
#[derive(Clone, Debug, PartialEq)]
pub struct PlanarSurface {
    /// The boundaries of the Surface
    pub boundaries: Im<Vec<EdgeId>>,

    /// The plane of the Surface
    pub plane: Im<Plane>,

    _immutable: (),
}

impl PlanarSurface {
    /// Get new planar surface
    pub fn new(boundaries: &[EdgeId], plane: &Plane) -> Result<Self> {
        if boundaries.len() < 3 {
            return Err(eyre!("Boundaries of planar must be greatee than 3"));
        }

        Ok(PlanarSurface {
            boundaries: Vec::from(boundaries).into(),
            plane: plane.clone().into(),
            _immutable: (),
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

    #[rstest]
    #[case(3)]
    #[case(4)]
    #[case(5)]
    fn new_planar_surface_succeeds_with_3_or_more_boundaries(#[case] count: usize) {
        // Arrange
        let edges = make_edge_ids(count);
        let plane = Plane::new_xy();

        // Act
        let result = PlanarSurface::new(&edges, &plane);

        // Assert
        let surface = result.expect("should create planar surface with 3 or more boundaries");
        assert_eq!(surface.boundaries.len(), count);
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    fn new_planar_surface_fails_with_fewer_than_3_boundaries(#[case] count: usize) {
        // Arrange
        let edges = make_edge_ids(count);
        let plane = Plane::new_xy();

        // Act
        let result = PlanarSurface::new(&edges, &plane);

        // Assert
        let _ = result.expect_err("should fail with fewer than 3 boundaries");
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
