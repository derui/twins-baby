#[cfg(test)]
mod tests;

use std::collections::HashMap;

use anyhow::Result;

use crate::{
    edge::Edge,
    id::{DefaultIdGenerator, EdgeId, GenerateId, PlaneId, PointId, SketchId},
    point::Point,
};

/// The sketch of base of modeling.
///
/// [Sketch] has these values:
///
/// - point and edges in the sketchs
/// - variables for all points in this sketch
/// - attached Plane with plane id.
/// - constraints equations for points (not implemented yet)
///
/// All points and edges as unique in the sketch.
#[derive(Debug, Clone)]
pub struct Sketch {
    id: SketchId,
    edge_id_gen: Box<dyn GenerateId<EdgeId>>,
    point_id_gen: Box<dyn GenerateId<PointId>>,

    /// Points in this sketch
    points: HashMap<PointId, Point>,

    /// Edges in this sketch
    edges: HashMap<EdgeId, Edge>,

    /// A plane atteched to sketch
    attached_plane: PlaneId,
}

/// The builder structure of the sketch
#[derive(Debug)]
pub struct SketchBuilder {
    pub edge_id_gen: Box<dyn GenerateId<EdgeId>>,
    pub point_id_gen: Box<dyn GenerateId<PointId>>,
    pub attached_plane: Option<PlaneId>,
}

impl Default for SketchBuilder {
    fn default() -> Self {
        Self {
            edge_id_gen: Box::new(DefaultIdGenerator::default()),
            point_id_gen: Box::new(DefaultIdGenerator::default()),
            attached_plane: None,
        }
    }
}

impl Sketch {
    /// Create a new sketch with builder
    ///
    /// ```rust
    /// Sketch::new(SketchId::new(1), SketchBuilder {
    ///   attached_plane: Some(PlaneId(3)),
    ///   ..Default::default()
    /// })
    /// ```
    pub fn new(id: SketchId, builder: SketchBuilder) -> Result<Self> {
        let Some(attached_plane) = builder.attached_plane else {
            return Err(anyhow::anyhow!("Must set attached_plane"));
        };

        Ok(Sketch {
            id,
            edge_id_gen: builder.edge_id_gen,
            point_id_gen: builder.point_id_gen,
            points: HashMap::new(),
            edges: HashMap::new(),
            attached_plane,
        })
    }
}
