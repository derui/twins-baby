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
